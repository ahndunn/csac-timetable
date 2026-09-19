-- ============================================================================
-- CSAC Timetable Studio - Production Real Data Seeding Script
-- Single Source of Truth (SSOT) database seed exported from production pipeline
-- ============================================================================

-- 1. Ensure auxiliary schema tables & types are present
DO $$ BEGIN
    ALTER TYPE user_role ADD VALUE IF NOT EXISTS 'qc';
    ALTER TYPE user_role ADD VALUE IF NOT EXISTS 'pm';
    ALTER TYPE user_role ADD VALUE IF NOT EXISTS 'dm';
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

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

-- 2. Clean & Truncate previous data before re-seeding
TRUNCATE TABLE 
    votes,
    event_time_slots,
    practice_tasks,
    instrument_reservations,
    member_sprint_availabilities,
    member_sprint_availabilities_history,
    sprint_schedule_runs,
    music_number_members,
    music_numbers,
    practice_sprints,
    instruments,
    admin_downgrade_votes,
    admin_downgrade_proposals,
    audit_logs,
    events,
    rooms,
    users
CASCADE;

-- 3. Practice Rooms
INSERT INTO rooms (id, name, capacity, equipment_tags) VALUES
  ('b0000000-0000-0000-0000-000000000001', 'Studio A (Main Band Room)', 8, '["drumkit", "guitar_amps", "bass_amp", "piano", "pa_system"]'),
  ('b0000000-0000-0000-0000-000000000002', 'Studio B (Acoustic & Vocal)', 4, '["piano", "microphones", "acoustic_treatment"]'),
  ('b0000000-0000-0000-0000-000000000003', 'Studio C (Rehearsal Room)', 6, '["guitar_amps", "synth_rig", "pa_system"]')
ON CONFLICT (id) DO NOTHING;

-- 4. Domain Entities Data Dump (Users, Events, Time Slots, Instruments, Numbers, Members, Sprints, Tasks, Reservations, Availabilities, Audit Logs)
--
-- PostgreSQL database dump
--


-- Dumped from database version 17.11
-- Dumped by pg_dump version 17.11


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: -
--

INSERT INTO users VALUES ('6732646e-10e0-4f22-99f1-7b553e0570df', 'minh.tri@csac.studio', 'Trần Minh Trí', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'admin', 'active', '2026-09-16 14:20:53.461129+00', '2026-09-16 14:20:53.461129+00');
INSERT INTO users VALUES ('e87c1e6c-1434-46a1-ba16-42954a5f85ab', 'hoang.nam@csac.studio', 'Nguyễn Hoàng Nam', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'admin', 'active', '2026-09-16 14:20:53.505308+00', '2026-09-16 14:20:53.505308+00');
INSERT INTO users VALUES ('25ec4a6a-687a-4cad-853a-9b8c4299a6f4', 'thanh.truc@csac.studio', 'Lê Thanh Trúc', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'admin', 'active', '2026-09-16 14:20:53.548815+00', '2026-09-16 14:20:53.548815+00');
INSERT INTO users VALUES ('85ef1521-bb30-4116-a0e0-f8bd8ab87705', 'viet.anh@csac.studio', 'Phạm Việt Anh', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'admin', 'active', '2026-09-16 14:20:53.592843+00', '2026-09-16 14:20:53.592843+00');
INSERT INTO users VALUES ('1527b949-09f0-46a3-9b43-5c82d7ef5eaf', 'quynh.anh@csac.studio', 'Đỗ Quỳnh Anh', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'admin', 'active', '2026-09-16 14:20:53.641131+00', '2026-09-16 14:20:53.641131+00');
INSERT INTO users VALUES ('43b8694f-bfb7-401d-8ad1-19c7c0755362', 'phap.minh@csac.studio', 'Vũ Minh Pháp', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'moderator', 'active', '2026-09-16 14:20:53.685108+00', '2026-09-16 14:20:53.685108+00');
INSERT INTO users VALUES ('da08a51a-837f-416a-9f72-c417eb79f2f8', 'duy.thanh@csac.studio', 'Hoàng Duy Thành', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'moderator', 'active', '2026-09-16 14:20:53.730912+00', '2026-09-16 14:20:53.730912+00');
INSERT INTO users VALUES ('3260e15d-534c-4542-a26d-216122cb3be6', 'anh.pha@csac.studio', 'Bùi Anh Pha', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'moderator', 'active', '2026-09-16 14:20:53.779077+00', '2026-09-16 14:20:53.779077+00');
INSERT INTO users VALUES ('5c34609d-0cd0-4342-be02-3b42ba182a0d', 'gia.huy@csac.studio', 'Ngô Gia Huy', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'moderator', 'active', '2026-09-16 14:20:53.824022+00', '2026-09-16 14:20:53.824022+00');
INSERT INTO users VALUES ('ec4e11fa-4758-4234-aaa6-36f9a4b60d8a', 'quang.luc@csac.studio', 'Trịnh Quang Lực', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'moderator', 'active', '2026-09-16 14:20:53.887685+00', '2026-09-16 14:20:53.887685+00');
INSERT INTO users VALUES ('b17b54ab-de0a-4f99-badd-b7428ddb2f07', 'bao.tram@csac.studio', 'Nguyễn Bảo Trâm', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'member', 'active', '2026-09-16 14:20:53.942412+00', '2026-09-16 14:20:53.942412+00');
INSERT INTO users VALUES ('4de34466-4e15-4bee-be27-8fafcc8b48db', 'tuan.kiet@csac.studio', 'Đặng Tuấn Kiệt', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'member', 'active', '2026-09-16 14:20:54.005247+00', '2026-09-16 14:20:54.005247+00');
INSERT INTO users VALUES ('adb9581e-098b-44c4-b923-a8a338ff3f01', 'my.duyen@csac.studio', 'Phan Mỹ Duyên', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'member', 'active', '2026-09-16 14:20:54.04863+00', '2026-09-16 14:20:54.04863+00');
INSERT INTO users VALUES ('b31d8477-a95a-4608-b152-b307cc1973dc', 'khanh.vy@csac.studio', 'Lâm Khánh Vy', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'member', 'active', '2026-09-16 14:20:54.092333+00', '2026-09-16 14:20:54.092333+00');
INSERT INTO users VALUES ('e6d141cc-c6fb-4097-9561-f8b780d1b532', 'duc.phuc@csac.studio', 'Trương Đức Phúc', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'member', 'active', '2026-09-16 14:20:54.134296+00', '2026-09-16 14:20:54.134296+00');
INSERT INTO users VALUES ('9f5c5533-0ea4-4426-ac25-808a363459aa', 'hai.dang@csac.studio', 'Võ Hải Đăng', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'member', 'active', '2026-09-16 14:20:54.185037+00', '2026-09-16 14:20:54.185037+00');
INSERT INTO users VALUES ('2b95ecc6-b8b1-4262-942e-28b181c0750a', 'ngoc.mai@csac.studio', 'Đinh Ngọc Mai', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'member', 'active', '2026-09-16 14:20:54.2278+00', '2026-09-16 14:20:54.2278+00');
INSERT INTO users VALUES ('669615d4-241b-47ee-b8ad-a93b6c99d865', 'hoang.long@csac.studio', 'Lý Hoàng Long', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'member', 'active', '2026-09-16 14:20:54.271019+00', '2026-09-16 14:20:54.271019+00');
INSERT INTO users VALUES ('995ce344-2633-49cf-8bec-e2c5df129f8d', 'thu.thao@csac.studio', 'Cao Thu Thảo', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'member', 'active', '2026-09-16 14:20:54.312632+00', '2026-09-16 14:20:54.312632+00');
INSERT INTO users VALUES ('f71e102a-83f6-42e7-b045-838f7c735920', 'quoc.bao@csac.studio', 'Dương Quốc Bảo', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'member', 'active', '2026-09-16 14:20:54.365407+00', '2026-09-16 14:20:54.365407+00');
INSERT INTO users VALUES ('65182525-4f89-404d-b247-2ebcd90550db', 'thanh.ha@csac.studio', 'Vũ Thanh Hà', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'member', 'active', '2026-09-16 14:20:54.415843+00', '2026-09-16 14:20:54.415843+00');
INSERT INTO users VALUES ('dc8e21ef-e8b6-43aa-9dbe-e8743a01cadf', 'minh.khoa@csac.studio', 'Hồ Minh Khoa', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'member', 'active', '2026-09-16 14:20:54.460746+00', '2026-09-16 14:20:54.460746+00');
INSERT INTO users VALUES ('fc1839d0-6f0f-46d0-8c9c-76ac291c1b63', 'kim.ngan@csac.studio', 'Mai Kim Ngân', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'member', 'active', '2026-09-16 14:20:54.504997+00', '2026-09-16 14:20:54.504997+00');
INSERT INTO users VALUES ('40c323ef-f8db-412e-bb3a-532ed3ad4080', 'tien.dat@csac.studio', 'Đỗ Tiến Đạt', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'member', 'active', '2026-09-16 14:20:54.564586+00', '2026-09-16 14:20:54.564586+00');
INSERT INTO users VALUES ('583899c4-6636-429f-825e-46d0674f2ea7', 'van.anh@csac.studio', 'Tạ Vân Anh', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'member', 'active', '2026-09-16 14:20:54.608315+00', '2026-09-16 14:20:54.608315+00');
INSERT INTO users VALUES ('3946fdd0-f837-448c-b47c-ffd299a38f2c', 'xuan.bach@csac.studio', 'Chu Xuân Bách', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'member', 'active', '2026-09-16 14:20:54.652623+00', '2026-09-16 14:20:54.652623+00');
INSERT INTO users VALUES ('3a65769b-1fcf-4c23-b4c1-a0d9ee634d58', 'phuong.thao@csac.studio', 'Đoàn Phương Thảo', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'member', 'active', '2026-09-16 14:20:54.704282+00', '2026-09-16 14:20:54.704282+00');
INSERT INTO users VALUES ('9e178c56-4003-409f-bef9-8a04a8f2d5de', 'tan.phat@csac.studio', 'La Tấn Phát', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'member', 'active', '2026-09-16 14:20:54.748426+00', '2026-09-16 14:20:54.748426+00');
INSERT INTO users VALUES ('eaf3aec1-7c19-4021-badc-bf816a278aaf', 'thuy.tien@csac.studio', 'Lưu Thủy Tiên', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'member', 'active', '2026-09-16 14:20:54.792414+00', '2026-09-16 14:20:54.792414+00');
INSERT INTO users VALUES ('27b4e667-b160-4817-a947-3098853227f3', 'duc.anh@csac.studio', 'Trịnh Đức Anh', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'member', 'active', '2026-09-16 14:20:54.837414+00', '2026-09-16 14:20:54.837414+00');
INSERT INTO users VALUES ('43210b26-d1f3-4891-b1aa-e0abedd6fdb1', 'bich.ngoc@csac.studio', 'Phùng Bích Ngọc', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'member', 'active', '2026-09-16 14:20:54.88656+00', '2026-09-16 14:20:54.88656+00');
INSERT INTO users VALUES ('9ef6d8df-177a-46ad-841a-20c1835a88d1', 'hieu.nghia@csac.studio', 'Đinh Hiếu Nghĩa', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'member', 'active', '2026-09-16 14:20:54.934446+00', '2026-09-16 14:20:54.934446+00');
INSERT INTO users VALUES ('7bb6e2d4-7e59-4f4e-8ff3-9a22305cb2af', 'yen.nhi@csac.studio', 'Tô Yến Nhi', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'member', 'active', '2026-09-16 14:20:54.977078+00', '2026-09-16 14:20:54.977078+00');
INSERT INTO users VALUES ('51618637-3131-428c-a8ee-63789cb8b505', 'trong.nhan@csac.studio', 'Quách Trọng Nhân', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'member', 'active', '2026-09-16 14:20:55.019871+00', '2026-09-16 14:20:55.019871+00');
INSERT INTO users VALUES ('04fbe6ca-a911-4ec0-bbbc-91b6aebd1b9b', 'lan.huong@csac.studio', 'Kiều Lan Hương', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'member', 'active', '2026-09-16 14:20:55.063879+00', '2026-09-16 14:20:55.063879+00');
INSERT INTO users VALUES ('8a46ea77-ab09-4da0-9ee8-1f392154766a', 'gia.bao@csac.studio', 'Ân Gia Bảo', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'member', 'active', '2026-09-16 14:20:55.106838+00', '2026-09-16 14:20:55.106838+00');
INSERT INTO users VALUES ('a13911a7-cdfa-4575-a82a-338723910638', 'hong.anh@csac.studio', 'Châu Hồng Ánh', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'member', 'active', '2026-09-16 14:20:55.149125+00', '2026-09-16 14:20:55.149125+00');
INSERT INTO users VALUES ('45bdabb9-f105-4bbd-bc74-32ac293baf01', 'hoang.bach@csac.studio', 'Mã Hoàng Bách', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'member', 'active', '2026-09-16 14:20:55.288833+00', '2026-09-16 14:20:55.288833+00');
INSERT INTO users VALUES ('cec5e868-9227-4958-a802-6cf39477451d', 'quang.minh@csac.studio', 'Thái Quang Minh', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'member', 'active', '2026-09-16 14:20:55.201261+00', '2026-09-16 14:20:55.201261+00');
INSERT INTO users VALUES ('0679280e-5d18-4960-86da-e33584e8f601', 'tram.anh@csac.studio', 'Khổng Trâm Anh', '$argon2id$v=19$m=65536,t=3,p=4$F6MPAWt55DAvxNJSMS9Fgw$VmklAqFWl1aRlq9246dOMXYNbsBBi2wftKvxbL4wSVY', 'member', 'active', '2026-09-16 14:20:55.245737+00', '2026-09-16 14:20:55.245737+00');


--
-- Data for Name: admin_downgrade_proposals; Type: TABLE DATA; Schema: public; Owner: -
--



--
-- Data for Name: admin_downgrade_votes; Type: TABLE DATA; Schema: public; Owner: -
--



--
-- Data for Name: audit_logs; Type: TABLE DATA; Schema: public; Owner: -
--

INSERT INTO audit_logs VALUES ('882a9b37-bca0-4fdb-9290-1afe2bc0c14e', NULL, 'QC_AUDIT_SUBMIT', 'music_numbers', 'a6794d91-e226-4e94-9acc-103cccfed2f1', '{"notes": "Milestone QC passed! Pitch balance and vocal harmony in the bridge locked at 100%. Approved for Main Concert Stage.", "verdict": "pass"}', '2026-09-16 14:20:55.461532+00');
INSERT INTO audit_logs VALUES ('6f555ddc-9934-4fd2-91a2-368ad8466a23', NULL, 'QC_AUDIT_SUBMIT', 'music_numbers', 'efa1dcb5-3209-4c6e-bfc8-8a6b76828f82', '{"notes": "Vocal pitch and piano rubato phrasing passed inspection with distinction.", "verdict": "pass"}', '2026-09-16 14:20:55.531973+00');
INSERT INTO audit_logs VALUES ('ae34f31b-443e-45ca-b96e-c2582f2a3dfe', NULL, 'QC_AUDIT_SUBMIT', 'music_numbers', 'fd1e4824-44b6-4097-be41-7d5fa2277e2b', '{"notes": "Rhythm section is tight, brass synth stabs fit the mix cleanly. Full pass.", "verdict": "pass"}', '2026-09-16 14:20:55.602044+00');
INSERT INTO audit_logs VALUES ('f6bbc401-f4b2-4354-9d39-9b05bd803227', NULL, 'QC_AUDIT_SUBMIT', 'music_numbers', 'df265763-3fe7-4e33-b91f-32144cbeff8d', '{"notes": "Dynamic crescendo in chorus 3 is vibrant. Lead vocal passed.", "verdict": "pass"}', '2026-09-16 14:20:55.674223+00');
INSERT INTO audit_logs VALUES ('d36e59f7-4b07-449d-8cf7-0654f6cd132d', NULL, 'QC_AUDIT_SUBMIT', 'music_numbers', '001ab9ed-e397-439d-8815-677c7268530a', '{"notes": "Rehearsal recordings submitted. QC review scheduled for Friday night.", "verdict": "revision"}', '2026-09-16 14:20:55.740111+00');
INSERT INTO audit_logs VALUES ('3fa4ec6f-8d88-434c-98ba-d243335ec5d3', NULL, 'QC_AUDIT_SUBMIT', 'music_numbers', '088720cc-c3f3-4561-a7c1-b9194a953bb7', '{"notes": "Pending audit on traditional string synth backing blend.", "verdict": "revision"}', '2026-09-16 14:20:55.916762+00');
INSERT INTO audit_logs VALUES ('1c62fb7d-34e1-4f76-9f2b-a117339e868d', NULL, 'QC_AUDIT_SUBMIT', 'music_numbers', 'a178de0a-e3b9-489c-94c1-4dff23221186', '{"notes": "Synth arpeggiator sync and tempo consistency passed.", "verdict": "pass"}', '2026-09-16 14:20:55.986645+00');
INSERT INTO audit_logs VALUES ('c1a44a13-3394-4f9a-9cba-2d0b909bc00e', NULL, 'QC_AUDIT_SUBMIT', 'music_numbers', '1503e433-f90c-49a1-b328-90214f0d1c41', '{"notes": "Superb dynamic transition and emotional impact. Stage ready!", "verdict": "pass"}', '2026-09-16 14:20:56.106121+00');
INSERT INTO audit_logs VALUES ('1debd3e1-8cf5-4d4e-91bd-86b79ccc1a02', NULL, 'QC_AUDIT_SUBMIT', 'music_numbers', '47362964-5de3-40f2-9ef6-9792bd422c3d', '{"notes": "Waltz swing feel and vocal phrasing approved.", "verdict": "pass"}', '2026-09-16 14:20:56.224179+00');
INSERT INTO audit_logs VALUES ('5c44ed6a-5d38-454a-be41-0c80a95ce1fa', NULL, 'QC_AUDIT_SUBMIT', 'music_numbers', '7e34be8b-4c6a-4611-ab88-bd2ae1337a03', '{"notes": "Guitar solo and high C vocal sustain verified cleanly.", "verdict": "pass"}', '2026-09-16 14:20:56.295664+00');
INSERT INTO audit_logs VALUES ('a79546c6-1c8c-460a-89dd-518bfeb3aba2', NULL, 'QC_AUDIT_SUBMIT', 'music_numbers', 'b20ee408-0084-4eb6-a189-e26d02396119', '{"notes": "Rehearsal audio uploaded. Audit pending.", "verdict": "revision"}', '2026-09-16 14:20:56.415704+00');
INSERT INTO audit_logs VALUES ('0dfe4ff7-9b65-4523-8cce-b7fbca0ac5f6', NULL, 'QC_AUDIT_SUBMIT', 'music_numbers', 'da717054-7b5d-4c98-9287-1d1d9e1c7cf8', '{"notes": "Show finale centerpiece approved with honors by all reviewers.", "verdict": "pass"}', '2026-09-16 14:20:56.587731+00');
INSERT INTO audit_logs VALUES ('8ba99474-c0e6-40bf-818b-e7355aa6049a', NULL, 'QC_AUDIT_SUBMIT', 'music_numbers', '31703493-1c88-458e-87d5-ffb1d4036d65', '{"notes": "Guitar ambient swells and vocal pitch tracking verified.", "verdict": "pass"}', '2026-09-16 14:20:56.661469+00');


--
-- Data for Name: events; Type: TABLE DATA; Schema: public; Owner: -
--

INSERT INTO events VALUES ('6db79fa8-e627-428e-b801-984eed4235b9', 'CSAC Autumn Symphony & Rock Showcase 2026', 'Main seasonal production showcasing 20 dynamic acoustic, rock, and pop arrangements across the university campus.', '2026-10-01', '2026-10-15', 'open', NULL, '2026-09-16 14:20:55.31911+00', NULL);


--
-- Data for Name: event_time_slots; Type: TABLE DATA; Schema: public; Owner: -
--

INSERT INTO event_time_slots VALUES ('64e6817a-6df7-4a99-9cbd-0b6f569dd801', '6db79fa8-e627-428e-b801-984eed4235b9', 'Thứ Hai', '17:00 - 18:30', 1, '2026-09-16 14:20:55.31911+00');
INSERT INTO event_time_slots VALUES ('1eb26309-968a-4223-8c7f-f7b826540fb1', '6db79fa8-e627-428e-b801-984eed4235b9', 'Thứ Hai', '18:30 - 20:00', 2, '2026-09-16 14:20:55.31911+00');
INSERT INTO event_time_slots VALUES ('d72a0547-fc05-4b13-a567-ea670f5dd14f', '6db79fa8-e627-428e-b801-984eed4235b9', 'Thứ Ba', '17:00 - 18:30', 3, '2026-09-16 14:20:55.31911+00');
INSERT INTO event_time_slots VALUES ('22bfb3c7-8244-4b04-b689-a418bdbfb07a', '6db79fa8-e627-428e-b801-984eed4235b9', 'Thứ Ba', '18:30 - 20:00', 4, '2026-09-16 14:20:55.31911+00');
INSERT INTO event_time_slots VALUES ('7420b709-b959-4152-ab4f-811caa64a919', '6db79fa8-e627-428e-b801-984eed4235b9', 'Thứ Tư', '17:00 - 18:30', 5, '2026-09-16 14:20:55.31911+00');
INSERT INTO event_time_slots VALUES ('e06308c0-4ed3-41d7-88dc-05ce247e1b98', '6db79fa8-e627-428e-b801-984eed4235b9', 'Thứ Tư', '18:30 - 20:00', 6, '2026-09-16 14:20:55.31911+00');
INSERT INTO event_time_slots VALUES ('c0a5bfa7-6efc-4601-94c8-84ef98d4d0fd', '6db79fa8-e627-428e-b801-984eed4235b9', 'Thứ Năm', '17:00 - 18:30', 7, '2026-09-16 14:20:55.31911+00');
INSERT INTO event_time_slots VALUES ('673712ad-67e3-4e36-8fd4-3b24d2657445', '6db79fa8-e627-428e-b801-984eed4235b9', 'Thứ Năm', '18:30 - 20:00', 8, '2026-09-16 14:20:55.31911+00');
INSERT INTO event_time_slots VALUES ('2976f393-4fc9-4b60-b88e-9eed8ab5dd5f', '6db79fa8-e627-428e-b801-984eed4235b9', 'Thứ Sáu', '17:00 - 18:30', 9, '2026-09-16 14:20:55.31911+00');
INSERT INTO event_time_slots VALUES ('774deacf-df6c-443d-b867-57113c332b45', '6db79fa8-e627-428e-b801-984eed4235b9', 'Thứ Sáu', '18:30 - 20:00', 10, '2026-09-16 14:20:55.31911+00');
INSERT INTO event_time_slots VALUES ('ee6eb758-b3a7-468e-9d85-7dd3096a8e2f', '6db79fa8-e627-428e-b801-984eed4235b9', 'Thứ Bảy', '14:00 - 15:30', 11, '2026-09-16 14:20:55.31911+00');
INSERT INTO event_time_slots VALUES ('a1483052-0cbd-4723-8663-ddce8397498f', '6db79fa8-e627-428e-b801-984eed4235b9', 'Thứ Bảy', '15:30 - 17:00', 12, '2026-09-16 14:20:55.31911+00');
INSERT INTO event_time_slots VALUES ('38a555c9-ba2c-424f-837b-7309df1faf2d', '6db79fa8-e627-428e-b801-984eed4235b9', 'Thứ Bảy', '18:00 - 19:30', 13, '2026-09-16 14:20:55.31911+00');
INSERT INTO event_time_slots VALUES ('4b6ed952-e7b0-4a03-9518-a411c4861e70', '6db79fa8-e627-428e-b801-984eed4235b9', 'Chủ Nhật', '14:00 - 15:30', 14, '2026-09-16 14:20:55.31911+00');
INSERT INTO event_time_slots VALUES ('d9741705-13b1-4c82-b091-0d4ece82afd6', '6db79fa8-e627-428e-b801-984eed4235b9', 'Chủ Nhật', '15:30 - 17:00', 15, '2026-09-16 14:20:55.31911+00');
INSERT INTO event_time_slots VALUES ('5bb9bd78-fd83-4fc7-ae74-322334760ab1', '6db79fa8-e627-428e-b801-984eed4235b9', 'Chủ Nhật', '18:00 - 19:30', 16, '2026-09-16 14:20:55.31911+00');


--
-- Data for Name: instruments; Type: TABLE DATA; Schema: public; Owner: -
--

INSERT INTO instruments VALUES ('aca31898-2640-4aac-bd98-545099643a08', 'Roland FP-30X Digital Piano', 'KEYS-01', 'Keyboard', 'club_property', NULL, NULL, 'Club Studio Locker 1', 'free_to_borrow', '88 weighted keys, Roland PHA-4 action, includes damper pedal and stand.', '2026-09-16 14:20:55.340932+00', '2026-09-16 14:20:55.340932+00');
INSERT INTO instruments VALUES ('c6e2443e-9347-479e-9141-c16ae1ce23d9', 'Nord Stage 3 88-Key Stage Keyboard', 'KEYS-02', 'Keyboard', 'club_property', NULL, NULL, 'Studio Room A', 'free_to_borrow', 'Flagship stage synth & grand piano library with dual OLED displays.', '2026-09-16 14:20:55.347964+00', '2026-09-16 14:20:55.347964+00');
INSERT INTO instruments VALUES ('5c1747ce-11dd-46ec-a68f-62afe34fa724', 'Yamaha MODX8+ Synthesizer Workstation', 'KEYS-03', 'Keyboard', 'member_owned', '25ec4a6a-687a-4cad-853a-9b8c4299a6f4', NULL, 'Kept by Lê Thanh Trúc', 'free_to_borrow', '88 weighted keys, FM-X and AWM2 engines. Lent for concert rehearsals.', '2026-09-16 14:20:55.353866+00', '2026-09-16 14:20:55.353866+00');
INSERT INTO instruments VALUES ('b2cd8ff8-064d-43c7-9912-acea760be22c', 'Korg Kross 2 61 Synthesizer', 'KEYS-04', 'Keyboard', 'member_owned', '5c34609d-0cd0-4342-be02-3b42ba182a0d', NULL, 'Kept by Ngô Gia Huy', 'free_to_borrow', 'Ultra-lightweight 61 synth keys, custom EDM and ballad pad patches.', '2026-09-16 14:20:55.360407+00', '2026-09-16 14:20:55.360407+00');
INSERT INTO instruments VALUES ('c121b380-3895-4abe-a2db-6ed6a57c414a', 'Fender Player Stratocaster (Lake Placid Blue)', 'GTR-01', 'Strings', 'club_property', NULL, NULL, 'Club Studio Rack #1', 'free_to_borrow', 'SSS Alnico 5 pickups, freshly set up with Elixir 10-46 strings.', '2026-09-16 14:20:55.365957+00', '2026-09-16 14:20:55.365957+00');
INSERT INTO instruments VALUES ('ed39f94f-82e0-4949-9b86-cdee2a07c938', 'Gibson Les Paul Studio (Ebony)', 'GTR-02', 'Strings', 'member_owned', '43b8694f-bfb7-401d-8ad1-19c7c0755362', NULL, 'Kept by Vũ Minh Pháp', 'free_to_borrow', '490R/498T humbuckers, high gain rock and blues tone.', '2026-09-16 14:20:55.371435+00', '2026-09-16 14:20:55.371435+00');
INSERT INTO instruments VALUES ('190c6ce1-34b8-4991-9351-a438ab5a4fba', 'Taylor 214ce Plus Acoustic-Electric Guitar', 'GTR-03', 'Strings', 'club_property', NULL, NULL, 'Acoustic Storage Case #2', 'free_to_borrow', 'Solid Sitka spruce top with ES2 electronics for pristine acoustic tone.', '2026-09-16 14:20:55.377102+00', '2026-09-16 14:20:55.377102+00');
INSERT INTO instruments VALUES ('f80b1967-c14a-4021-a543-8a1931b944e5', 'Ibanez SR500E Active Bass (Brown Mahogany)', 'BASS-01', 'Strings', 'club_property', NULL, NULL, 'Studio Room A Rack', 'free_to_borrow', 'Bartolini BH2 pickups, active 3-band EQ with mid-frequency switch.', '2026-09-16 14:20:55.382412+00', '2026-09-16 14:20:55.382412+00');
INSERT INTO instruments VALUES ('e0292cd6-f212-4b7a-8155-78c0194651de', 'Yamaha Stage Custom Birch Drum Kit (Raven Black)', 'DRUM-01', 'Percussion', 'club_property', NULL, NULL, 'Studio Room A', 'free_to_borrow', '5-piece 100% birch shell pack with Zildjian A Custom cymbal set.', '2026-09-16 14:20:55.387472+00', '2026-09-16 14:20:55.387472+00');
INSERT INTO instruments VALUES ('8c6963af-16db-427c-8418-9eb4bc6ec233', 'Roland TD-17KVX V-Drums Electronic Kit', 'DRUM-02', 'Percussion', 'club_property', NULL, NULL, 'Studio Room B', 'free_to_borrow', 'Mesh heads with VH-10 hi-hat stand, ideal for quiet night practices.', '2026-09-16 14:20:55.393374+00', '2026-09-16 14:20:55.393374+00');
INSERT INTO instruments VALUES ('b829dc20-2aba-48e0-858a-32ba1b2b9316', 'Shure SM58 Wireless Microphone System (Dual Handheld)', 'MIC-01', 'Audio Gear', 'club_property', NULL, NULL, 'Mic Flight Case #1', 'free_to_borrow', 'Dual BLX288/PG58 handheld wireless transmitters with receiver base.', '2026-09-16 14:20:55.398589+00', '2026-09-16 14:20:55.398589+00');
INSERT INTO instruments VALUES ('dcc8626f-11a7-466b-afa2-3c2f4933ee9b', 'Boss GT-1000 Multi-Effects Guitar Processor', 'FX-01', 'Audio Gear', 'member_owned', '6732646e-10e0-4f22-99f1-7b553e0570df', NULL, 'Kept by Trần Minh Trí', 'free_to_borrow', 'AIRD technology 32-bit DSP engine with full club setlist presets.', '2026-09-16 14:20:55.40369+00', '2026-09-16 14:20:55.40369+00');


--
-- Data for Name: music_numbers; Type: TABLE DATA; Schema: public; Owner: -
--

INSERT INTO music_numbers VALUES ('a6794d91-e226-4e94-9acc-103cccfed2f1', '6db79fa8-e627-428e-b801-984eed4235b9', 'PHONECERT', 'Indie Pop / Acoustic', '43b8694f-bfb7-401d-8ad1-19c7c0755362', 3, 'qc_approved', 'Upbeat acoustic pop piece with dual harmonized vocals, rhythmic acoustic guitar, and soft drum pocket.', '2026-09-16 14:20:55.409311+00', '2026-09-16 14:20:55.459592+00');
INSERT INTO music_numbers VALUES ('98e02096-399e-41e0-9fba-9cc207b08f66', '6db79fa8-e627-428e-b801-984eed4235b9', 'DỰ BÁO THỜI TIẾT HÔM NAY MƯA', 'R&B / Ballad', '43210b26-d1f3-4891-b1aa-e0abedd6fdb1', 2, 'in_practice', 'Lush Rhodes piano ballad with rain ambient effects, tight rim-shot drums, and 5-part harmonies.', '2026-09-16 14:20:56.006249+00', '2026-09-16 14:20:56.049026+00');
INSERT INTO music_numbers VALUES ('efa1dcb5-3209-4c6e-bfc8-8a6b76828f82', '6db79fa8-e627-428e-b801-984eed4235b9', 'NÀNG THƠ', 'Ballad', 'da08a51a-837f-416a-9f72-c417eb79f2f8', 2, 'qc_approved', 'Emotional grand piano ballad with warm cello synth pads and soaring tenor vocal climax.', '2026-09-16 14:20:55.481199+00', '2026-09-16 14:20:55.53022+00');
INSERT INTO music_numbers VALUES ('fd1e4824-44b6-4097-be41-7d5fa2277e2b', '6db79fa8-e627-428e-b801-984eed4235b9', 'BẬT TÌNH YÊU LÊN', 'Pop Dance / Disco Funk', 'b17b54ab-de0a-4f99-badd-b7428ddb2f07', 3, 'qc_approved', 'High-energy disco funk arrangement with slap bass grooves and sparkling 80s synth brass.', '2026-09-16 14:20:55.551479+00', '2026-09-16 14:20:55.600297+00');
INSERT INTO music_numbers VALUES ('df265763-3fe7-4e33-b91f-32144cbeff8d', '6db79fa8-e627-428e-b801-984eed4235b9', 'ĐI GIỮA TRỜI RỰC RỠ', 'Folk Contemporary / Pop', 'b31d8477-a95a-4608-b152-b307cc1973dc', 2, 'qc_approved', 'Inspiring folk-pop anthem celebrating youth, featuring acoustic fingerstyle and choir chorus.', '2026-09-16 14:20:55.62198+00', '2026-09-16 14:20:55.672519+00');
INSERT INTO music_numbers VALUES ('b20ee408-0084-4eb6-a189-e26d02396119', '6db79fa8-e627-428e-b801-984eed4235b9', 'HÀ NỘI 12 MÙA HOA', 'Chamber Pop / Acoustic', 'b31d8477-a95a-4608-b152-b307cc1973dc', 2, 'in_practice', 'Elegant arrangement honoring Hanoi with violin synth lines, classical acoustic guitar, and pure vocal tone.', '2026-09-16 14:20:56.367127+00', '2026-09-16 14:20:56.414055+00');
INSERT INTO music_numbers VALUES ('001ab9ed-e397-439d-8815-677c7268530a', '6db79fa8-e627-428e-b801-984eed4235b9', 'TỪNG QUEN', 'R&B / Soul', '2b95ecc6-b8b1-4262-942e-28b181c0750a', 2, 'in_practice', 'Smooth R&B groove with neo-soul electric piano chords and expressive vocal runs.', '2026-09-16 14:20:55.691794+00', '2026-09-16 14:20:55.73851+00');
INSERT INTO music_numbers VALUES ('fae72639-4145-45e0-8a8f-7b7c8e1368e8', '6db79fa8-e627-428e-b801-984eed4235b9', 'NGÀY ĐẦU TIÊN', 'Acoustic Pop', '65182525-4f89-404d-b247-2ebcd90550db', 2, 'in_practice', 'Sweet acoustic wedding pop song with dual acoustic guitars and shakers.', '2026-09-16 14:20:55.759699+00', '2026-09-16 14:20:55.810881+00');
INSERT INTO music_numbers VALUES ('bf05133e-6c6c-4abe-ba87-65526ca7175f', '6db79fa8-e627-428e-b801-984eed4235b9', 'CẮT ĐÔI NỖI SẦU', 'House Pop / EDM', 'dc8e21ef-e8b6-43aa-9dbe-e8743a01cadf', 2, 'in_practice', 'Dance-pop beat with four-on-the-floor kick, synth bass arpeggio, and live energetic vocals.', '2026-09-16 14:20:55.816786+00', '2026-09-16 14:20:55.85963+00');
INSERT INTO music_numbers VALUES ('1503e433-f90c-49a1-b328-90214f0d1c41', '6db79fa8-e627-428e-b801-984eed4235b9', 'SAU LỜI TỪ KHƯỚC', 'Cinematic Ballad', '04fbe6ca-a911-4ec0-bbbc-91b6aebd1b9b', 3, 'qc_approved', 'Grand cinematic score arrangement with emotional grand piano, building into thunderous rock drums.', '2026-09-16 14:20:56.054716+00', '2026-09-16 14:20:56.104515+00');
INSERT INTO music_numbers VALUES ('088720cc-c3f3-4561-a7c1-b9194a953bb7', '6db79fa8-e627-428e-b801-984eed4235b9', 'CHÂN ÁI', 'Contemporary R&B / World Music', '583899c4-6636-429f-825e-46d0674f2ea7', 2, 'in_practice', 'Dramatic blend of traditional Asian scales with modern trip-hop drums and electric guitar riffs.', '2026-09-16 14:20:55.865909+00', '2026-09-16 14:20:55.914887+00');
INSERT INTO music_numbers VALUES ('a178de0a-e3b9-489c-94c1-4dff23221186', '6db79fa8-e627-428e-b801-984eed4235b9', 'BÊN TRÊN TẦNG LẦU', 'Synthwave / Dark Pop', 'eaf3aec1-7c19-4021-badc-bf816a278aaf', 2, 'qc_approved', 'Retro synthwave baseline with pulsing 80s drum machines and moody vocal tone.', '2026-09-16 14:20:55.936171+00', '2026-09-16 14:20:55.984637+00');
INSERT INTO music_numbers VALUES ('53f72023-d86e-464e-beaa-bc1ba3438de0', '6db79fa8-e627-428e-b801-984eed4235b9', 'ÁNH SAO VÀ BẦU TRỜI', 'Acoustic Pop', '0679280e-5d18-4960-86da-e33584e8f601', 2, 'in_practice', 'Warm acoustic duet with gentle nylon guitar picking and soothing keyboard pads.', '2026-09-16 14:20:56.124496+00', '2026-09-16 14:20:56.167726+00');
INSERT INTO music_numbers VALUES ('47362964-5de3-40f2-9ef6-9792bd422c3d', '6db79fa8-e627-428e-b801-984eed4235b9', 'MỘT ĐÊM SAY', 'Indie Acoustic / Waltz', '43b8694f-bfb7-401d-8ad1-19c7c0755362', 2, 'qc_approved', 'Classic 3/4 waltz rhythm with romantic acoustic guitar chords and playful vocal modulation.', '2026-09-16 14:20:56.173937+00', '2026-09-16 14:20:56.222497+00');
INSERT INTO music_numbers VALUES ('7e34be8b-4c6a-4611-ab88-bd2ae1337a03', '6db79fa8-e627-428e-b801-984eed4235b9', 'CƠN MƯA BĂNG GIÁ', 'Power Ballad', 'da08a51a-837f-416a-9f72-c417eb79f2f8', 2, 'qc_approved', 'Legendary power ballad with screaming guitar solo, driving rock bass, and powerful high notes.', '2026-09-16 14:20:56.243024+00', '2026-09-16 14:20:56.292865+00');
INSERT INTO music_numbers VALUES ('0fc68ab8-0ebf-46e8-9dfb-da9ce47ba705', '6db79fa8-e627-428e-b801-984eed4235b9', 'CHÚNG TA CỦA TƯƠNG LAI', 'Synth Pop / R&B', 'b17b54ab-de0a-4f99-badd-b7428ddb2f07', 2, 'draft', 'Modern synth-pop track with dynamic drops, 808 sub bass, and rhythmic lead vocals.', '2026-09-16 14:20:56.318353+00', '2026-09-16 14:20:56.361497+00');
INSERT INTO music_numbers VALUES ('cb26d111-ee77-45cb-9fcf-342b48f950f4', '6db79fa8-e627-428e-b801-984eed4235b9', 'ĐƯA NHAU ĐI TRỐN', 'Indie Hip-hop / Acoustic', '2b95ecc6-b8b1-4262-942e-28b181c0750a', 2, 'in_practice', 'Signature acoustic hip-hop duet with energetic acoustic strumming and melodic rap verses.', '2026-09-16 14:20:56.434595+00', '2026-09-16 14:20:56.476708+00');
INSERT INTO music_numbers VALUES ('c630f95a-5bd6-4b34-a010-cc4d6025f1f7', '6db79fa8-e627-428e-b801-984eed4235b9', 'CHILL PHẾT', 'Lo-fi / Pop', '65182525-4f89-404d-b247-2ebcd90550db', 2, 'in_practice', 'Relaxing lo-fi acoustic track with laidback snare brushes and Rhodes keyboard textures.', '2026-09-16 14:20:56.482834+00', '2026-09-16 14:20:56.528554+00');
INSERT INTO music_numbers VALUES ('da717054-7b5d-4c98-9287-1d1d9e1c7cf8', '6db79fa8-e627-428e-b801-984eed4235b9', 'BÀI CA HY VỌNG', 'Classical Epic / Symphony', '04fbe6ca-a911-4ec0-bbbc-91b6aebd1b9b', 3, 'qc_approved', 'Grand symphonic choral arrangement featuring all section leaders and full dynamic orchestration.', '2026-09-16 14:20:56.534784+00', '2026-09-16 14:20:56.585928+00');
INSERT INTO music_numbers VALUES ('31703493-1c88-458e-87d5-ffb1d4036d65', '6db79fa8-e627-428e-b801-984eed4235b9', 'BƯỚC QUA MÙA CÔ ĐƠN', 'Indie Ballad', '0679280e-5d18-4960-86da-e33584e8f601', 2, 'qc_approved', 'Atmospheric indie ballad with reverberant electric guitar swells and tender vocal melodies.', '2026-09-16 14:20:56.607091+00', '2026-09-16 14:20:56.659672+00');


--
-- Data for Name: instrument_reservations; Type: TABLE DATA; Schema: public; Owner: -
--

INSERT INTO instrument_reservations VALUES ('bd3cd9dc-ec86-4581-a32a-f90ef44d830f', 'aca31898-2640-4aac-bd98-545099643a08', 'a6794d91-e226-4e94-9acc-103cccfed2f1', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Thứ Hai', '17:00 - 18:30', NULL, 'Roland FP-30X for Phonecert rhythm rehearsal', '2026-09-16 14:20:56.807372+00');
INSERT INTO instrument_reservations VALUES ('100f0d5f-4fac-4662-b3a4-e66cfcb6192f', 'c6e2443e-9347-479e-9141-c16ae1ce23d9', 'efa1dcb5-3209-4c6e-bfc8-8a6b76828f82', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Thứ Ba', '18:30 - 20:00', NULL, 'Nord Stage 3 for Nàng Thơ grand piano rehearsal', '2026-09-16 14:20:56.816098+00');
INSERT INTO instrument_reservations VALUES ('d86f97cb-24b6-47b7-ab01-1ec72aceb071', 'e0292cd6-f212-4b7a-8155-78c0194651de', 'fd1e4824-44b6-4097-be41-7d5fa2277e2b', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Thứ Tư', '17:00 - 18:30', NULL, 'Yamaha Birch kit for disco funk session', '2026-09-16 14:20:56.822339+00');
INSERT INTO instrument_reservations VALUES ('5d2acc25-efe5-4e89-8fcf-46d68082f99e', 'c121b380-3895-4abe-a2db-6ed6a57c414a', '7e34be8b-4c6a-4611-ab88-bd2ae1337a03', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Thứ Sáu', '18:30 - 20:00', NULL, 'Fender Strat for solo lead practice', '2026-09-16 14:20:56.828527+00');
INSERT INTO instrument_reservations VALUES ('c0142ca4-e0cd-43b5-a2dc-3c9e49acb499', 'f80b1967-c14a-4021-a543-8a1931b944e5', 'a178de0a-e3b9-489c-94c1-4dff23221186', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Thứ Bảy', '15:30 - 17:00', NULL, 'Ibanez Bass for synthwave groove', '2026-09-16 14:20:56.835264+00');


--
-- Data for Name: practice_sprints; Type: TABLE DATA; Schema: public; Owner: -
--

INSERT INTO practice_sprints VALUES ('2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6db79fa8-e627-428e-b801-984eed4235b9', 'Sprint 1: Harmonization & Dynamic Stage Lock', 'Lock rhythm section tempo, vocal harmony balance, and pass stage-ready QC audits.', '2026-10-01', '2026-10-08', true, '2026-09-16 14:20:55.335205+00');


--
-- Data for Name: member_sprint_availabilities; Type: TABLE DATA; Schema: public; Owner: -
--

INSERT INTO member_sprint_availabilities VALUES ('426d2c5a-7910-4e8d-a1e6-edc092b229be', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Monday', '18:00', true, '2026-09-16 14:20:56.841581+00');
INSERT INTO member_sprint_availabilities VALUES ('9a2047f2-0f56-4b28-b858-2cf88b2d8546', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Monday', '18:15', true, '2026-09-16 14:20:56.843919+00');
INSERT INTO member_sprint_availabilities VALUES ('11fb7c13-3d25-480f-b8f6-20c824eea5f0', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Monday', '18:30', true, '2026-09-16 14:20:56.846161+00');
INSERT INTO member_sprint_availabilities VALUES ('e943e162-1c79-4db4-8fd5-0e8c9aa668d7', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Monday', '18:45', true, '2026-09-16 14:20:56.848302+00');
INSERT INTO member_sprint_availabilities VALUES ('08b24fe5-4616-4757-966b-f405828c2287', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Monday', '19:00', true, '2026-09-16 14:20:56.850019+00');
INSERT INTO member_sprint_availabilities VALUES ('7451d47d-fb18-4952-a838-86fdd7a48168', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Monday', '19:15', true, '2026-09-16 14:20:56.851662+00');
INSERT INTO member_sprint_availabilities VALUES ('d9250ae8-c582-4658-8918-22aa6ca33272', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Monday', '19:30', true, '2026-09-16 14:20:56.853502+00');
INSERT INTO member_sprint_availabilities VALUES ('3eccee47-58b3-40bc-b6eb-d5e7145f9135', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Monday', '19:45', true, '2026-09-16 14:20:56.855504+00');
INSERT INTO member_sprint_availabilities VALUES ('19299e6d-21e3-44c2-b4f1-c6a397335140', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Monday', '20:00', true, '2026-09-16 14:20:56.857456+00');
INSERT INTO member_sprint_availabilities VALUES ('06270e26-1152-4913-af43-9cf7fae64901', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Monday', '20:15', true, '2026-09-16 14:20:56.859836+00');
INSERT INTO member_sprint_availabilities VALUES ('29466846-2623-4bf2-9acd-58cc12ce7deb', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Monday', '20:30', true, '2026-09-16 14:20:56.861979+00');
INSERT INTO member_sprint_availabilities VALUES ('a5afb512-c849-455a-9ecd-a7cb59b5447d', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Monday', '20:45', true, '2026-09-16 14:20:56.86416+00');
INSERT INTO member_sprint_availabilities VALUES ('5f1078c0-0a98-4233-b23b-01f43ea545c8', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Tuesday', '18:00', true, '2026-09-16 14:20:56.866765+00');
INSERT INTO member_sprint_availabilities VALUES ('b54e5c69-b78a-4ea4-9594-f6466790d1d7', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Tuesday', '18:15', true, '2026-09-16 14:20:56.869466+00');
INSERT INTO member_sprint_availabilities VALUES ('916e2136-82bf-4446-a5ce-93e05e75ba75', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Tuesday', '18:30', true, '2026-09-16 14:20:56.871889+00');
INSERT INTO member_sprint_availabilities VALUES ('6a0c8dfb-5c20-499e-8d61-22e8922c9596', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Tuesday', '18:45', true, '2026-09-16 14:20:56.874292+00');
INSERT INTO member_sprint_availabilities VALUES ('63e09b8d-058b-4521-852e-5c3b264c9fa8', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Tuesday', '19:00', true, '2026-09-16 14:20:56.876014+00');
INSERT INTO member_sprint_availabilities VALUES ('66072435-6fb5-4697-86f9-959658859742', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Tuesday', '19:15', true, '2026-09-16 14:20:56.877751+00');
INSERT INTO member_sprint_availabilities VALUES ('9b33723d-1fe9-486d-92d9-1312434e25aa', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Tuesday', '19:30', true, '2026-09-16 14:20:56.879598+00');
INSERT INTO member_sprint_availabilities VALUES ('623cdfa2-faf3-40c5-b0cb-f79fbaaf7728', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Tuesday', '19:45', true, '2026-09-16 14:20:56.8822+00');
INSERT INTO member_sprint_availabilities VALUES ('6760b584-fad2-4f30-970c-07d047537e7c', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Tuesday', '20:00', true, '2026-09-16 14:20:56.883938+00');
INSERT INTO member_sprint_availabilities VALUES ('d483bdb1-5a8c-4a8d-813d-80b2353a00a2', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Tuesday', '20:15', true, '2026-09-16 14:20:56.885623+00');
INSERT INTO member_sprint_availabilities VALUES ('21d57fe7-fb50-469d-a0b9-c552661e92c0', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Tuesday', '20:30', true, '2026-09-16 14:20:56.887285+00');
INSERT INTO member_sprint_availabilities VALUES ('d4e18ca6-6f7b-4556-9aaf-4d907e975f89', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Tuesday', '20:45', true, '2026-09-16 14:20:56.889304+00');
INSERT INTO member_sprint_availabilities VALUES ('418d7293-cb00-408a-8496-96721bc860af', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Wednesday', '18:00', true, '2026-09-16 14:20:56.891102+00');
INSERT INTO member_sprint_availabilities VALUES ('622e07aa-d04b-4819-8e01-d496169feef4', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Wednesday', '18:15', true, '2026-09-16 14:20:56.892974+00');
INSERT INTO member_sprint_availabilities VALUES ('5ba41161-d3d0-411c-880c-6e105ab4d364', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Wednesday', '18:30', true, '2026-09-16 14:20:56.894729+00');
INSERT INTO member_sprint_availabilities VALUES ('6a3d3d5d-e85c-4994-897b-b24033e3cac6', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Wednesday', '18:45', true, '2026-09-16 14:20:56.896646+00');
INSERT INTO member_sprint_availabilities VALUES ('fc8a8ca9-941f-4baf-9324-f328ae8a3689', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Wednesday', '19:00', true, '2026-09-16 14:20:56.898293+00');
INSERT INTO member_sprint_availabilities VALUES ('01524a36-6ba9-4f8d-81e8-23109b581b2b', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Wednesday', '19:15', true, '2026-09-16 14:20:56.899996+00');
INSERT INTO member_sprint_availabilities VALUES ('1e45f154-ca43-4ee0-93dc-cfc8927e1fa7', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Wednesday', '19:30', true, '2026-09-16 14:20:56.901749+00');
INSERT INTO member_sprint_availabilities VALUES ('5f8cfde7-8ede-4a13-8fe7-a038f46cf13f', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Wednesday', '19:45', true, '2026-09-16 14:20:56.904107+00');
INSERT INTO member_sprint_availabilities VALUES ('3377ea7c-9f80-4ea3-a8cb-bbce9208bb1f', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Wednesday', '20:00', true, '2026-09-16 14:20:56.905755+00');
INSERT INTO member_sprint_availabilities VALUES ('3d3a864c-e8e4-4d5a-a8d9-2ab79e9fbf14', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Wednesday', '20:15', true, '2026-09-16 14:20:56.907343+00');
INSERT INTO member_sprint_availabilities VALUES ('602cd103-1a1b-42bf-86a4-5aab2623dd3f', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Wednesday', '20:30', true, '2026-09-16 14:20:56.908932+00');
INSERT INTO member_sprint_availabilities VALUES ('babfb3e0-20b4-4645-a40a-fb3091f86fee', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Wednesday', '20:45', true, '2026-09-16 14:20:56.910686+00');
INSERT INTO member_sprint_availabilities VALUES ('8193a3a9-a72a-419b-802b-7c5041d2e5b0', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Thursday', '18:00', true, '2026-09-16 14:20:56.912334+00');
INSERT INTO member_sprint_availabilities VALUES ('1fbebe81-304b-42c0-a821-31cf2682f9f6', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Thursday', '18:15', true, '2026-09-16 14:20:56.914108+00');
INSERT INTO member_sprint_availabilities VALUES ('02f0ed09-03a1-4922-ac35-773406190ed4', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Thursday', '18:30', true, '2026-09-16 14:20:56.915863+00');
INSERT INTO member_sprint_availabilities VALUES ('367f9866-f3e1-4e0a-b2ab-a97efb74ec05', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Thursday', '18:45', true, '2026-09-16 14:20:56.917511+00');
INSERT INTO member_sprint_availabilities VALUES ('3afd3c64-76df-45b6-bcdf-617b5f74a52f', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Thursday', '19:00', true, '2026-09-16 14:20:56.919099+00');
INSERT INTO member_sprint_availabilities VALUES ('670b50b1-a2c0-4049-a1d4-6edecc65e30b', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Thursday', '19:15', true, '2026-09-16 14:20:56.920822+00');
INSERT INTO member_sprint_availabilities VALUES ('c00619e4-f3b0-4070-93b5-54d5013105cd', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Thursday', '19:30', true, '2026-09-16 14:20:56.922684+00');
INSERT INTO member_sprint_availabilities VALUES ('31b1226c-b1c8-4a1c-adc4-b0d204311726', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Thursday', '19:45', true, '2026-09-16 14:20:56.924565+00');
INSERT INTO member_sprint_availabilities VALUES ('989d641d-8b93-4c29-a6e2-77dad1a2d9e4', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Thursday', '20:00', true, '2026-09-16 14:20:56.926109+00');
INSERT INTO member_sprint_availabilities VALUES ('78d30d53-56d9-4f9e-9f13-899e9bb3db3e', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Thursday', '20:15', true, '2026-09-16 14:20:56.927707+00');
INSERT INTO member_sprint_availabilities VALUES ('ff511de0-1be3-4791-a2d2-d9215d6dedd5', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Thursday', '20:30', true, '2026-09-16 14:20:56.929714+00');
INSERT INTO member_sprint_availabilities VALUES ('7a675b90-5f47-469f-bdc7-0e96bdbc5ef5', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Thursday', '20:45', true, '2026-09-16 14:20:56.931561+00');
INSERT INTO member_sprint_availabilities VALUES ('9f5e7a96-f8d9-483c-9ccf-edc7fde94819', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Friday', '18:00', true, '2026-09-16 14:20:56.933168+00');
INSERT INTO member_sprint_availabilities VALUES ('ff958f8f-c082-40c2-b115-df2186993a7e', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Friday', '18:15', true, '2026-09-16 14:20:56.934828+00');
INSERT INTO member_sprint_availabilities VALUES ('d4eacccc-8d09-4faf-b681-491c2c88af29', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Friday', '18:30', true, '2026-09-16 14:20:56.937145+00');
INSERT INTO member_sprint_availabilities VALUES ('887faa5e-fc91-47d5-9f06-e4227c775819', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Friday', '18:45', true, '2026-09-16 14:20:56.939192+00');
INSERT INTO member_sprint_availabilities VALUES ('639c1236-e35e-4250-abee-ca406d386e9e', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Friday', '19:00', true, '2026-09-16 14:20:56.941918+00');
INSERT INTO member_sprint_availabilities VALUES ('7ec24439-7920-41f9-9e3f-bf82621d4050', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Friday', '19:15', true, '2026-09-16 14:20:56.944388+00');
INSERT INTO member_sprint_availabilities VALUES ('09503faf-25d8-451c-80a3-96fa35c6891c', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Friday', '19:30', true, '2026-09-16 14:20:56.946129+00');
INSERT INTO member_sprint_availabilities VALUES ('7e6297e4-e2c0-47b5-91cb-d2b1bea62c2e', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Friday', '19:45', true, '2026-09-16 14:20:56.947911+00');
INSERT INTO member_sprint_availabilities VALUES ('00391c94-4163-4454-a599-af853e85e4fa', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Friday', '20:00', true, '2026-09-16 14:20:56.949764+00');
INSERT INTO member_sprint_availabilities VALUES ('4a3a5e6b-ad2a-4517-8a13-0d59844a4c8b', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Friday', '20:15', true, '2026-09-16 14:20:56.951553+00');
INSERT INTO member_sprint_availabilities VALUES ('effa872b-8610-4d33-8938-0f67b763fded', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Friday', '20:30', true, '2026-09-16 14:20:56.953898+00');
INSERT INTO member_sprint_availabilities VALUES ('48bfb686-ae03-4a29-a6d3-5a0c14ffcce5', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Friday', '20:45', true, '2026-09-16 14:20:56.955917+00');
INSERT INTO member_sprint_availabilities VALUES ('85f68b8a-4b3b-4a36-9ae6-607e6cfb0958', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Saturday', '18:00', true, '2026-09-16 14:20:56.958773+00');
INSERT INTO member_sprint_availabilities VALUES ('8d3caab6-492a-4338-ad12-a031024da5c8', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Saturday', '18:15', true, '2026-09-16 14:20:56.961239+00');
INSERT INTO member_sprint_availabilities VALUES ('e7eb3a47-a784-4f10-a564-dcf285b461aa', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Saturday', '18:30', true, '2026-09-16 14:20:56.962997+00');
INSERT INTO member_sprint_availabilities VALUES ('e7ae1e0f-1c33-4073-a849-ddae97bf32dd', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Saturday', '18:45', true, '2026-09-16 14:20:56.964706+00');
INSERT INTO member_sprint_availabilities VALUES ('9138c005-6e75-40d5-9169-6d902303677d', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Saturday', '19:00', true, '2026-09-16 14:20:56.966639+00');
INSERT INTO member_sprint_availabilities VALUES ('84a4dd68-d5ee-4cd0-9e00-94bac7c75c4b', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Saturday', '19:15', true, '2026-09-16 14:20:56.968433+00');
INSERT INTO member_sprint_availabilities VALUES ('9ffc22de-b802-43f2-b47e-16720f8cf62f', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Saturday', '19:30', true, '2026-09-16 14:20:56.970063+00');
INSERT INTO member_sprint_availabilities VALUES ('6cc59822-5084-4219-aa01-b3d7b0eb32d4', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Saturday', '19:45', true, '2026-09-16 14:20:56.971774+00');
INSERT INTO member_sprint_availabilities VALUES ('7428fb21-1ef8-467e-b4db-5bb8609e1138', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Saturday', '20:00', true, '2026-09-16 14:20:56.973495+00');
INSERT INTO member_sprint_availabilities VALUES ('352ffa5b-275f-4b31-a92e-9253808dafa8', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Saturday', '20:15', true, '2026-09-16 14:20:56.975123+00');
INSERT INTO member_sprint_availabilities VALUES ('d63d076e-5035-4f0a-9d62-175907eafe0f', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Saturday', '20:30', true, '2026-09-16 14:20:56.976742+00');
INSERT INTO member_sprint_availabilities VALUES ('2c4a263f-cf1a-451e-9019-195e83d7f168', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Saturday', '20:45', true, '2026-09-16 14:20:56.979103+00');
INSERT INTO member_sprint_availabilities VALUES ('38cec534-356e-4dd1-8f81-a1b6ecbcd4eb', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Sunday', '18:00', true, '2026-09-16 14:20:56.981168+00');
INSERT INTO member_sprint_availabilities VALUES ('eebe9ab4-6ad9-406e-9c6e-427ba951994b', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Sunday', '18:15', true, '2026-09-16 14:20:56.982991+00');
INSERT INTO member_sprint_availabilities VALUES ('81aef07a-ddd7-49cc-a709-dab4cc0e65a7', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Sunday', '18:30', true, '2026-09-16 14:20:56.98467+00');
INSERT INTO member_sprint_availabilities VALUES ('9b0d1132-0e59-4e0c-abb6-6634e45420a8', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Sunday', '18:45', true, '2026-09-16 14:20:56.986271+00');
INSERT INTO member_sprint_availabilities VALUES ('073b0547-56a2-407c-90d8-7c49ddc0f23f', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Sunday', '19:00', true, '2026-09-16 14:20:56.987842+00');
INSERT INTO member_sprint_availabilities VALUES ('efe39092-b2e2-4dc3-b873-3b231df79e7a', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Sunday', '19:15', true, '2026-09-16 14:20:56.989402+00');
INSERT INTO member_sprint_availabilities VALUES ('ae8697a2-ad73-47a7-ab08-09d807a82542', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Sunday', '19:30', true, '2026-09-16 14:20:56.991029+00');
INSERT INTO member_sprint_availabilities VALUES ('c3472230-3ab1-442a-ba1f-8d061ea35b7d', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Sunday', '19:45', true, '2026-09-16 14:20:56.992672+00');
INSERT INTO member_sprint_availabilities VALUES ('c735ba9a-dbf5-4887-8632-25b5b145fd23', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Sunday', '20:00', true, '2026-09-16 14:20:56.994397+00');
INSERT INTO member_sprint_availabilities VALUES ('fdc164aa-3f3b-47cd-87a9-0ef95357c86e', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Sunday', '20:45', true, '2026-09-16 14:20:57.000401+00');
INSERT INTO member_sprint_availabilities VALUES ('99a35f67-d07d-49ee-800f-3629ca5dcee0', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Sunday', '20:15', true, '2026-09-16 14:20:56.99621+00');
INSERT INTO member_sprint_availabilities VALUES ('89830648-9499-4497-b4ec-523333cb3c79', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Sunday', '20:30', true, '2026-09-16 14:20:56.997975+00');


--
-- Data for Name: member_sprint_availabilities_history; Type: TABLE DATA; Schema: public; Owner: -
--



--
-- Data for Name: music_number_members; Type: TABLE DATA; Schema: public; Owner: -
--

INSERT INTO music_number_members VALUES ('ce1f3cc7-36d2-44f0-8448-f0c00d78ee70', 'a6794d91-e226-4e94-9acc-103cccfed2f1', '43b8694f-bfb7-401d-8ad1-19c7c0755362', 'Lead Vocal', true, '2026-09-16 14:20:55.439661+00');
INSERT INTO music_number_members VALUES ('7c4914d1-270f-4471-868c-17cae864547b', 'a6794d91-e226-4e94-9acc-103cccfed2f1', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Lead Guitar', false, '2026-09-16 14:20:55.444399+00');
INSERT INTO music_number_members VALUES ('87f20c1f-a343-4d45-9856-08558b1e23ee', 'a6794d91-e226-4e94-9acc-103cccfed2f1', '3260e15d-534c-4542-a26d-216122cb3be6', 'Bass Guitar', false, '2026-09-16 14:20:55.446457+00');
INSERT INTO music_number_members VALUES ('4b495b9b-8ef9-4720-977a-73f05318f9e1', 'a6794d91-e226-4e94-9acc-103cccfed2f1', 'ec4e11fa-4758-4234-aaa6-36f9a4b60d8a', 'Drums', false, '2026-09-16 14:20:55.448503+00');
INSERT INTO music_number_members VALUES ('9784ffaa-01f4-4033-ad2c-c87d4b77663f', 'a6794d91-e226-4e94-9acc-103cccfed2f1', '25ec4a6a-687a-4cad-853a-9b8c4299a6f4', 'Keyboards & Piano', false, '2026-09-16 14:20:55.450245+00');
INSERT INTO music_number_members VALUES ('f453cb53-a2a3-47f0-8a6d-2bcd01621699', 'efa1dcb5-3209-4c6e-bfc8-8a6b76828f82', 'da08a51a-837f-416a-9f72-c417eb79f2f8', 'Lead Vocal', true, '2026-09-16 14:20:55.510115+00');
INSERT INTO music_number_members VALUES ('2cd00b39-e360-417c-a5a5-04f34f54cf08', 'efa1dcb5-3209-4c6e-bfc8-8a6b76828f82', '85ef1521-bb30-4116-a0e0-f8bd8ab87705', 'Lead Guitar', false, '2026-09-16 14:20:55.515095+00');
INSERT INTO music_number_members VALUES ('48d52323-2f07-41f7-b11a-2914e67c046b', 'efa1dcb5-3209-4c6e-bfc8-8a6b76828f82', '1527b949-09f0-46a3-9b43-5c82d7ef5eaf', 'Bass Guitar', false, '2026-09-16 14:20:55.517267+00');
INSERT INTO music_number_members VALUES ('f8ae5a68-3ee6-43b9-b98f-3468fe04aa34', 'efa1dcb5-3209-4c6e-bfc8-8a6b76828f82', 'e6d141cc-c6fb-4097-9561-f8b780d1b532', 'Drums', false, '2026-09-16 14:20:55.51933+00');
INSERT INTO music_number_members VALUES ('c8ef86b2-c0d8-4266-848c-d09c8de78e94', 'efa1dcb5-3209-4c6e-bfc8-8a6b76828f82', '5c34609d-0cd0-4342-be02-3b42ba182a0d', 'Keyboards & Piano', false, '2026-09-16 14:20:55.521268+00');
INSERT INTO music_number_members VALUES ('a7b82544-2b95-4235-8dcd-1fa93396a49b', 'fd1e4824-44b6-4097-be41-7d5fa2277e2b', 'b17b54ab-de0a-4f99-badd-b7428ddb2f07', 'Lead Vocal', true, '2026-09-16 14:20:55.581289+00');
INSERT INTO music_number_members VALUES ('a8fa083b-27f5-469b-a509-7eae3d077fc8', 'fd1e4824-44b6-4097-be41-7d5fa2277e2b', '4de34466-4e15-4bee-be27-8fafcc8b48db', 'Lead Guitar', false, '2026-09-16 14:20:55.585712+00');
INSERT INTO music_number_members VALUES ('ab03a8a6-5dd1-4973-91db-114542398a87', 'fd1e4824-44b6-4097-be41-7d5fa2277e2b', '3260e15d-534c-4542-a26d-216122cb3be6', 'Bass Guitar', false, '2026-09-16 14:20:55.587538+00');
INSERT INTO music_number_members VALUES ('6efc3e41-7f21-487c-85e3-54811120d3a2', 'fd1e4824-44b6-4097-be41-7d5fa2277e2b', '85ef1521-bb30-4116-a0e0-f8bd8ab87705', 'Drums', false, '2026-09-16 14:20:55.589596+00');
INSERT INTO music_number_members VALUES ('65e319ad-0102-4a7d-a4bd-baf3e9506ce3', 'fd1e4824-44b6-4097-be41-7d5fa2277e2b', 'adb9581e-098b-44c4-b923-a8a338ff3f01', 'Keyboards & Piano', false, '2026-09-16 14:20:55.591478+00');
INSERT INTO music_number_members VALUES ('e8fd74c2-691a-4a27-9ab3-16eed36c8d10', 'df265763-3fe7-4e33-b91f-32144cbeff8d', 'b31d8477-a95a-4608-b152-b307cc1973dc', 'Lead Vocal', true, '2026-09-16 14:20:55.651614+00');
INSERT INTO music_number_members VALUES ('09ccf0f6-2125-4e2c-a088-f3407d2a0fdc', 'df265763-3fe7-4e33-b91f-32144cbeff8d', '43b8694f-bfb7-401d-8ad1-19c7c0755362', 'Lead Guitar', false, '2026-09-16 14:20:55.656042+00');
INSERT INTO music_number_members VALUES ('120c7a04-a39b-4fe3-bc30-23342d68cf45', 'df265763-3fe7-4e33-b91f-32144cbeff8d', '9f5c5533-0ea4-4426-ac25-808a363459aa', 'Bass Guitar', false, '2026-09-16 14:20:55.657902+00');
INSERT INTO music_number_members VALUES ('16023763-2f7e-4bed-9db4-cd0b5787b92b', 'df265763-3fe7-4e33-b91f-32144cbeff8d', '51618637-3131-428c-a8ee-63789cb8b505', 'Drums', false, '2026-09-16 14:20:55.659853+00');
INSERT INTO music_number_members VALUES ('8db8b70e-c98e-4b5b-8801-afcc48e9cd70', 'df265763-3fe7-4e33-b91f-32144cbeff8d', '995ce344-2633-49cf-8bec-e2c5df129f8d', 'Keyboards & Piano', false, '2026-09-16 14:20:55.662127+00');
INSERT INTO music_number_members VALUES ('54090962-9424-4d64-961c-263bed4d954e', '001ab9ed-e397-439d-8815-677c7268530a', '2b95ecc6-b8b1-4262-942e-28b181c0750a', 'Lead Vocal', true, '2026-09-16 14:20:55.720205+00');
INSERT INTO music_number_members VALUES ('fb3d80be-92cd-4f1d-b18c-fa735ccf5f99', '001ab9ed-e397-439d-8815-677c7268530a', '669615d4-241b-47ee-b8ad-a93b6c99d865', 'Lead Guitar', false, '2026-09-16 14:20:55.724573+00');
INSERT INTO music_number_members VALUES ('78442c4e-e956-4e3c-b816-6622c759f152', '001ab9ed-e397-439d-8815-677c7268530a', '40c323ef-f8db-412e-bb3a-532ed3ad4080', 'Bass Guitar', false, '2026-09-16 14:20:55.726305+00');
INSERT INTO music_number_members VALUES ('bbc5a443-ff63-47e5-853a-37f784f97858', '001ab9ed-e397-439d-8815-677c7268530a', 'e6d141cc-c6fb-4097-9561-f8b780d1b532', 'Drums', false, '2026-09-16 14:20:55.728059+00');
INSERT INTO music_number_members VALUES ('307fe3f4-f7b8-442f-a51a-a363acb70005', '001ab9ed-e397-439d-8815-677c7268530a', '25ec4a6a-687a-4cad-853a-9b8c4299a6f4', 'Keyboards & Piano', false, '2026-09-16 14:20:55.729792+00');
INSERT INTO music_number_members VALUES ('62fa2bfe-d6b1-47a3-8e58-0ab2c283f7e0', 'fae72639-4145-45e0-8a8f-7b7c8e1368e8', '65182525-4f89-404d-b247-2ebcd90550db', 'Lead Vocal', true, '2026-09-16 14:20:55.792044+00');
INSERT INTO music_number_members VALUES ('79b2a0c3-6298-4625-8be9-18fb3a87f401', 'fae72639-4145-45e0-8a8f-7b7c8e1368e8', 'dc8e21ef-e8b6-43aa-9dbe-e8743a01cadf', 'Lead Guitar', false, '2026-09-16 14:20:55.800104+00');
INSERT INTO music_number_members VALUES ('ea8cb59a-412f-4b19-8b94-cc3082643a92', 'fae72639-4145-45e0-8a8f-7b7c8e1368e8', '1527b949-09f0-46a3-9b43-5c82d7ef5eaf', 'Bass Guitar', false, '2026-09-16 14:20:55.802869+00');
INSERT INTO music_number_members VALUES ('f7d9eb9a-0be8-4cb9-b053-2ece4db35401', 'fae72639-4145-45e0-8a8f-7b7c8e1368e8', 'f71e102a-83f6-42e7-b045-838f7c735920', 'Drums', false, '2026-09-16 14:20:55.804827+00');
INSERT INTO music_number_members VALUES ('dd453239-7e66-45b6-a29a-d64d22c14ee1', 'fae72639-4145-45e0-8a8f-7b7c8e1368e8', 'fc1839d0-6f0f-46d0-8c9c-76ac291c1b63', 'Keyboards & Piano', false, '2026-09-16 14:20:55.806734+00');
INSERT INTO music_number_members VALUES ('4023d4b7-ea2d-4bbf-ad1f-707c0220a16d', 'bf05133e-6c6c-4abe-ba87-65526ca7175f', 'e87c1e6c-1434-46a1-ba16-42954a5f85ab', 'Lead Vocal', true, '2026-09-16 14:20:55.844748+00');
INSERT INTO music_number_members VALUES ('6f2cb88d-58bd-40fe-afff-6104bf89fe1e', 'bf05133e-6c6c-4abe-ba87-65526ca7175f', 'dc8e21ef-e8b6-43aa-9dbe-e8743a01cadf', 'Lead Guitar', false, '2026-09-16 14:20:55.849068+00');
INSERT INTO music_number_members VALUES ('3aacaff1-2fed-43f3-b16a-ebcbab62c442', 'bf05133e-6c6c-4abe-ba87-65526ca7175f', '27b4e667-b160-4817-a947-3098853227f3', 'Bass Guitar', false, '2026-09-16 14:20:55.85086+00');
INSERT INTO music_number_members VALUES ('146abed0-91e6-4248-8b3b-06c44af192cf', 'bf05133e-6c6c-4abe-ba87-65526ca7175f', '3946fdd0-f837-448c-b47c-ffd299a38f2c', 'Drums', false, '2026-09-16 14:20:55.85268+00');
INSERT INTO music_number_members VALUES ('569fb2bc-521a-458b-b0cd-2db234fa575c', 'bf05133e-6c6c-4abe-ba87-65526ca7175f', '3a65769b-1fcf-4c23-b4c1-a0d9ee634d58', 'Keyboards & Piano', false, '2026-09-16 14:20:55.855124+00');
INSERT INTO music_number_members VALUES ('e61e7027-9cd1-4788-b3ca-beb3060e2c09', '088720cc-c3f3-4561-a7c1-b9194a953bb7', '583899c4-6636-429f-825e-46d0674f2ea7', 'Lead Vocal', true, '2026-09-16 14:20:55.894655+00');
INSERT INTO music_number_members VALUES ('550b93f3-50e6-453e-9d16-ce9e0ca622c7', '088720cc-c3f3-4561-a7c1-b9194a953bb7', '9e178c56-4003-409f-bef9-8a04a8f2d5de', 'Lead Guitar', false, '2026-09-16 14:20:55.899478+00');
INSERT INTO music_number_members VALUES ('a651267d-e407-494f-bfbd-6f63256cbd09', '088720cc-c3f3-4561-a7c1-b9194a953bb7', '9f5c5533-0ea4-4426-ac25-808a363459aa', 'Bass Guitar', false, '2026-09-16 14:20:55.901173+00');
INSERT INTO music_number_members VALUES ('59a6e0a3-bb6c-492e-8aef-c22317f16dac', '088720cc-c3f3-4561-a7c1-b9194a953bb7', 'ec4e11fa-4758-4234-aaa6-36f9a4b60d8a', 'Drums', false, '2026-09-16 14:20:55.903105+00');
INSERT INTO music_number_members VALUES ('a8f78d42-4cb4-4928-928d-68319acf1b13', '088720cc-c3f3-4561-a7c1-b9194a953bb7', 'adb9581e-098b-44c4-b923-a8a338ff3f01', 'Keyboards & Piano', false, '2026-09-16 14:20:55.905204+00');
INSERT INTO music_number_members VALUES ('153e1da3-b937-4365-9cc0-28b3e5b9fcdf', 'a178de0a-e3b9-489c-94c1-4dff23221186', 'eaf3aec1-7c19-4021-badc-bf816a278aaf', 'Lead Vocal', true, '2026-09-16 14:20:55.965794+00');
INSERT INTO music_number_members VALUES ('cc0dc12a-c630-4fe2-b203-db7cb27d4a05', 'a178de0a-e3b9-489c-94c1-4dff23221186', '4de34466-4e15-4bee-be27-8fafcc8b48db', 'Lead Guitar', false, '2026-09-16 14:20:55.970192+00');
INSERT INTO music_number_members VALUES ('5ada5409-69dd-4787-8357-5f3c7e762e64', 'a178de0a-e3b9-489c-94c1-4dff23221186', '8a46ea77-ab09-4da0-9ee8-1f392154766a', 'Bass Guitar', false, '2026-09-16 14:20:55.971986+00');
INSERT INTO music_number_members VALUES ('40957741-ed79-4a66-9af8-fe631dbd7509', 'a178de0a-e3b9-489c-94c1-4dff23221186', '85ef1521-bb30-4116-a0e0-f8bd8ab87705', 'Drums', false, '2026-09-16 14:20:55.973768+00');
INSERT INTO music_number_members VALUES ('b58147ba-a6b9-46fd-92b2-6e0e3cc46563', 'a178de0a-e3b9-489c-94c1-4dff23221186', '5c34609d-0cd0-4342-be02-3b42ba182a0d', 'Keyboards & Piano', false, '2026-09-16 14:20:55.975488+00');
INSERT INTO music_number_members VALUES ('c01d979b-8a9d-45f8-a50d-19f5af759a0c', '98e02096-399e-41e0-9fba-9cc207b08f66', '43210b26-d1f3-4891-b1aa-e0abedd6fdb1', 'Lead Vocal', true, '2026-09-16 14:20:56.033287+00');
INSERT INTO music_number_members VALUES ('dd38ce7e-6d47-49e9-9a91-69ae00b72609', '98e02096-399e-41e0-9fba-9cc207b08f66', '9ef6d8df-177a-46ad-841a-20c1835a88d1', 'Lead Guitar', false, '2026-09-16 14:20:56.038403+00');
INSERT INTO music_number_members VALUES ('f23e8e7b-1a82-4b73-b546-e8097142fed4', '98e02096-399e-41e0-9fba-9cc207b08f66', '40c323ef-f8db-412e-bb3a-532ed3ad4080', 'Bass Guitar', false, '2026-09-16 14:20:56.041131+00');
INSERT INTO music_number_members VALUES ('7e57e864-213b-4695-ac0b-8d28d49a536a', '98e02096-399e-41e0-9fba-9cc207b08f66', '51618637-3131-428c-a8ee-63789cb8b505', 'Drums', false, '2026-09-16 14:20:56.043681+00');
INSERT INTO music_number_members VALUES ('ade54846-9b85-47d7-b255-d2c5f1d40a56', '98e02096-399e-41e0-9fba-9cc207b08f66', '7bb6e2d4-7e59-4f4e-8ff3-9a22305cb2af', 'Keyboards & Piano', false, '2026-09-16 14:20:56.045563+00');
INSERT INTO music_number_members VALUES ('fefec4c0-3689-4a33-9a22-2a4314f6f057', '1503e433-f90c-49a1-b328-90214f0d1c41', '04fbe6ca-a911-4ec0-bbbc-91b6aebd1b9b', 'Lead Vocal', true, '2026-09-16 14:20:56.082431+00');
INSERT INTO music_number_members VALUES ('d636ee71-0718-41e8-9c15-19b53f486b9b', '1503e433-f90c-49a1-b328-90214f0d1c41', 'cec5e868-9227-4958-a802-6cf39477451d', 'Lead Guitar', false, '2026-09-16 14:20:56.086832+00');
INSERT INTO music_number_members VALUES ('f7455734-941c-4d4d-b786-3e45d46869de', '1503e433-f90c-49a1-b328-90214f0d1c41', '1527b949-09f0-46a3-9b43-5c82d7ef5eaf', 'Bass Guitar', false, '2026-09-16 14:20:56.089249+00');
INSERT INTO music_number_members VALUES ('60d8e769-0627-4f32-af86-41ffe15dd227', '1503e433-f90c-49a1-b328-90214f0d1c41', '45bdabb9-f105-4bbd-bc74-32ac293baf01', 'Drums', false, '2026-09-16 14:20:56.09109+00');
INSERT INTO music_number_members VALUES ('c27935eb-7f5a-4b20-8be1-4b0af8e37a45', '1503e433-f90c-49a1-b328-90214f0d1c41', '25ec4a6a-687a-4cad-853a-9b8c4299a6f4', 'Keyboards & Piano', false, '2026-09-16 14:20:56.093641+00');
INSERT INTO music_number_members VALUES ('9d63dd00-7567-4ec7-9e35-9c4f9e25cd7b', '53f72023-d86e-464e-beaa-bc1ba3438de0', '0679280e-5d18-4960-86da-e33584e8f601', 'Lead Vocal', true, '2026-09-16 14:20:56.153305+00');
INSERT INTO music_number_members VALUES ('97df21af-5d61-4300-a154-1b058bfda23f', '53f72023-d86e-464e-beaa-bc1ba3438de0', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Lead Guitar', false, '2026-09-16 14:20:56.158632+00');
INSERT INTO music_number_members VALUES ('682ab038-6ba1-4a68-a113-d7d4f6517f8e', '53f72023-d86e-464e-beaa-bc1ba3438de0', '3260e15d-534c-4542-a26d-216122cb3be6', 'Bass Guitar', false, '2026-09-16 14:20:56.160574+00');
INSERT INTO music_number_members VALUES ('0e8a08c6-e054-4e96-87d4-3302077e8eeb', '53f72023-d86e-464e-beaa-bc1ba3438de0', 'f71e102a-83f6-42e7-b045-838f7c735920', 'Drums', false, '2026-09-16 14:20:56.162572+00');
INSERT INTO music_number_members VALUES ('0bf1faa2-290d-4deb-a99c-fd26664b0cb5', '53f72023-d86e-464e-beaa-bc1ba3438de0', 'a13911a7-cdfa-4575-a82a-338723910638', 'Keyboards & Piano', false, '2026-09-16 14:20:56.164466+00');
INSERT INTO music_number_members VALUES ('374e54e5-8ad0-4278-a102-6033001535be', '47362964-5de3-40f2-9ef6-9792bd422c3d', '43b8694f-bfb7-401d-8ad1-19c7c0755362', 'Lead Vocal', true, '2026-09-16 14:20:56.201332+00');
INSERT INTO music_number_members VALUES ('0fe635d2-7841-43c3-a0db-230e057b517e', '47362964-5de3-40f2-9ef6-9792bd422c3d', 'dc8e21ef-e8b6-43aa-9dbe-e8743a01cadf', 'Lead Guitar', false, '2026-09-16 14:20:56.20569+00');
INSERT INTO music_number_members VALUES ('638da5ef-8fce-4aa5-a4a3-63ba3b2a4d09', '47362964-5de3-40f2-9ef6-9792bd422c3d', '8a46ea77-ab09-4da0-9ee8-1f392154766a', 'Bass Guitar', false, '2026-09-16 14:20:56.207473+00');
INSERT INTO music_number_members VALUES ('c5962a83-77fb-4891-93a1-bb0b326a5419', '47362964-5de3-40f2-9ef6-9792bd422c3d', 'ec4e11fa-4758-4234-aaa6-36f9a4b60d8a', 'Drums', false, '2026-09-16 14:20:56.209195+00');
INSERT INTO music_number_members VALUES ('96ed3669-8da0-4ac6-a0d7-edc7d467444d', '47362964-5de3-40f2-9ef6-9792bd422c3d', 'adb9581e-098b-44c4-b923-a8a338ff3f01', 'Keyboards & Piano', false, '2026-09-16 14:20:56.211063+00');
INSERT INTO music_number_members VALUES ('dac024fa-96a9-4eae-9390-446e34fb556b', '7e34be8b-4c6a-4611-ab88-bd2ae1337a03', 'da08a51a-837f-416a-9f72-c417eb79f2f8', 'Lead Vocal', true, '2026-09-16 14:20:56.272026+00');
INSERT INTO music_number_members VALUES ('3b279de9-426c-42ae-bac2-705c43c7a8f4', '7e34be8b-4c6a-4611-ab88-bd2ae1337a03', '669615d4-241b-47ee-b8ad-a93b6c99d865', 'Lead Guitar', false, '2026-09-16 14:20:56.27698+00');
INSERT INTO music_number_members VALUES ('56f7d46d-7cb1-464f-9ad1-4c808e4c4166', '7e34be8b-4c6a-4611-ab88-bd2ae1337a03', '27b4e667-b160-4817-a947-3098853227f3', 'Bass Guitar', false, '2026-09-16 14:20:56.278958+00');
INSERT INTO music_number_members VALUES ('fff1e441-5179-4630-b9a2-95598334ce6b', '7e34be8b-4c6a-4611-ab88-bd2ae1337a03', '85ef1521-bb30-4116-a0e0-f8bd8ab87705', 'Drums', false, '2026-09-16 14:20:56.280994+00');
INSERT INTO music_number_members VALUES ('06e3b967-b726-4b4d-9661-72f1aa8d9f68', '7e34be8b-4c6a-4611-ab88-bd2ae1337a03', '995ce344-2633-49cf-8bec-e2c5df129f8d', 'Keyboards & Piano', false, '2026-09-16 14:20:56.283122+00');
INSERT INTO music_number_members VALUES ('7dd8debc-4f49-4f57-9e5f-c82d218a4f64', '0fc68ab8-0ebf-46e8-9dfb-da9ce47ba705', 'b17b54ab-de0a-4f99-badd-b7428ddb2f07', 'Lead Vocal', true, '2026-09-16 14:20:56.34671+00');
INSERT INTO music_number_members VALUES ('744105cb-22c4-4722-974b-adcd21c143d5', '0fc68ab8-0ebf-46e8-9dfb-da9ce47ba705', '9e178c56-4003-409f-bef9-8a04a8f2d5de', 'Lead Guitar', false, '2026-09-16 14:20:56.351174+00');
INSERT INTO music_number_members VALUES ('6c1d9609-a729-4a2a-8c29-f2ed376b0ac0', '0fc68ab8-0ebf-46e8-9dfb-da9ce47ba705', '9f5c5533-0ea4-4426-ac25-808a363459aa', 'Bass Guitar', false, '2026-09-16 14:20:56.352898+00');
INSERT INTO music_number_members VALUES ('a497a088-68ff-4247-8f65-840a50b302e6', '0fc68ab8-0ebf-46e8-9dfb-da9ce47ba705', '3946fdd0-f837-448c-b47c-ffd299a38f2c', 'Drums', false, '2026-09-16 14:20:56.354702+00');
INSERT INTO music_number_members VALUES ('11d63699-92e6-45e2-a6a7-93b06810fc62', '0fc68ab8-0ebf-46e8-9dfb-da9ce47ba705', '3a65769b-1fcf-4c23-b4c1-a0d9ee634d58', 'Keyboards & Piano', false, '2026-09-16 14:20:56.356607+00');
INSERT INTO music_number_members VALUES ('4de2ec78-5615-42cc-91e5-da7401700ba4', 'b20ee408-0084-4eb6-a189-e26d02396119', 'b31d8477-a95a-4608-b152-b307cc1973dc', 'Lead Vocal', true, '2026-09-16 14:20:56.394346+00');
INSERT INTO music_number_members VALUES ('073a3cd9-12e6-4fbb-a314-4b8a0f1f3035', 'b20ee408-0084-4eb6-a189-e26d02396119', '4de34466-4e15-4bee-be27-8fafcc8b48db', 'Lead Guitar', false, '2026-09-16 14:20:56.39882+00');
INSERT INTO music_number_members VALUES ('1d6ac0ce-298e-4c9f-80f6-931f9630c0d5', 'b20ee408-0084-4eb6-a189-e26d02396119', '3260e15d-534c-4542-a26d-216122cb3be6', 'Bass Guitar', false, '2026-09-16 14:20:56.400656+00');
INSERT INTO music_number_members VALUES ('17e189a4-9263-450b-a79e-d7483661cde1', 'b20ee408-0084-4eb6-a189-e26d02396119', '51618637-3131-428c-a8ee-63789cb8b505', 'Drums', false, '2026-09-16 14:20:56.403175+00');
INSERT INTO music_number_members VALUES ('556ff558-0d9e-4344-87ab-1beeaa308c64', 'b20ee408-0084-4eb6-a189-e26d02396119', 'fc1839d0-6f0f-46d0-8c9c-76ac291c1b63', 'Keyboards & Piano', false, '2026-09-16 14:20:56.405164+00');
INSERT INTO music_number_members VALUES ('c3fcba86-3ac7-41a6-92d7-84faee7e86ec', 'cb26d111-ee77-45cb-9fcf-342b48f950f4', '2b95ecc6-b8b1-4262-942e-28b181c0750a', 'Lead Vocal', true, '2026-09-16 14:20:56.462499+00');
INSERT INTO music_number_members VALUES ('e87f11c1-c5db-4ab8-8734-b085fa7ecd36', 'cb26d111-ee77-45cb-9fcf-342b48f950f4', 'cec5e868-9227-4958-a802-6cf39477451d', 'Lead Guitar', false, '2026-09-16 14:20:56.467226+00');
INSERT INTO music_number_members VALUES ('b0422753-6fa8-4a92-8cba-b7ee7c4b7577', 'cb26d111-ee77-45cb-9fcf-342b48f950f4', '40c323ef-f8db-412e-bb3a-532ed3ad4080', 'Bass Guitar', false, '2026-09-16 14:20:56.469422+00');
INSERT INTO music_number_members VALUES ('b8359455-f275-4d8f-baa9-8a257756aee5', 'cb26d111-ee77-45cb-9fcf-342b48f950f4', '45bdabb9-f105-4bbd-bc74-32ac293baf01', 'Drums', false, '2026-09-16 14:20:56.471307+00');
INSERT INTO music_number_members VALUES ('125df73d-b362-4b73-bf5d-2db4414f8970', 'cb26d111-ee77-45cb-9fcf-342b48f950f4', 'a13911a7-cdfa-4575-a82a-338723910638', 'Keyboards & Piano', false, '2026-09-16 14:20:56.473212+00');
INSERT INTO music_number_members VALUES ('5624a45a-cdd0-4f1d-908e-785bcb769b64', 'c630f95a-5bd6-4b34-a010-cc4d6025f1f7', '65182525-4f89-404d-b247-2ebcd90550db', 'Lead Vocal', true, '2026-09-16 14:20:56.515022+00');
INSERT INTO music_number_members VALUES ('54740acf-caeb-48c3-b195-fff21237d374', 'c630f95a-5bd6-4b34-a010-cc4d6025f1f7', '9ef6d8df-177a-46ad-841a-20c1835a88d1', 'Lead Guitar', false, '2026-09-16 14:20:56.519697+00');
INSERT INTO music_number_members VALUES ('f1d391b9-49bf-41f3-aed5-78877589155e', 'c630f95a-5bd6-4b34-a010-cc4d6025f1f7', '8a46ea77-ab09-4da0-9ee8-1f392154766a', 'Bass Guitar', false, '2026-09-16 14:20:56.521858+00');
INSERT INTO music_number_members VALUES ('41979084-57eb-4af7-a2cb-bf7be5fe4567', 'c630f95a-5bd6-4b34-a010-cc4d6025f1f7', 'f71e102a-83f6-42e7-b045-838f7c735920', 'Drums', false, '2026-09-16 14:20:56.523637+00');
INSERT INTO music_number_members VALUES ('a12b017d-3e5d-4ba5-9596-209b8cb42d6f', 'c630f95a-5bd6-4b34-a010-cc4d6025f1f7', '7bb6e2d4-7e59-4f4e-8ff3-9a22305cb2af', 'Keyboards & Piano', false, '2026-09-16 14:20:56.525356+00');
INSERT INTO music_number_members VALUES ('37a50111-bfbb-48d1-872a-e050f431be47', 'da717054-7b5d-4c98-9287-1d1d9e1c7cf8', '04fbe6ca-a911-4ec0-bbbc-91b6aebd1b9b', 'Lead Vocal', true, '2026-09-16 14:20:56.562075+00');
INSERT INTO music_number_members VALUES ('ef8a09ee-1124-4c4b-8738-9881bd4b7e15', 'da717054-7b5d-4c98-9287-1d1d9e1c7cf8', '6732646e-10e0-4f22-99f1-7b553e0570df', 'Lead Guitar', false, '2026-09-16 14:20:56.567527+00');
INSERT INTO music_number_members VALUES ('1d2550d8-923f-469a-9855-d0cb5e6392c7', 'da717054-7b5d-4c98-9287-1d1d9e1c7cf8', '1527b949-09f0-46a3-9b43-5c82d7ef5eaf', 'Bass Guitar', false, '2026-09-16 14:20:56.569741+00');
INSERT INTO music_number_members VALUES ('3543a1a6-0928-4f9e-b8de-96363bafde80', 'da717054-7b5d-4c98-9287-1d1d9e1c7cf8', '85ef1521-bb30-4116-a0e0-f8bd8ab87705', 'Drums', false, '2026-09-16 14:20:56.572073+00');
INSERT INTO music_number_members VALUES ('d0fe55e4-85cc-4512-9c81-f2701d631715', 'da717054-7b5d-4c98-9287-1d1d9e1c7cf8', '25ec4a6a-687a-4cad-853a-9b8c4299a6f4', 'Keyboards & Piano', false, '2026-09-16 14:20:56.574019+00');
INSERT INTO music_number_members VALUES ('c8bfaac2-c46c-4dda-b87f-929a8653c72c', '31703493-1c88-458e-87d5-ffb1d4036d65', '0679280e-5d18-4960-86da-e33584e8f601', 'Lead Vocal', true, '2026-09-16 14:20:56.636531+00');
INSERT INTO music_number_members VALUES ('2c6b45f3-ac81-4788-966b-9548917886d2', '31703493-1c88-458e-87d5-ffb1d4036d65', '669615d4-241b-47ee-b8ad-a93b6c99d865', 'Lead Guitar', false, '2026-09-16 14:20:56.641982+00');
INSERT INTO music_number_members VALUES ('871f8075-c13b-41a8-ad00-b85cec426a78', '31703493-1c88-458e-87d5-ffb1d4036d65', '27b4e667-b160-4817-a947-3098853227f3', 'Bass Guitar', false, '2026-09-16 14:20:56.644238+00');
INSERT INTO music_number_members VALUES ('1ec408c0-0d9c-47cf-b509-4e00ac8b1b3d', '31703493-1c88-458e-87d5-ffb1d4036d65', 'e6d141cc-c6fb-4097-9561-f8b780d1b532', 'Drums', false, '2026-09-16 14:20:56.646416+00');
INSERT INTO music_number_members VALUES ('703a94b2-71bb-4069-98f3-6bc5780fee5f', '31703493-1c88-458e-87d5-ffb1d4036d65', '5c34609d-0cd0-4342-be02-3b42ba182a0d', 'Keyboards & Piano', false, '2026-09-16 14:20:56.648772+00');


--
-- Data for Name: practice_tasks; Type: TABLE DATA; Schema: public; Owner: -
--

INSERT INTO practice_tasks VALUES ('bcbf36b4-f91b-49b1-af92-2a85f08d9a29', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', 'a6794d91-e226-4e94-9acc-103cccfed2f1', 'review_qc', 'Milestone QC: PHONECERT', 'Verify pitch stability, tempo alignment, and harmonic balance', '43b8694f-bfb7-401d-8ad1-19c7c0755362', 'e87c1e6c-1434-46a1-ba16-42954a5f85ab', 'passed', 'Milestone QC passed! Pitch balance and vocal harmony in the bridge locked at 100%. Approved for Main Concert Stage.', NULL, '2026-09-16 14:20:55.474118+00', '2026-09-16 14:20:55.474118+00');
INSERT INTO practice_tasks VALUES ('eef11a27-e6f3-49ad-b9e2-d1b156cae6f6', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', 'efa1dcb5-3209-4c6e-bfc8-8a6b76828f82', 'review_qc', 'Milestone QC: NÀNG THƠ', 'Verify pitch stability, tempo alignment, and harmonic balance', 'da08a51a-837f-416a-9f72-c417eb79f2f8', '25ec4a6a-687a-4cad-853a-9b8c4299a6f4', 'passed', 'Vocal pitch and piano rubato phrasing passed inspection with distinction.', NULL, '2026-09-16 14:20:55.544565+00', '2026-09-16 14:20:55.544565+00');
INSERT INTO practice_tasks VALUES ('8f753c74-0ef1-4b81-9a47-a5b023340aaa', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', 'fd1e4824-44b6-4097-be41-7d5fa2277e2b', 'review_qc', 'Milestone QC: BẬT TÌNH YÊU LÊN', 'Verify pitch stability, tempo alignment, and harmonic balance', 'b17b54ab-de0a-4f99-badd-b7428ddb2f07', '6732646e-10e0-4f22-99f1-7b553e0570df', 'passed', 'Rhythm section is tight, brass synth stabs fit the mix cleanly. Full pass.', NULL, '2026-09-16 14:20:55.61502+00', '2026-09-16 14:20:55.61502+00');
INSERT INTO practice_tasks VALUES ('5e12aa0b-5f1f-443f-8984-5b805ac15727', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', 'df265763-3fe7-4e33-b91f-32144cbeff8d', 'review_qc', 'Milestone QC: ĐI GIỮA TRỜI RỰC RỠ', 'Verify pitch stability, tempo alignment, and harmonic balance', 'b31d8477-a95a-4608-b152-b307cc1973dc', '1527b949-09f0-46a3-9b43-5c82d7ef5eaf', 'passed', 'Dynamic crescendo in chorus 3 is vibrant. Lead vocal passed.', NULL, '2026-09-16 14:20:55.685274+00', '2026-09-16 14:20:55.685274+00');
INSERT INTO practice_tasks VALUES ('5d9b6438-1766-465b-9e77-338765c72e8a', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '001ab9ed-e397-439d-8815-677c7268530a', 'review_qc', 'Milestone QC: TỪNG QUEN', 'Verify pitch stability, tempo alignment, and harmonic balance', '2b95ecc6-b8b1-4262-942e-28b181c0750a', '85ef1521-bb30-4116-a0e0-f8bd8ab87705', 'under_review', 'Rehearsal recordings submitted. QC review scheduled for Friday night.', NULL, '2026-09-16 14:20:55.752423+00', '2026-09-16 14:20:55.752423+00');
INSERT INTO practice_tasks VALUES ('c41aca8b-8a64-4e4e-a186-204f454e9882', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '088720cc-c3f3-4561-a7c1-b9194a953bb7', 'review_qc', 'Milestone QC: CHÂN ÁI', 'Verify pitch stability, tempo alignment, and harmonic balance', '583899c4-6636-429f-825e-46d0674f2ea7', '3260e15d-534c-4542-a26d-216122cb3be6', 'under_review', 'Pending audit on traditional string synth backing blend.', NULL, '2026-09-16 14:20:55.928847+00', '2026-09-16 14:20:55.928847+00');
INSERT INTO practice_tasks VALUES ('92d3887d-09eb-4d7b-b4fa-454bbecbe48f', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', 'a178de0a-e3b9-489c-94c1-4dff23221186', 'review_qc', 'Milestone QC: BÊN TRÊN TẦNG LẦU', 'Verify pitch stability, tempo alignment, and harmonic balance', 'eaf3aec1-7c19-4021-badc-bf816a278aaf', '5c34609d-0cd0-4342-be02-3b42ba182a0d', 'passed', 'Synth arpeggiator sync and tempo consistency passed.', NULL, '2026-09-16 14:20:55.999689+00', '2026-09-16 14:20:55.999689+00');
INSERT INTO practice_tasks VALUES ('85739264-6441-4873-bb09-266b1819e6b5', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '1503e433-f90c-49a1-b328-90214f0d1c41', 'review_qc', 'Milestone QC: SAU LỜI TỪ KHƯỚC', 'Verify pitch stability, tempo alignment, and harmonic balance', '04fbe6ca-a911-4ec0-bbbc-91b6aebd1b9b', 'e87c1e6c-1434-46a1-ba16-42954a5f85ab', 'passed', 'Superb dynamic transition and emotional impact. Stage ready!', NULL, '2026-09-16 14:20:56.1173+00', '2026-09-16 14:20:56.1173+00');
INSERT INTO practice_tasks VALUES ('64803e43-58ae-4388-a495-633855a351ce', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '47362964-5de3-40f2-9ef6-9792bd422c3d', 'review_qc', 'Milestone QC: MỘT ĐÊM SAY', 'Verify pitch stability, tempo alignment, and harmonic balance', '43b8694f-bfb7-401d-8ad1-19c7c0755362', '6732646e-10e0-4f22-99f1-7b553e0570df', 'passed', 'Waltz swing feel and vocal phrasing approved.', NULL, '2026-09-16 14:20:56.235535+00', '2026-09-16 14:20:56.235535+00');
INSERT INTO practice_tasks VALUES ('ebb45dee-2315-40de-9349-47a050bf234a', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '7e34be8b-4c6a-4611-ab88-bd2ae1337a03', 'review_qc', 'Milestone QC: CƠN MƯA BĂNG GIÁ', 'Verify pitch stability, tempo alignment, and harmonic balance', 'da08a51a-837f-416a-9f72-c417eb79f2f8', '85ef1521-bb30-4116-a0e0-f8bd8ab87705', 'passed', 'Guitar solo and high C vocal sustain verified cleanly.', NULL, '2026-09-16 14:20:56.310538+00', '2026-09-16 14:20:56.310538+00');
INSERT INTO practice_tasks VALUES ('80862d0a-7aca-4f42-80d4-d4fed19d6b56', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', 'b20ee408-0084-4eb6-a189-e26d02396119', 'review_qc', 'Milestone QC: HÀ NỘI 12 MÙA HOA', 'Verify pitch stability, tempo alignment, and harmonic balance', 'b31d8477-a95a-4608-b152-b307cc1973dc', '43b8694f-bfb7-401d-8ad1-19c7c0755362', 'under_review', 'Rehearsal audio uploaded. Audit pending.', NULL, '2026-09-16 14:20:56.427897+00', '2026-09-16 14:20:56.427897+00');
INSERT INTO practice_tasks VALUES ('3b5c4e69-2d08-4c68-8334-77b53492201c', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', 'da717054-7b5d-4c98-9287-1d1d9e1c7cf8', 'review_qc', 'Milestone QC: BÀI CA HY VỌNG', 'Verify pitch stability, tempo alignment, and harmonic balance', '04fbe6ca-a911-4ec0-bbbc-91b6aebd1b9b', '5c34609d-0cd0-4342-be02-3b42ba182a0d', 'passed', 'Show finale centerpiece approved with honors by all reviewers.', NULL, '2026-09-16 14:20:56.599762+00', '2026-09-16 14:20:56.599762+00');
INSERT INTO practice_tasks VALUES ('e6674aea-77c3-4727-8ce8-bec4924284a1', '2c9d8e90-894c-47e3-94a7-01ff65e5115a', '31703493-1c88-458e-87d5-ffb1d4036d65', 'review_qc', 'Milestone QC: BƯỚC QUA MÙA CÔ ĐƠN', 'Verify pitch stability, tempo alignment, and harmonic balance', '0679280e-5d18-4960-86da-e33584e8f601', 'ec4e11fa-4758-4234-aaa6-36f9a4b60d8a', 'passed', 'Guitar ambient swells and vocal pitch tracking verified.', NULL, '2026-09-16 14:20:56.672898+00', '2026-09-16 14:20:56.672898+00');


--
-- Data for Name: sprint_schedule_runs; Type: TABLE DATA; Schema: public; Owner: -
--



--
-- Data for Name: votes; Type: TABLE DATA; Schema: public; Owner: -
--



--
-- PostgreSQL database dump complete
--


COMMIT;
