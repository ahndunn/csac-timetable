-- CSAC Timetable Studio PostgreSQL Schema Initialization
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Role and Status Enums
DO $$ BEGIN
    CREATE TYPE user_role AS ENUM ('admin', 'moderator', 'member');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE user_status AS ENUM ('active', 'suspended');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE event_status AS ENUM ('draft', 'open', 'closed');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE proposal_status AS ENUM ('pending', 'approved', 'rejected', 'expired');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE vote_decision AS ENUM ('approve', 'reject');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- 1. Users Table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL UNIQUE,
    full_name VARCHAR(255) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role user_role NOT NULL DEFAULT 'member',
    status user_status NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Seed Initial Admin Account
-- password: password
INSERT INTO users (email, full_name, password_hash, role, status)
VALUES ('admin@csac.local', 'System Administrator', '$argon2d$v=19$m=16,t=2,p=1$U2lYQjBCNmlMakxURUlRag$M/e/uvcWAVwPmvflmP0Yfg', 'admin', 'active')
ON CONFLICT (email) DO UPDATE SET password_hash = EXCLUDED.password_hash, role = EXCLUDED.role, status = EXCLUDED.status;

-- 2. Events Table
CREATE TABLE IF NOT EXISTS events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    description TEXT,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    status event_status NOT NULL DEFAULT 'open',
    created_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    closed_at TIMESTAMPTZ
);

-- 3. Event Time Slots Table
CREATE TABLE IF NOT EXISTS event_time_slots (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_id UUID NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    day_of_week VARCHAR(50) NOT NULL,
    slot_label VARCHAR(100) NOT NULL,
    sort_order INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 4. Member Votes Table
CREATE TABLE IF NOT EXISTS votes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_id UUID NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    slot_id UUID NOT NULL REFERENCES event_time_slots(id) ON DELETE CASCADE,
    is_available BOOLEAN NOT NULL DEFAULT TRUE,
    note TEXT,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT unique_user_event_slot UNIQUE (event_id, user_id, slot_id)
);

-- 5. Admin Downgrade Proposals
CREATE TABLE IF NOT EXISTS admin_downgrade_proposals (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    target_admin_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    target_role user_role NOT NULL DEFAULT 'moderator',
    initiated_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    reason TEXT NOT NULL,
    total_admins_at_proposal INT NOT NULL,
    required_approvals INT NOT NULL,
    current_approvals INT NOT NULL DEFAULT 0,
    status proposal_status NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL DEFAULT (NOW() + INTERVAL '7 days')
);

-- 6. Admin Downgrade Quorum Votes
CREATE TABLE IF NOT EXISTS admin_downgrade_votes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    proposal_id UUID NOT NULL REFERENCES admin_downgrade_proposals(id) ON DELETE CASCADE,
    admin_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    decision vote_decision NOT NULL,
    voted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT unique_proposal_admin UNIQUE (proposal_id, admin_id)
);

-- 7. Audit Logs Table
CREATE TABLE IF NOT EXISTS audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    actor_id UUID REFERENCES users(id) ON DELETE SET NULL,
    action VARCHAR(100) NOT NULL,
    resource_type VARCHAR(100) NOT NULL,
    resource_id UUID,
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create Indexes for Query Performance
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_role ON users(role);
CREATE INDEX IF NOT EXISTS idx_events_status ON events(status);
CREATE INDEX IF NOT EXISTS idx_votes_event_user ON votes(event_id, user_id);
CREATE INDEX IF NOT EXISTS idx_proposals_status ON admin_downgrade_proposals(status);

-- ==========================================
-- 8. Music Organization & Fleet Management
-- ==========================================

DO $$ BEGIN
    CREATE TYPE instrument_ownership AS ENUM ('club_property', 'member_owned');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE instrument_availability AS ENUM ('free_to_borrow', 'in_use', 'unavailable', 'in_maintenance');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE music_number_status AS ENUM ('draft', 'in_practice', 'ready_for_qc', 'qc_approved', 'stage_ready');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE task_type AS ENUM ('study', 'create', 'review_qc');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE task_status AS ENUM ('todo', 'in_progress', 'under_review', 'passed', 'blocked');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- Instruments (CSAC Property & Member Personal Gear)
CREATE TABLE IF NOT EXISTS instruments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    code VARCHAR(50) NOT NULL UNIQUE,
    category VARCHAR(100) NOT NULL,
    ownership_type instrument_ownership NOT NULL DEFAULT 'club_property',
    owner_user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    custody_user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    custody_location VARCHAR(255) DEFAULT 'Club Studio Locker',
    availability_status instrument_availability NOT NULL DEFAULT 'free_to_borrow',
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Music Numbers
CREATE TABLE IF NOT EXISTS music_numbers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_id UUID REFERENCES events(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    genre VARCHAR(100),
    pm_user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    target_sessions_per_week INT NOT NULL DEFAULT 2,
    status music_number_status NOT NULL DEFAULT 'in_practice',
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Music Number Members (Performer Lineup)
CREATE TABLE IF NOT EXISTS music_number_members (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    music_number_id UUID NOT NULL REFERENCES music_numbers(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    instrument_role VARCHAR(100) NOT NULL,
    is_lead BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT unique_number_member UNIQUE (music_number_id, user_id)
);

-- Practice Sprints (Agile SDLC Sprints)
CREATE TABLE IF NOT EXISTS practice_sprints (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_id UUID REFERENCES events(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    sprint_goal TEXT,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Practice Tasks & QC Reviews
CREATE TABLE IF NOT EXISTS practice_tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    sprint_id UUID NOT NULL REFERENCES practice_sprints(id) ON DELETE CASCADE,
    music_number_id UUID NOT NULL REFERENCES music_numbers(id) ON DELETE CASCADE,
    task_type task_type NOT NULL DEFAULT 'study',
    title VARCHAR(255) NOT NULL,
    description TEXT,
    assigned_to UUID REFERENCES users(id) ON DELETE SET NULL,
    qc_reviewer_id UUID REFERENCES users(id) ON DELETE SET NULL,
    status task_status NOT NULL DEFAULT 'todo',
    qc_feedback TEXT,
    reviewed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Instrument Reservations (Zero Double-Booking Guarantee)
CREATE TABLE IF NOT EXISTS instrument_reservations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    instrument_id UUID NOT NULL REFERENCES instruments(id) ON DELETE CASCADE,
    music_number_id UUID NOT NULL REFERENCES music_numbers(id) ON DELETE CASCADE,
    reserved_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    day_of_week VARCHAR(50) NOT NULL,
    slot_label VARCHAR(100) NOT NULL,
    session_date DATE,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT unique_instrument_slot UNIQUE (instrument_id, day_of_week, slot_label)
);

-- Member Sprint Availabilities (Fast Free-Time Registration)
CREATE TABLE IF NOT EXISTS member_sprint_availabilities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    sprint_id UUID NOT NULL REFERENCES practice_sprints(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    day_of_week VARCHAR(50) NOT NULL,
    slot_label VARCHAR(100) NOT NULL,
    is_available BOOLEAN NOT NULL DEFAULT TRUE,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT unique_sprint_user_slot UNIQUE (sprint_id, user_id, day_of_week, slot_label)
);

-- Member Sprint Availabilities History Audit Log
CREATE TABLE IF NOT EXISTS member_sprint_availabilities_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    sprint_id UUID NOT NULL REFERENCES practice_sprints(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    actor_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    action VARCHAR(20) NOT NULL, -- 'ADD', 'UPDATE', 'DELETE'
    day_of_week VARCHAR(50) NOT NULL,
    slot_label VARCHAR(100) NOT NULL,
    is_available BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Practice Sprint Schedule Compute Runs & Audit History
CREATE TABLE IF NOT EXISTS sprint_schedule_runs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    sprint_id UUID NOT NULL REFERENCES practice_sprints(id) ON DELETE CASCADE,
    triggered_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    status VARCHAR(50) NOT NULL DEFAULT 'queued', -- 'queued', 'processing', 'completed', 'failed'
    duration_ms INT,
    score DOUBLE PRECISION,
    conflict_count INT DEFAULT 0,
    assignments JSONB,
    error_message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_instruments_code ON instruments(code);
CREATE INDEX IF NOT EXISTS idx_music_numbers_pm ON music_numbers(pm_user_id);
CREATE INDEX IF NOT EXISTS idx_practice_tasks_sprint ON practice_tasks(sprint_id);
CREATE INDEX IF NOT EXISTS idx_reservations_instrument ON instrument_reservations(instrument_id);
CREATE INDEX IF NOT EXISTS idx_sprint_runs_sprint ON sprint_schedule_runs(sprint_id);

-- ==========================================
-- SEED REALISTIC CSAC CLUB ASSETS & MEMBERS
-- ==========================================

-- Seed Club Members (password: password)
INSERT INTO users (id, email, full_name, password_hash, role, status)
VALUES
    ('a0000000-0000-0000-0000-000000000001', 'phap.minh@csac.local', 'Minh Pháp', '$argon2d$v=19$m=16,t=2,p=1$U2lYQjBCNmlMakxURUlRag$M/e/uvcWAVwPmvflmP0Yfg', 'moderator', 'active'),
    ('a0000000-0000-0000-0000-000000000002', 'thanh.duy@csac.local', 'Duy Thành', '$argon2d$v=19$m=16,t=2,p=1$U2lYQjBCNmlMakxURUlRag$M/e/uvcWAVwPmvflmP0Yfg', 'member', 'active'),
    ('a0000000-0000-0000-0000-000000000003', 'pha.anh@csac.local', 'Anh Pha', '$argon2d$v=19$m=16,t=2,p=1$U2lYQjBCNmlMakxURUlRag$M/e/uvcWAVwPmvflmP0Yfg', 'member', 'active'),
    ('a0000000-0000-0000-0000-000000000004', 'huy.gia@csac.local', 'Gia Huy', '$argon2d$v=19$m=16,t=2,p=1$U2lYQjBCNmlMakxURUlRag$M/e/uvcWAVwPmvflmP0Yfg', 'member', 'active'),
    ('a0000000-0000-0000-0000-000000000005', 'luc.quang@csac.local', 'Quang Lực', '$argon2d$v=19$m=16,t=2,p=1$U2lYQjBCNmlMakxURUlRag$M/e/uvcWAVwPmvflmP0Yfg', 'member', 'active')
ON CONFLICT (email) DO NOTHING;

-- Seed Default Live Event
INSERT INTO events (id, title, description, start_date, end_date, status)
VALUES (
    'e0000000-0000-0000-0000-000000000001',
    'CSAC Autumn Acoustic Concert 2026',
    'Main seasonal live showcase for Computer Science Art Club',
    '2026-09-15',
    '2026-09-30',
    'open'
) ON CONFLICT (id) DO NOTHING;

-- Seed Practice Sprint 1
INSERT INTO practice_sprints (id, event_id, name, sprint_goal, start_date, end_date, is_active)
VALUES (
    'b0000000-0000-0000-0000-000000000001',
    'e0000000-0000-0000-0000-000000000001',
    'Sprint 1: Harmonization & Rhythm Lock',
    'Complete study chord charts, vocal harmonization and pass initial QC milestone',
    '2026-09-15',
    '2026-09-22',
    true
) ON CONFLICT (id) DO NOTHING;

-- Seed Instruments (CSAC Property & Member-Owned)
INSERT INTO instruments (id, name, code, category, ownership_type, owner_user_id, custody_user_id, custody_location, availability_status, notes)
VALUES
    -- Club Property
    ('c0000000-0000-0000-0000-000000000001', 'Yamaha Stage Custom Drum Kit', 'DRUM-01', 'Percussion', 'club_property', null, null, 'Studio Room A', 'free_to_borrow', 'Complete shell pack with Zildjian cymbals'),
    ('c0000000-0000-0000-0000-000000000002', 'Roland FP-30X Digital Piano', 'KEYS-01', 'Keyboard', 'club_property', null, 'a0000000-0000-0000-0000-000000000001', 'Club Studio Locker 1', 'free_to_borrow', 'Weighted 88 keys, includes sustain pedal and stand'),
    ('c0000000-0000-0000-0000-000000000003', 'Shure SM58 Wireless Mic Set (Pair)', 'MIC-01', 'Audio Gear', 'club_property', null, null, 'Mic Storage Case #1', 'free_to_borrow', 'Includes 2 transmitters and receiver base'),
    ('c0000000-0000-0000-0000-000000000004', 'Fender Champion 50XL Guitar Amp', 'AMP-01', 'Amplifier', 'club_property', null, null, 'Studio Room B', 'free_to_borrow', '50-watt modeling amplifier with footswitch'),
    -- Member Owned
    ('c0000000-0000-0000-0000-000000000005', 'Fender Stratocaster MIJ (Lake Placid Blue)', 'GTR-M01', 'Strings', 'member_owned', 'a0000000-0000-0000-0000-000000000001', 'a0000000-0000-0000-0000-000000000001', 'Kept by Minh Pháp', 'free_to_borrow', 'Lent by Minh Pháp for band rehearsals. Handle with care!'),
    ('c0000000-0000-0000-0000-000000000006', 'Ibanez SR300E Active Bass (Weathered Black)', 'BASS-M01', 'Strings', 'member_owned', 'a0000000-0000-0000-0000-000000000003', 'a0000000-0000-0000-0000-000000000003', 'Kept by Anh Pha', 'unavailable', 'Personal bass for Song 1 only. Not for general loan.'),
    ('c0000000-0000-0000-0000-000000000007', 'Boss GT-1 Multi-Effects Pedal', 'FX-M01', 'Audio Gear', 'member_owned', 'a0000000-0000-0000-0000-000000000002', 'a0000000-0000-0000-0000-000000000002', 'Kept by Duy Thành', 'free_to_borrow', 'Multi-FX with power adapter, custom acoustic & rock presets')
ON CONFLICT (code) DO NOTHING;

-- Seed Music Numbers
INSERT INTO music_numbers (id, event_id, title, genre, pm_user_id, target_sessions_per_week, status, description)
VALUES
    ('d0000000-0000-0000-0000-000000000001', 'e0000000-0000-0000-0000-000000000001', 'PHONECERT', 'Indie Pop / Acoustic', 'a0000000-0000-0000-0000-000000000001', 3, 'in_practice', 'Acoustic arrangement with dual vocals and soft drum accompaniment'),
    ('d0000000-0000-0000-0000-000000000002', 'e0000000-0000-0000-0000-000000000001', 'NÀNG THƠ', 'Ballad', 'a0000000-0000-0000-0000-000000000002', 2, 'ready_for_qc', 'Piano ballad with cello synth and rich dynamic vocal harmonies')
ON CONFLICT (id) DO NOTHING;

-- Seed Lineup for PHONECERT
INSERT INTO music_number_members (music_number_id, user_id, instrument_role, is_lead)
VALUES
    ('d0000000-0000-0000-0000-000000000001', 'a0000000-0000-0000-0000-000000000001', 'Lead Vocal & Acoustic Guitar', true),
    ('d0000000-0000-0000-0000-000000000001', 'a0000000-0000-0000-0000-000000000002', 'Backing Vocal & Cajon/Drums', false),
    ('d0000000-0000-0000-0000-000000000001', 'a0000000-0000-0000-0000-000000000003', 'Bass Guitar', false)
ON CONFLICT (music_number_id, user_id) DO NOTHING;

-- Seed Lineup for NÀNG THƠ
INSERT INTO music_number_members (music_number_id, user_id, instrument_role, is_lead)
VALUES
    ('d0000000-0000-0000-0000-000000000002', 'a0000000-0000-0000-0000-000000000002', 'Lead Vocal', true),
    ('d0000000-0000-0000-0000-000000000001', 'a0000000-0000-0000-0000-000000000001', 'Grand Piano & Keys', false),
    ('d0000000-0000-0000-0000-000000000004', 'a0000000-0000-0000-0000-000000000004', 'Backing Vocal', false)
ON CONFLICT (music_number_id, user_id) DO NOTHING;

-- Seed Agile Practice Tasks & QC Reviews
INSERT INTO practice_tasks (sprint_id, music_number_id, task_type, title, description, assigned_to, qc_reviewer_id, status, qc_feedback)
VALUES
    ('b0000000-0000-0000-0000-000000000001', 'd0000000-0000-0000-0000-000000000001', 'study', 'Study Acoustic Guitar Chords & Intro Riff', 'Memorize bridge progression (F#m7 to B7) without chord sheet', 'a0000000-0000-0000-0000-000000000001', null, 'in_progress', null),
    ('b0000000-0000-0000-0000-000000000001', 'd0000000-0000-0000-0000-000000000001', 'create', 'Record Acoustic Demo & Harmony Guide Track', 'Record 1-take vocal and rhythm guitar guide track for band to practice', 'a0000000-0000-0000-0000-000000000001', null, 'passed', 'Clean guide track uploaded to club drive'),
    ('b0000000-0000-0000-0000-000000000001', 'd0000000-0000-0000-0000-000000000001', 'review_qc', 'Milestone 1 QC: Vocal Pitch & Harmony Audit', 'Verify lead vocal pitch stability and 3rd harmony blend during chorus', 'a0000000-0000-0000-0000-000000000001', 'a0000000-0000-0000-0000-000000000002', 'under_review', null),
    ('b0000000-0000-0000-0000-000000000001', 'd0000000-0000-0000-0000-000000000002', 'review_qc', 'Milestone 1 QC: Full Run-Through Review', 'Full piano & vocal run-through review for dynamic control', 'a0000000-0000-0000-0000-000000000002', 'a0000000-0000-0000-0000-000000000001', 'passed', 'Beautiful dynamic transition in chorus 2. Approved for stage tempo rehearsals.')
ON CONFLICT DO NOTHING;

-- Seed Valid Non-Conflicting Instrument Reservation
INSERT INTO instrument_reservations (instrument_id, music_number_id, reserved_by, day_of_week, slot_label, notes)
VALUES
    ('c0000000-0000-0000-0000-000000000002', 'd0000000-0000-0000-0000-000000000002', 'a0000000-0000-0000-0000-000000000002', 'Thứ Bảy', '18h - 19h', 'Reserved Roland FP-30X for Nàng Thơ rehearsal')
ON CONFLICT DO NOTHING;

