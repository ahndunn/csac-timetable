# Database Cleanup, Real Mock Data Definition & API Gateway Seeding Pipeline

## 1. Overview
This specification details the end-to-end database cleanup and seeding architecture using Python scripts executed via `uv` in `scripts/`.

## 2. Seeding Architecture & Flow

### 2.1 Database Wipe & Initial Temporary Seed Account
1. **Direct SQL Database Cleanup**:
   - Truncates all existing records across tables: `votes`, `event_time_slots`, `practice_tasks`, `instrument_reservations`, `member_sprint_availabilities`, `music_number_members`, `music_numbers`, `practice_sprints`, `instruments`, `admin_downgrade_votes`, `admin_downgrade_proposals`, `audit_logs`, `events`, `users`.
   - Clears Redis cache keys (`DEL cache:shows:overview:*`, `otp:*`, etc.).
2. **Initial Temporary "seed" Account Creation**:
   - Creates a temporary admin account:
     - `email`: `seed`
     - `full_name`: `Initial Seed Worker`
     - `role`: `admin`
     - `status`: `active`
     - `password_hash`: Generated via `argon2id` (using `argon2-cffi` in Python matching `csac-common`).
3. **Gateway Authentication**:
   - Logs in via `POST /api/v1/auth/login` with `email="seed"`, `password="seed"`.
   - Receives JWT Bearer token with full `admin` privileges.

### 2.2 Mock Data Specification & Entities (`scripts/data/mock_seed.json`)
The mock dataset defines realistic Vietnamese & international university music club members, gear, numbers, and show:
- **Show (1 Active Show)**:
  - Title: `CSAC Autumn Symphony & Rock Showcase 2026`
  - Venue: `CSAC Main Concert Hall`
  - Dates: `2026-10-01` to `2026-10-15`
  - Sprint: `Sprint 1: Harmony & Stage Readiness`
- **Members (40 Distinct Members)**:
  - 5 System Admins (e.g. `minh.tri@csac.studio`, `hoang.nam@csac.studio`, `thanh.truc@csac.studio`, `viet.anh@csac.studio`, `quynh.anh@csac.studio`)
  - 5 System Moderators
  - 30 Club Members
  - Role assignments:
    - 1 designated Delivery Manager (DM) for the show (e.g. `hoang.nam@csac.studio`)
    - 10 Performance Managers (PMs)
    - 10 Quality Controllers / Reviewers (QCs)
    - Multi-role distribution (members holding multiple roles across different songs and instruments)
- **Music Numbers (20 Song Numbers)**:
  - Real, authentic songs (e.g. `PHONECERT`, `NÀNG THƠ`, `BẬT TÌNH YÊU LÊN`, `ĐI GIỮA TRỜI RỰC RỠ`, `TỪNG QUEN`, `NGÀY ĐẦU TIÊN`, `CẮT ĐÔI NỖI SẦU`, `CHÂN ÁI`, `BÊN TRÊN TẦNG LẦU`, `DỰ BÁO THỜI TIẾT HÔM NAY MƯA`, `SAU LỜI TỪ KHƯỚC`, `ÁNH SAO VÀ BẦU TRỜI`, `MỘT ĐÊM SAY`, `CƠN MƯA BĂNG GIÁ`, `CHÚNG TA CỦA TƯƠNG LAI`, `HÀ NỘI 12 MÙA HOA`, `ĐƯA NHAU ĐI TRỐN`, `CHILL PHẾT`, `BÀI CA HY VỌNG`, `BƯỚC QUA MÙA CÔ ĐƠN`)
  - Assigned PMs and QCs randomly/coherently distributed from the 10 PMs and 10 QCs
  - Realistic band lineups (vocal, lead guitar, rhythm guitar, bass, drums, keys)
  - Varied stages: `draft`, `in_practice`, `ready_for_qc`, `qc_approved`, `stage_ready`
- **Music Gear / Instruments (10+ Items)**:
  - Distribution: Keyboards (most numerous, ~4-5 items), Guitars (~3-4 items), Drums/Percussion (~2 items), Audio/Mics (~2 items)
  - Dual-ownership: Mix of `club_property` and `member_owned`

### 2.3 Seeding Execution via API Gateway
All actual domain entities are seeded through Gateway REST endpoints using the JWT token:
- `POST /api/v1/events` (Create active show event)
- `POST /api/v1/admin/users` (Create users)
- `POST /api/v1/shows/:id/numbers` and `PUT /api/v1/shows/:id/numbers/:id/lineup` & `PUT .../stage` (Create & setup music numbers)
- `POST /api/v1/music/instruments` (Register gear fleet)
- `POST /api/v1/sprints/:id/tasks` (Create practice & QC tasks)
- `POST /api/v1/music/instruments/reserve` (Create non-conflicting gear reservations)
- `POST /api/v1/shows/:id/sprints/:id/availability` (Register sample 15m free-time matrix slots)

### 2.4 Cleanup & Deallocation of Temporary Account
- After seeding completes successfully, the script executes direct SQL to permanently deallocate the `seed` user account (`DELETE FROM users WHERE email = 'seed'`).
- Validates that the database only contains the intended 40 real members and active show data.

## 3. Script Structure & Tooling
- Directory: `scripts/`
  - `scripts/hash_password.py`: Standalone Argon2id password hashing & verification CLI.
  - `scripts/data/mock_seed.json`: Structured mock dataset for members, songs, gear, and show configuration.
  - `scripts/clean_and_seed.py`: Main entrypoint running via `uv run` managing cleanup, initial temporary account, API Gateway seeding, and cleanup deallocation.
  - `scripts/pyproject.toml` / inline uv dependencies (`argon2-cffi`, `psycopg[binary]`, `requests`, `redis`).
