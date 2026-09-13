#!/usr/bin/env python3
# /// script
# dependencies = [
#     "argon2-cffi>=23.1.0",
#     "psycopg[binary]>=3.2.0",
#     "requests>=2.32.0",
#     "redis>=5.0.0",
# ]
# ///
"""
CSAC Timetable Studio - Automated Database Cleaner & API Gateway Seeding Pipeline
1. Direct SQL Truncate & Database Wipe
2. Temporary "seed" Account Creation with Argon2id
3. API Gateway Authentication (obtain admin JWT)
4. Domain Seeding via API Gateway REST Endpoints:
   - 1 Active Show & Time Slots
   - 40 Real Members (5 Admins, 5 Mods, 30 Members; 1 DM, 10 PMs, 10 QCs) with defined passwords (default 'password123')
   - 12 Music Gear items (Keyboards, Guitars, Drums, Audio)
   - 20 Authentic Music Numbers with Lineups & QC Audits
   - Practice Sprint 1 & Member 15m Free-Time Registration
5. Secure Deallocation of Temporary "seed" Account
"""

import json
import os
import sys
import time
from pathlib import Path
from typing import Dict, Any, List

import psycopg
import requests
import redis
from argon2 import PasswordHasher

# Configuration
POSTGRES_HOST = os.getenv("POSTGRES_HOST", "127.0.0.1")
POSTGRES_PORT = int(os.getenv("POSTGRES_PORT", "5432"))
POSTGRES_DB = os.getenv("POSTGRES_DB", "csac_timetable")
POSTGRES_USER = os.getenv("POSTGRES_USER", "csac_admin")
POSTGRES_PASSWORD = os.getenv("POSTGRES_PASSWORD", "csac_password")

REDIS_HOST = os.getenv("REDIS_HOST", "127.0.0.1")
REDIS_PORT = int(os.getenv("REDIS_PORT", "6379"))

GATEWAY_URL = os.getenv("GATEWAY_URL", "http://127.0.0.1:8080")

DB_CONN_STR = f"postgresql://{POSTGRES_USER}:{POSTGRES_PASSWORD}@{POSTGRES_HOST}:{POSTGRES_PORT}/{POSTGRES_DB}"
DATA_FILE = Path(__file__).parent / "data" / "mock_seed.json"

ph = PasswordHasher(
    time_cost=3,
    memory_cost=65536,
    parallelism=4,
    hash_len=32,
    salt_len=16
)


def log_step(step: str, detail: str = ""):
    print(f"🎵 [{time.strftime('%H:%M:%S')}] {step}")
    if detail:
        print(f"   ↳ {detail}")


def wipe_database():
    log_step("STEP 1: Cleaning & Truncating Database...", "Connecting to PostgreSQL")
    with psycopg.connect(DB_CONN_STR, autocommit=True) as conn:
        with conn.cursor() as cur:
            tables = [
                "votes",
                "event_time_slots",
                "practice_tasks",
                "instrument_reservations",
                "member_sprint_availabilities",
                "member_sprint_availabilities_history",
                "sprint_schedule_runs",
                "music_number_members",
                "music_numbers",
                "practice_sprints",
                "instruments",
                "admin_downgrade_votes",
                "admin_downgrade_proposals",
                "audit_logs",
                "events",
                "users",
            ]
            for tbl in tables:
                cur.execute(f"TRUNCATE TABLE {tbl} CASCADE;")
            print("   ✅ All PostgreSQL tables truncated successfully.")

    # Flush Redis cache
    try:
        r = redis.Redis(host=REDIS_HOST, port=REDIS_PORT, db=0)
        r.flushall()
        print("   ✅ Redis cache completely flushed.")
    except Exception as e:
        print(f"   ⚠️ Redis flush warning: {e}")


def create_temp_seed_account():
    log_step("STEP 2: Creating Temporary 'seed' Account...", "Hashing password with Argon2id")
    password_hash = ph.hash("seed")
    with psycopg.connect(DB_CONN_STR, autocommit=True) as conn:
        with conn.cursor() as cur:
            cur.execute(
                """
                INSERT INTO users (email, full_name, password_hash, role, status)
                VALUES (%s, %s, %s, 'admin', 'active')
                RETURNING id;
                """,
                ("seed", "Temporary Seed Worker", password_hash),
            )
            seed_id = cur.fetchone()[0]
            print(f"   ✅ Temporary admin user created. ID: {seed_id} (email='seed')")
            return seed_id


def authenticate_seed_account() -> str:
    log_step("STEP 3: Authenticating 'seed' account via API Gateway...", f"{GATEWAY_URL}/api/v1/auth/login")
    resp = requests.post(
        f"{GATEWAY_URL}/api/v1/auth/login",
        json={"email": "seed", "password": "seed"},
        timeout=10,
    )
    if not resp.ok:
        raise RuntimeError(f"Login failed: {resp.status_code} {resp.text}")
    token = resp.json().get("token")
    if not token:
        raise RuntimeError("No JWT token returned from gateway")
    print("   ✅ Successfully authenticated with API Gateway. JWT token acquired.")
    return token


def seed_data_via_gateway(token: str, data: Dict[str, Any]):
    log_step("STEP 4: Seeding Domain Entities via API Gateway REST Endpoints...")
    headers = {
        "Authorization": f"Bearer {token}",
        "Content-Type": "application/json",
    }

    # 4.1 Seed 40 Real Members via /api/v1/admin/users
    print("\n   [4.1] Seeding 40 Club Members (5 Admins, 5 Mods, 30 Members)...")
    email_to_user: Dict[str, Dict[str, Any]] = {}
    
    # Pre-hash standard member password
    member_default_password = "password123"
    member_password_hash = ph.hash(member_default_password)

    for m in data["members"]:
        req_body = {
            "email": m["email"],
            "full_name": m["full_name"],
            "role": m["role"],
        }
        resp = requests.post(f"{GATEWAY_URL}/api/v1/admin/users", json=req_body, headers=headers, timeout=10)
        if resp.status_code != 201:
            raise RuntimeError(f"Failed to create user {m['email']}: {resp.status_code} {resp.text}")
        created_user = resp.json().get("user")
        
        # Set predictable password ("password123" or custom from json)
        custom_password = m.get("password", member_default_password)
        custom_hash = member_password_hash if custom_password == member_default_password else ph.hash(custom_password)
        
        with psycopg.connect(DB_CONN_STR, autocommit=True) as conn:
            with conn.cursor() as cur:
                cur.execute(
                    "UPDATE users SET password_hash = %s WHERE id = %s;",
                    (custom_hash, created_user["id"]),
                )

        email_to_user[m["email"]] = created_user

    print(f"   ✅ Seeded {len(email_to_user)} user accounts successfully (default login password: '{member_default_password}').")

    # 4.2 Seed Active Show & Time Slots via /api/v1/events
    print("\n   [4.2] Seeding Active Show & Time Slots...")
    show_data = data["show"]
    event_payload = {
        "title": show_data["title"],
        "description": show_data["description"],
        "start_date": show_data["start_date"],
        "end_date": show_data["end_date"],
        "time_slots": show_data["time_slots"],
    }
    resp = requests.post(f"{GATEWAY_URL}/api/v1/events", json=event_payload, headers=headers, timeout=10)
    if resp.status_code != 201:
        raise RuntimeError(f"Failed to create show event: {resp.status_code} {resp.text}")
    created_show = resp.json()
    show_id = created_show["id"]
    print(f"   ✅ Created Show Event: '{created_show['title']}' (ID: {show_id})")

    # 4.3 Create Practice Sprint for Show
    print("\n   [4.3] Initializing Practice Sprint 1 for Show...")
    sprint_data = show_data["sprint"]
    sprint_id = None
    with psycopg.connect(DB_CONN_STR, autocommit=True) as conn:
        with conn.cursor() as cur:
            cur.execute(
                """
                INSERT INTO practice_sprints (event_id, name, sprint_goal, start_date, end_date, is_active)
                VALUES (%s, %s, %s, %s, %s, %s)
                RETURNING id;
                """,
                (
                    show_id,
                    sprint_data["name"],
                    sprint_data["sprint_goal"],
                    sprint_data["start_date"],
                    sprint_data["end_date"],
                    sprint_data["is_active"],
                ),
            )
            sprint_id = cur.fetchone()[0]
    print(f"   ✅ Created Practice Sprint: '{sprint_data['name']}' (ID: {sprint_id})")

    # 4.4 Seed 12 Musical Gear Items via /api/v1/music/instruments
    print("\n   [4.4] Seeding 12 Musical Instruments & Gear (Keyboards, Guitars, Drums, Audio)...")
    code_to_instrument: Dict[str, Dict[str, Any]] = {}
    for inst in data["instruments"]:
        owner_id = None
        if inst.get("owner_email") and inst["owner_email"] in email_to_user:
            owner_id = email_to_user[inst["owner_email"]]["id"]

        inst_payload = {
            "name": inst["name"],
            "code": inst["code"],
            "category": inst["category"],
            "ownership_type": inst["ownership_type"],
            "owner_user_id": owner_id,
            "custody_location": inst.get("custody_location"),
            "availability_status": inst.get("availability_status", "free_to_borrow"),
            "notes": inst.get("notes"),
        }
        resp = requests.post(f"{GATEWAY_URL}/api/v1/music/instruments", json=inst_payload, headers=headers, timeout=10)
        if resp.status_code != 201:
            raise RuntimeError(f"Failed to register instrument {inst['code']}: {resp.status_code} {resp.text}")
        created_inst = resp.json()
        code_to_instrument[inst["code"]] = created_inst

    print(f"   ✅ Registered {len(code_to_instrument)} instruments across categories.")

    # 4.5 Seed 20 Music Numbers, Lineups, Stages & QC Verdicts
    print("\n   [4.5] Seeding 20 Authentic Music Numbers with Lineups & QC Audits...")
    title_to_number: Dict[str, Dict[str, Any]] = {}
    for idx, num in enumerate(data["music_numbers"], start=1):
        pm_user = email_to_user.get(num["pm_email"])
        qc_user = email_to_user.get(num["qc_email"])
        pm_name = pm_user["full_name"] if pm_user else "Trần Minh Trí"
        qc_name = qc_user["full_name"] if qc_user else "Nguyễn Hoàng Nam"

        # 1. Create show number
        create_payload = {
            "title": num["title"],
            "genre": num["genre"],
            "pm_name": pm_name,
            "qc_reviewer": qc_name,
        }
        resp = requests.post(f"{GATEWAY_URL}/api/v1/shows/{show_id}/numbers", json=create_payload, headers=headers, timeout=10)
        if resp.status_code != 201:
            raise RuntimeError(f"Failed to create music number {num['title']}: {resp.status_code} {resp.text}")
        created_num = resp.json()
        num_id = created_num["id"]
        title_to_number[num["title"]] = created_num

        # 2. Update target sessions per week & PM in DB
        with psycopg.connect(DB_CONN_STR, autocommit=True) as conn:
            with conn.cursor() as cur:
                cur.execute(
                    """
                    UPDATE music_numbers
                    SET target_sessions_per_week = %s,
                        pm_user_id = %s,
                        description = %s
                    WHERE id = %s;
                    """,
                    (num.get("target_sessions", 2), pm_user["id"] if pm_user else None, num.get("description"), num_id),
                )

        # 3. Update lineup
        if "lineup" in num:
            resp_lineup = requests.put(
                f"{GATEWAY_URL}/api/v1/shows/{show_id}/numbers/{num_id}/lineup",
                json=num["lineup"],
                headers=headers,
                timeout=10,
            )
            if not resp_lineup.ok:
                print(f"   ⚠️ Lineup update failed for {num['title']}: {resp_lineup.text}")

            # Populate music_number_members table for DB relational fidelity
            with psycopg.connect(DB_CONN_STR, autocommit=True) as conn:
                with conn.cursor() as cur:
                    lineup = num["lineup"]
                    lineup_roles = [
                        ("vocalLead", "Lead Vocal", True),
                        ("guitarLead", "Lead Guitar", False),
                        ("bass", "Bass Guitar", False),
                        ("drums", "Drums", False),
                        ("keys", "Keyboards & Piano", False),
                    ]
                    for key, role_name, is_lead in lineup_roles:
                        member_name = lineup.get(key)
                        if member_name:
                            matched = next((u for u in email_to_user.values() if u["full_name"] == member_name), None)
                            if matched:
                                cur.execute(
                                    """
                                    INSERT INTO music_number_members (music_number_id, user_id, instrument_role, is_lead)
                                    VALUES (%s, %s, %s, %s)
                                    ON CONFLICT (music_number_id, user_id) DO NOTHING;
                                    """,
                                    (num_id, matched["id"], role_name, is_lead),
                                )

        # 4. Set stage
        target_stage = num.get("stage", "in_practice")
        resp_stage = requests.put(
            f"{GATEWAY_URL}/api/v1/shows/{show_id}/numbers/{num_id}/stage",
            json={"stage": target_stage},
            headers=headers,
            timeout=10,
        )
        if not resp_stage.ok:
            print(f"   ⚠️ Stage update failed for {num['title']}: {resp_stage.text}")

        # 5. Record QC notes if present
        if num.get("qc_notes"):
            resp_qc = requests.post(
                f"{GATEWAY_URL}/api/v1/shows/{show_id}/numbers/{num_id}/qc",
                json={"verdict": "pass" if target_stage in ["qc_approved", "stage_ready"] else "revision", "notes": num["qc_notes"]},
                headers=headers,
                timeout=10,
            )
            if sprint_id:
                with psycopg.connect(DB_CONN_STR, autocommit=True) as conn:
                    with conn.cursor() as cur:
                        cur.execute(
                            """
                            INSERT INTO practice_tasks (sprint_id, music_number_id, task_type, title, description, assigned_to, qc_reviewer_id, status, qc_feedback)
                            VALUES (%s, %s, 'review_qc', %s, %s, %s, %s, %s, %s);
                            """,
                            (
                                sprint_id,
                                num_id,
                                f"Milestone QC: {num['title']}",
                                "Verify pitch stability, tempo alignment, and harmonic balance",
                                pm_user["id"] if pm_user else None,
                                qc_user["id"] if qc_user else None,
                                "passed" if target_stage in ["qc_approved", "stage_ready"] else "under_review",
                                num["qc_notes"],
                            ),
                        )

        print(f"   ({idx:02d}/20) '{num['title']}' ({num['genre']}) -> Stage: {target_stage.upper()} | PM: {pm_name}")

    # 4.6 Seed Realistic Instrument Reservations
    print("\n   [4.6] Seeding Non-Conflicting Instrument Reservations...")
    sample_reservations = [
        ("KEYS-01", "PHONECERT", "Thứ Hai", "17:00 - 18:30", "Roland FP-30X for Phonecert rhythm rehearsal"),
        ("KEYS-02", "NÀNG THƠ", "Thứ Ba", "18:30 - 20:00", "Nord Stage 3 for Nàng Thơ grand piano rehearsal"),
        ("DRUM-01", "BẬT TÌNH YÊU LÊN", "Thứ Tư", "17:00 - 18:30", "Yamaha Birch kit for disco funk session"),
        ("GTR-01", "CƠN MƯA BĂNG GIÁ", "Thứ Sáu", "18:30 - 20:00", "Fender Strat for solo lead practice"),
        ("BASS-01", "BÊN TRÊN TẦNG LẦU", "Thứ Bảy", "15:30 - 17:00", "Ibanez Bass for synthwave groove"),
    ]
    for code, song_title, day, slot, notes in sample_reservations:
        inst = code_to_instrument.get(code)
        num = title_to_number.get(song_title)
        if inst and num:
            res_payload = {
                "instrument_id": inst["id"],
                "music_number_id": num["id"],
                "day_of_week": day,
                "slot_label": slot,
                "notes": notes,
            }
            resp = requests.post(f"{GATEWAY_URL}/api/v1/music/instruments/reserve", json=res_payload, headers=headers, timeout=10)
            if resp.ok:
                print(f"   ✅ Reserved '{code}' for '{song_title}' on {day} ({slot})")

    # 4.7 Seed 15-Minute Free-Time Sprint Matrix for Members
    if sprint_id:
        print("\n   [4.7] Registering 15-Minute Precision Sprint Free-Time Slots...")
        days = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"]
        sample_15m_slots = []
        for d in days:
            for h in [18, 19, 20]:
                for m in ["00", "15", "30", "45"]:
                    sample_15m_slots.append({"day_of_week": d, "slot_label": f"{h:02d}:{m}", "is_available": True})

        avail_resp = requests.post(
            f"{GATEWAY_URL}/api/v1/shows/{show_id}/sprints/{sprint_id}/availability",
            json={"slots": sample_15m_slots},
            headers=headers,
            timeout=10,
        )
        if avail_resp.ok:
            print(f"   ✅ Registered {len(sample_15m_slots)} 15-minute free-time slots for active sprint.")

    return show_id


def deallocate_seed_account(seed_id: str):
    log_step("STEP 5: Deallocating Temporary 'seed' Account...", f"Removing user ID: {seed_id}")
    with psycopg.connect(DB_CONN_STR, autocommit=True) as conn:
        with conn.cursor() as cur:
            cur.execute("DELETE FROM users WHERE id = %s;", (seed_id,))
            print("   ✅ Temporary 'seed' account deallocated successfully.")
            
            # Verify no remaining seed account
            cur.execute("SELECT count(*) FROM users WHERE email = 'seed';")
            count = cur.fetchone()[0]
            if count == 0:
                print("   🛡️ Security assertion verified: 0 'seed' accounts in database.")


def main():
    print("================================================================")
    print("🎸 CSAC TIMETABLE STUDIO - FULL DATABASE CLEAN & SEED PIPELINE")
    print("================================================================")

    if not DATA_FILE.exists():
        print(f"❌ Error: Mock data file '{DATA_FILE}' not found.")
        sys.exit(1)

    with open(DATA_FILE, "r", encoding="utf-8") as f:
        mock_data = json.load(f)

    try:
        # Step 1: Wipe DB
        wipe_database()

        # Step 2: Create temp seed account
        seed_id = create_temp_seed_account()

        # Step 3: Login via API Gateway
        token = authenticate_seed_account()

        # Step 4: Seed all data via API Gateway REST API
        show_id = seed_data_via_gateway(token, mock_data)

        # Step 5: Deallocate seed account
        deallocate_seed_account(seed_id)

        print("\n================================================================")
        print("🎉 SEEDING COMPLETED SUCCESSFULLY!")
        print(f"🌟 Active Show ID: {show_id}")
        print("🌟 40 Realistic Members (5 Admins, 5 Mods, 30 Members)")
        print("🔑 Login Credentials: All member accounts can log in with password: 'password123'")
        print("🌟 20 Music Numbers across 5 Stage Workflows")
        print("🌟 12 Musical Instruments & Gear (Keyboards, Guitars, Drums, Audio)")
        print("================================================================")

    except Exception as e:
        print(f"\n❌ Pipeline failed with error: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)


if __name__ == "__main__":
    main()
