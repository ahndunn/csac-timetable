# Technical Architecture & Engineering Specification

**Project**: CSAC Timetable Studio 🎵📅  
**Document Status**: Active / Single Source of Truth (SSOT)  
**Last Updated**: September 2026

---

## 1. System Architecture & Tech Stack

### 1.1 Architecture Overview
CSAC Timetable Studio is architected as a production-grade multi-service monorepo:

```mermaid
flowchart TB
    subgraph Clients ["Client Layer (clients/)"]
        Web["Web Client (SvelteKit SSR / Svelte 5 Runes)<br/>Routes: /, /utils/*, /admin/*, /auth/login, /events/*"]
    end

    subgraph GatewayLayer ["Reverse Proxy & Edge Gateway"]
        GW["API Gateway (Rust / Axum + Tower)<br/>Argon2id Auth, JWT Middleware, RBAC, REST API"]
    end

    subgraph EventAndCache ["Event Bus, Cache & Observability"]
        Redis[("Redis 7.4<br/>OTP Storage (TTL 10m), Token Rate Limiting")]
        Kafka{{"Apache Kafka 3.9 (KRaft Mode)<br/>Topics: user.created, otp.generated, event.status_changed"}}
        OpenObserve[("OpenObserve v0.14<br/>Unified Structured JSON Logs & OTLP Traces (:5080)")]
        Mailpit[("Mailpit / SMTP<br/>Dev/Prod SMTP Email Dispatcher (:1025/:8025)")]
    end

    subgraph Microservices ["Backend Services & Workers (servers/)"]
        SchedSvc["Scheduler Service (Rust 1.85+ / Tonic gRPC)<br/>CSP Timetable Engine"]
        NotifyWorker["Notification Worker (Rust 1.85+ / rdkafka + lettre)<br/>Async SMTP Email Dispatcher"]
    end

    subgraph DataStore ["Persistence Layer"]
        PG[("PostgreSQL 17<br/>users, events, event_time_slots, votes, admin_downgrade_proposals, admin_downgrade_votes, audit_logs")]
    end

    Web -->|HTTP / REST Proxy (:3000 -> :8080)| GW
    GW <-->|Check / Consume OTP & Rate Limits| Redis
    GW <-->|CRUD & Relational Integrity| PG
    GW -->|Produce Events| Kafka
    GW -->|Sync RPC: solve| SchedSvc
    GW -->|JSON Telemetry Logs| OpenObserve

    Kafka -->|Consume Events| NotifyWorker
    NotifyWorker -->|Send Email Credentials / OTP| Mailpit
    NotifyWorker -->|JSON Telemetry Logs| OpenObserve
    SchedSvc -->|JSON Telemetry Logs| OpenObserve
```

### 1.2 Web Client Reverse Proxy Architecture
* **SvelteKit SSR Proxy Hook (`clients/web/src/hooks.server.ts`)**:
  * Intercepts all incoming client requests matching `/api/*`.
  * Forwards requests seamlessly to the Rust API Gateway backend at `PUBLIC_GATEWAY_URL` or `GATEWAY_URL` (defaulting to `http://gateway:8080` in containerized environments, and `http://localhost:8080` in local development).
  * Forwards HTTP method, request headers, query parameters, and streaming request body, while returning the Gateway's response and status code.
* **Vite Dev Server Proxy (`clients/web/vite.config.ts`)**:
  * Configures development server proxy for `/api` pointing to `http://localhost:8080`.

### 1.3 Web Client Component Specifications
* **Bento Card Surfaces (`.bento-card`)**: 18px rounded corner radius (`--radius-bento`), 1px translucent border (`rgba(0,0,0,0.08)`), multi-layer elevation shadow (`--shadow-card`).
* **Kanban Music Numbers (`/studio/shows/[id]/numbers`)**:
  * Action button row (`.card-btn-row`) uses responsive wrapping (`flex-wrap: wrap`) and full flex sizing (`flex: 1 1 auto`) to ensure labels (such as "Submit for QC" and "Assign Lineup") adapt without text clipping.


---

## 2. Database Schema (PostgreSQL 16)

```sql
-- Role and Status Enums
CREATE TYPE user_role AS ENUM ('admin', 'moderator', 'member');
CREATE TYPE user_status AS ENUM ('active', 'suspended');
CREATE TYPE event_status AS ENUM ('draft', 'open', 'closed');
CREATE TYPE proposal_status AS ENUM ('pending', 'approved', 'rejected', 'expired');
CREATE TYPE vote_decision AS ENUM ('approve', 'reject');

-- 1. Users Table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL UNIQUE,
    full_name VARCHAR(255) NOT NULL,
    password_hash VARCHAR(255) NOT NULL, -- Argon2id hash
    role user_role NOT NULL DEFAULT 'member',
    status user_status NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Initial Admin Seed
-- Seed admin@csac.local account into users table
INSERT INTO users (email, full_name, password_hash, role, status)
VALUES ('admin@csac.local', 'System Administrator', '$argon2d$v=19$m=16,t=2,p=1$U2lYQjBCNmlMakxURUlRag$M/e/uvcWAVwPmvflmP0Yfg', 'admin', 'active')
ON CONFLICT (email) DO UPDATE SET password_hash = EXCLUDED.password_hash, role = EXCLUDED.role, status = EXCLUDED.status;

-- 2. Events Table
CREATE TABLE events (
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
CREATE TABLE event_time_slots (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_id UUID NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    day_of_week VARCHAR(50) NOT NULL,
    slot_label VARCHAR(100) NOT NULL,
    sort_order INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 4. Member Votes Table
CREATE TABLE votes (
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
CREATE TABLE admin_downgrade_proposals (
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
CREATE TABLE admin_downgrade_votes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    proposal_id UUID NOT NULL REFERENCES admin_downgrade_proposals(id) ON DELETE CASCADE,
    admin_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    decision vote_decision NOT NULL,
    voted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT unique_proposal_admin UNIQUE (proposal_id, admin_id)
);

-- 7. Audit Logs Table
CREATE TABLE audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    actor_id UUID REFERENCES users(id) ON DELETE SET NULL,
    action VARCHAR(100) NOT NULL,
    resource_type VARCHAR(100) NOT NULL,
    resource_id UUID,
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 8. Music Organization Enums
CREATE TYPE instrument_ownership AS ENUM ('club_property', 'member_owned');
CREATE TYPE instrument_availability AS ENUM ('free_to_borrow', 'in_use', 'unavailable', 'in_maintenance');
CREATE TYPE music_number_status AS ENUM ('draft', 'in_practice', 'ready_for_qc', 'qc_approved', 'stage_ready');
CREATE TYPE task_type AS ENUM ('study', 'create', 'review_qc');
CREATE TYPE task_status AS ENUM ('todo', 'in_progress', 'under_review', 'passed', 'blocked');

-- 9. Instruments Table (CSAC Property & Member-Owned)
CREATE TABLE instruments (
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

-- 10. Music Numbers Table
CREATE TABLE music_numbers (
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

-- 11. Music Number Members (Performer Lineup)
CREATE TABLE music_number_members (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    music_number_id UUID NOT NULL REFERENCES music_numbers(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    instrument_role VARCHAR(100) NOT NULL, -- e.g. Lead Vocal, Rhythm Guitar, Bass, Keys, Drums
    is_lead BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT unique_number_member UNIQUE (music_number_id, user_id)
);

-- 12. Practice Sprints (Agile SDLC Sprints)
CREATE TABLE practice_sprints (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_id UUID REFERENCES events(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    sprint_goal TEXT,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 13. Practice Tasks & QC Reviews
CREATE TABLE practice_tasks (
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

-- 14. Instrument Reservations (Zero Double-Booking Guarantee)
CREATE TABLE instrument_reservations (
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

-- 15. Member Sprint Availabilities (Fast Free-Time Registration)
CREATE TABLE member_sprint_availabilities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    sprint_id UUID NOT NULL REFERENCES practice_sprints(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    day_of_week VARCHAR(50) NOT NULL,
    slot_label VARCHAR(100) NOT NULL,
    is_available BOOLEAN NOT NULL DEFAULT TRUE,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT unique_sprint_user_slot UNIQUE (sprint_id, user_id, day_of_week, slot_label)
);
```

---

## 3. Security, Authentication & Cryptography

### 3.1 Argon2id Password Hashing
- **Algorithm**: `Argon2id` (RFC 9106)
- **Parameters**:
  - Memory: $19\text{ MiB}$ ($19456\text{ KiB}$)
  - Iterations: $2$
  - Parallelism: $1$
  - Salt: $16$ cryptographically secure random bytes

### 3.2 JWT Token Architecture
- **Header**: `{"alg": "HS256", "typ": "JWT"}`
- **Payload Claims**:
  ```json
  {
    "sub": "uuid-user-id",
    "email": "user@csac.local",
    "name": "Member Name",
    "role": "admin" | "moderator" | "member",
    "exp": 1780000000,
    "iat": 1779000000
  }
  ```
- **Transmission**: `Authorization: Bearer <token>` or HttpOnly cookie `csac_session`.

### 3.3 Redis OTP Storage Protocol
- **Key Pattern**: `otp:admin:{admin_id}:{proposal_id}`
- **Value**: `{"code": "6-digit-string", "attempts": 0}`
- **TTL**: $600\text{ seconds}$ (10 minutes).
- **Rate Limit**: Max 3 incorrect attempts before key invalidation.

---

## 4. REST API Endpoint Specifications

### 4.1 Authentication (`/api/v1/auth`)
* `POST /api/v1/auth/login`: `{ email, password }` $\rightarrow$ `{ token, user: { id, email, full_name, role } }`.
* `GET /api/v1/auth/me`: Validates JWT $\rightarrow$ returns current user profile.

### 4.2 Admin User Management (`/api/v1/admin/users`) — *Admin Only*
* `GET /api/v1/admin/users`: List users with pagination and role filter.
* `POST /api/v1/admin/users`: Create user `{ email, full_name, role }` $\rightarrow$ generates random password, hashes with Argon2id, writes to DB, emits Kafka `csac.user.created` event.
* `PUT /api/v1/admin/users/:id/role`: Change user role to `moderator` or `admin` (or initiate downgrade proposal if target is `admin`).
* `POST /api/v1/admin/users/:id/downgrade-proposal`: Create downgrade proposal `{ target_role, reason }` $\rightarrow$ calculates required approvals $M = \min(\lceil N/2 \rceil, 3)$.

### 4.3 Admin Quorum Approval (`/api/v1/admin/approve`) — *Admin Only*
* `GET /api/v1/admin/approve/proposals`: List all pending and historical downgrade proposals.
* `POST /api/v1/admin/approve/:proposal_id/request-otp`: Generates 6-digit OTP in Redis and sends via Kafka `csac.admin.otp_generated`.
* `POST /api/v1/admin/approve/:proposal_id/vote`: Submit `{ otp, decision: 'approve' | 'reject' }` $\rightarrow$ checks OTP, records vote, updates role if quorum met.

### 4.4 Event Management (`/api/v1/events`) — *Admin & Moderator*
* `GET /api/v1/events`: List events.
* `POST /api/v1/events`: Create event `{ title, description, start_date, end_date, time_slots: [...] }`.
* `PUT /api/v1/events/:id/close`: Close event $\rightarrow$ prevents further voting.
* `GET /api/v1/events/:id/votes`: Retrieve voting matrix for event.
* `POST /api/v1/events/:id/vote`: Submit member vote `{ slot_id, is_available, note }` *(Any authenticated user if event is open)*.

### 4.5 Music Numbers & Instrument Fleet Management (`/api/v1/music`)
* `GET /api/v1/music/numbers`: List music numbers with PM and performers lineup.
* `POST /api/v1/music/numbers`: Create a music number `{ title, genre, pm_user_id, target_sessions_per_week }`.
* `GET /api/v1/music/instruments`: List instruments with ownership type, current custody ("kept by whom"), and availability.
* `POST /api/v1/music/instruments`: Register a new instrument (CSAC club property or member personal gear) `{ name, code, category, ownership_type, owner_user_id, custody_location }`.
* `PUT /api/v1/music/instruments/:id/status`: Update availability (`free_to_borrow`, `unavailable`, `in_maintenance`) or transfer custody `{ custody_user_id, custody_location, availability_status }`.
* `POST /api/v1/music/instruments/reserve`: Reserve an instrument for a rehearsal session `{ instrument_id, music_number_id, day_of_week, slot_label }`. Returns `409 Conflict` if the instrument is already booked for that slot.

### 4.6 Agile Practice Sprints & Scheduling (`/api/v1/sprints`)
* `GET /api/v1/sprints`: List practice sprints.
* `GET /api/v1/sprints/:id/tasks`: List all tasks (study, create, review_qc) for a sprint with QC status and feedback.
* `POST /api/v1/sprints/:id/tasks`: Create practice task `{ music_number_id, task_type, title, description, assigned_to, qc_reviewer_id }`.
* `PUT /api/v1/sprints/:id/tasks/:task_id/review`: Submit QC review verdict `{ status: 'passed' | 'blocked' | 'in_progress', qc_feedback: string }`.
* `POST /api/v1/sprints/:id/availability`: Fast 1-click member free-time slot registration `{ slots: [{ day_of_week, slot_label, is_available }] }`.
* `POST /api/v1/sprints/:id/schedule`: Backend CSP scheduling engine stub $\rightarrow$ returns `{ "status": "not_implemented", "message": "Backend CSP scheduling engine will be implemented in upcoming release" }` with HTTP status `501 Not Implemented`.

---

## 5. Observability & Telemetry (OpenObserve)

* **OpenObserve Service**: Deployed on port `5080` (HTTP web console & API), with user/password credentials configured via container environment.
* **Structured Tracing**: All Rust Axum endpoints emit JSON logs containing:
  ```json
  {
    "timestamp": "2026-09-11T00:45:00Z",
    "level": "INFO",
    "service": "gateway",
    "request_id": "req-12345",
    "method": "POST",
    "path": "/api/v1/admin/users",
    "status": 201,
    "user_id": "uuid-admin-id",
    "role": "admin",
    "latency_ms": 12.4
  }
  ```
* **Log Shipper**: Services stream structured logs to OpenObserve via HTTP endpoint `http://openobserve:5080/api/default/default/_json`.

---

## 6. SvelteKit Web Route Restructuring & Design System Architecture

### 6.1 Route Directory & Primary Application Pillars
* `/`: Main Entrypoint Hub with Bento grid navigation modules, highlighting the CSAC Music Production Studio as the primary flagship workflow.
* `/studio`: CSAC Music Production Studio — Agile Practice SDLC (Study, Create, Review QC), song lineup tracking, dual-ownership instrument fleet management, 15-minute fine-grained click-and-drag sprint free-time registration grid, and auto-scheduled sprint rehearsals calendar display.
* `/auth/login`: Bento-styled modern login card with True Orange accents.
* `/admin/users`: User management table, role modal, user onboarding form.
* `/admin/events`: Event creation wizard, voting status toggle (Open/Close), slot configuration.
* `/admin/approve`: Quorum approval cards, interactive OTP modal, ballot progress bar ($k / M$ votes).
* `/events/[id]/vote`: Performer availability voting matrix.
* `/utils/timetable`: Dedicated legacy timetable solver, pastel cards, sheet selector modal, conflict resolver, contextual solver toolbar, and Excel exporter.
* `/utils/inspector`: Excel workbook multi-tab inspector.

### 6.2 Global Navbar & Dedicated Viewport Architecture
* **Decoupled Global Navbar (`Navbar.svelte`)**:
  * Provides clean, distraction-free top-level application navigation across all routes (`Studio`, `Sự kiện / Events`, `Quản trị / Admin`, `Hub`).
  * Features the Brand Logo, Navigation Hub Dropdown, User Profile Badge / Sign In CTA, and ISO 639-1 Language Switcher.
  * All legacy timetable-specific solver actions (title editor, file upload, auto-scheduler solve button, Excel export, sample data loaders) are strictly optional and rendered only when explicitly enabled or passed by the dedicated `/utils/timetable` tool.
  * Eliminates dummy no-op handler props from all non-timetable routes.
* **Viewport Scrolling & Bento Card Surfaces (`app.css`)**:
  * Body and `#root` maintain flexible full-height layouts (`min-height: 100vh; overflow-y: auto; overflow-x: hidden;`) allowing natural vertical scrolling without double scrollbars or overflow clipping on content-rich pages (`/studio`, `/admin/*`, `/`).
  * Dedicated fixed-viewport tools such as `/utils/timetable` constrain their internal grid containers (`.app-container { height: calc(100vh - 68px); overflow: hidden; }`) without constraining the outer HTML document.
  * Standardized `.bento-tabs` and `.bento-tab-btn` styling across the application for tabbed interfaces (e.g. role switcher and instrument fleet filters).

---

## 7. Internationalization (i18n) Architecture & Standards

### 7.1 ISO 639-1 Compliance & Zero-Dependency Svelte 5 Runes Engine
* **Supported Locales**: Strictly standardized on ISO 639-1 two-letter codes:
  * `vi`: Vietnamese (Tiếng Việt 🇻🇳)
  * `en`: English (English 🇺🇸)
* **Zero Hardcoded Display Text Rule**: Every single user-visible string across all routes (`/`, `/auth/*`, `/admin/*`, `/events/*`, `/utils/*`) must be rendered using reactive translations via `$tStore('namespace.key')` or `translate(locale, 'namespace.key', params)`. Hardcoded template text is strictly prohibited.
* **1-to-1 Translation Parity**: Dictionaries (`clients/web/src/lib/i18n/locales/vi.ts` and `clients/web/src/lib/i18n/locales/en.ts`) must maintain exact symmetric key parity. Automated tests validate 100% mutual presence and fail if any key is missing in either dictionary.
* **Key Namespaces**:
  * `hub`: Landing page hero, tags, descriptions, CTA buttons, and Bento card modules.
  * `auth`: Login card, form fields, placeholders, action buttons, error messages.
  * `admin_users`: User directory, role promotion, downgrade proposals, credentials dispatch.
  * `admin_approve`: Multi-admin quorum approval ballots, OTP verification modal, voting decisions.
  * `admin_events`: Rehearsal voting events lifecycle, slot builder, date ranges.
  * `events_vote`: Performer interactive availability matrix, slot checkboxes, submission feedback.
  * `inspector`: Excel workbook validator, sheet previews, integrity inspection.
  * `nav`: Studio app switcher dropdown, auth badges, profile indicators, mobile navigation.
  * `navbar`, `days`, `sidebar`, `calendar`, `upload_modal`, `sheet_modal`, `conflict_modal`, `event_detail_modal`, `slot_add_modal`, `raw_vote_modal`, `messages`: Core timetable scheduler engine and solver dialogs.

---

## 8. Containerization & Docker Build Optimization Standards

### 8.1 Rust Microservices Build Pipeline (`deploy/docker/Dockerfile.service` & `deploy/docker/Dockerfile.gateway`)
* **`cargo-chef` Recipe Dependency Caching**:
  * **Planner Stage**: Utilizes `lukemathwalker/cargo-chef:latest-rust-1-bookworm` to inspect `Cargo.lock` and workspace manifests, producing an isolated `recipe.json`.
  * **Dependency Cook Stage**: Runs `cargo chef cook --release --recipe-path recipe.json --bin ${SERVICE_NAME}`. All 300+ external crates (e.g. `axum`, `sqlx`, `tonic`, `tokio`, `rdkafka`, `argon2`) are compiled in a discrete cache layer that is preserved across application code modifications.
  * **Application Build Stage**: Copies service sources and executes `cargo build --release --bin ${SERVICE_NAME}`. Debug symbols are stripped (`strip`) to minimize output binary size (~70% reduction).
  * **BuildKit Cache Mounts**: Mounts `/usr/local/cargo/registry` and `/usr/local/cargo/git` across compilation steps to prevent redundant network downloads of crate sources.
* **Minimal Non-Root Runtime Layer**:
  * Base: `debian:bookworm-slim` with minimal runtime libraries (`ca-certificates`, `libssl3`, `tzdata`).
  * Security Context: Dedicated non-root user `USER 10001:10001`.

### 8.2 Web Frontend Build Pipeline (`deploy/docker/Dockerfile.web`)
* **Multi-Stage Node / SvelteKit Build**:
  * **Builder Stage (`node:22-alpine`)**: Uses Corepack-managed `pnpm` with persistent cache mount (`--mount=type=cache,id=pnpm,target=/pnpm/store`) for frozen-lockfile dependency resolution. Executes `pnpm run build` followed by `pnpm prune --prod` to discard development-only tooling (`vite`, `svelte-check`, `typescript`, `@sveltejs/kit`).
  * **Runner Stage (`node:22-alpine`)**: Copies only the compiled `@sveltejs/adapter-node` standalone server (`build/`), pruned production dependencies (`node_modules/`), and `package.json`.
  * Security Context: Non-root user `USER node`.

### 8.3 Docker & Podman Compose Profiles Strategy (`deploy/compose.yml`)
* **Default Stack Profile (`full` / default)**:
  * Running `podman compose -f deploy/compose.yml up -d` default-targets the `full` profile via `deploy/.env` (`COMPOSE_PROFILES=full`).
  * Launches all 8 services: `postgres`, `redis`, `kafka`, `openobserve`, `mailpit`, `gateway`, `scheduler-service`, and `web`.
* **Fast-Iteration Dev Profile (`dev` / `backend`)**:
  * Running `podman compose -f deploy/compose.yml --profile dev up -d` (or `--profile backend`) launches all backend microservices and infrastructure components (`postgres`, `redis`, `kafka`, `openobserve`, `mailpit`, `gateway`, `scheduler-service`), omitting the containerized SvelteKit `web` frontend.
  * Allows developers to run `web` locally (`pnpm --prefix clients/web run dev`) for instant Hot Module Replacement (HMR) and rapid UI development.

