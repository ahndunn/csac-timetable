-- CSAC Timetable Studio - Production Real Data Seeding & Audit Schema Script
ALTER TYPE user_role ADD VALUE IF NOT EXISTS 'qc';
ALTER TYPE user_role ADD VALUE IF NOT EXISTS 'pm';
ALTER TYPE user_role ADD VALUE IF NOT EXISTS 'dm';

-- 1. Create auxiliary tables if missing
CREATE TABLE IF NOT EXISTS rooms (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    capacity INT NOT NULL DEFAULT 4,
    equipment_tags JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS member_sprint_availabilities_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    sprint_id UUID NOT NULL,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    actor_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    action VARCHAR(20) NOT NULL,
    slot_label VARCHAR(100) NOT NULL,
    day_of_week VARCHAR(50) NOT NULL,
    is_available BOOLEAN NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS sprint_schedule_runs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    sprint_id UUID NOT NULL,
    triggered_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    status VARCHAR(50) NOT NULL DEFAULT 'queued',
    duration_ms INT,
    score DOUBLE PRECISION,
    conflict_count INT DEFAULT 0,
    assignments JSONB,
    error_message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);

BEGIN;

-- 2. Users across 6 role levels
INSERT INTO users (id, email, full_name, password_hash, role) VALUES
  ('a0000000-0000-0000-0000-000000000001', 'alice@csac.studio', 'Alice (Lead Vocal & Guitar)', '$2b$12$e8n1P.j1WnJ7jX8L2/Z9ve9kX6.8X8X8X8X8X8X8X8X8X8X8X8X8X', 'pm'),
  ('a0000000-0000-0000-0000-000000000002', 'bob@csac.studio', 'Bob (Drums & Percussion)', '$2b$12$e8n1P.j1WnJ7jX8L2/Z9ve9kX6.8X8X8X8X8X8X8X8X8X8X8X8X8X', 'member'),
  ('a0000000-0000-0000-0000-000000000003', 'charlie@csac.studio', 'Charlie (Bass Guitar)', '$2b$12$e8n1P.j1WnJ7jX8L2/Z9ve9kX6.8X8X8X8X8X8X8X8X8X8X8X8X8X', 'qc'),
  ('a0000000-0000-0000-0000-000000000004', 'diana@csac.studio', 'Diana (Keyboards & Synth)', '$2b$12$e8n1P.j1WnJ7jX8L2/Z9ve9kX6.8X8X8X8X8X8X8X8X8X8X8X8X8X', 'member'),
  ('a0000000-0000-0000-0000-000000000005', 'eve@csac.studio', 'Eve (Lead Guitar)', '$2b$12$e8n1P.j1WnJ7jX8L2/Z9ve9kX6.8X8X8X8X8X8X8X8X8X8X8X8X8X', 'member'),
  ('a0000000-0000-0000-0000-000000000006', 'frank@csac.studio', 'Frank (Acoustic Guitar)', '$2b$12$e8n1P.j1WnJ7jX8L2/Z9ve9kX6.8X8X8X8X8X8X8X8X8X8X8X8X8X', 'member'),
  ('a0000000-0000-0000-0000-000000000007', 'grace@csac.studio', 'Grace (Delivery Manager)', '$2b$12$e8n1P.j1WnJ7jX8L2/Z9ve9kX6.8X8X8X8X8X8X8X8X8X8X8X8X8X', 'dm'),
  ('a0000000-0000-0000-0000-000000000008', 'admin@csac.studio', 'System Administrator', '$2b$12$e8n1P.j1WnJ7jX8L2/Z9ve9kX6.8X8X8X8X8X8X8X8X8X8X8X8X8X', 'admin')
ON CONFLICT (id) DO NOTHING;

-- 3. Practice Rooms
INSERT INTO rooms (id, name, capacity, equipment_tags) VALUES
  ('b0000000-0000-0000-0000-000000000001', 'Studio A (Main Band Room)', 8, '["drumkit", "guitar_amps", "bass_amp", "piano", "pa_system"]'),
  ('b0000000-0000-0000-0000-000000000002', 'Studio B (Acoustic & Vocal)', 4, '["piano", "microphones", "acoustic_treatment"]'),
  ('b0000000-0000-0000-0000-000000000003', 'Studio C (Rehearsal Room)', 6, '["guitar_amps", "synth_rig", "pa_system"]')
ON CONFLICT (id) DO NOTHING;

COMMIT;
