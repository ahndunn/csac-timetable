# CSAC Timetable Studio 🎵📅

[![CI Test Suite](https://img.shields.io/badge/tests-109%20passed-brightgreen.svg)](file:///home/ahndunn/dev/csac-timetable/clients/web/test_suite.ts)
[![Architecture](https://img.shields.io/badge/architecture-Microservices%20%2B%20SvelteKit-orange.svg)](file:///home/ahndunn/dev/csac-timetable/docs/TECHNICAL.md)
[![Governance](https://img.shields.io/badge/policy-Documentation--Driven%20(DDD)-blue.svg)](file:///home/ahndunn/dev/csac-timetable/docs/DOCUMENTATION_POLICY.md)
[![License](https://img.shields.io/badge/license-MIT-lightgrey.svg)](file:///home/ahndunn/dev/csac-timetable/package.json)

**CSAC Timetable Studio** is an enterprise-grade automated scheduling, rehearsal pipeline, and physical equipment custody platform created for university music clubs, bands, and performance collectives (specifically CSAC — *Club of Songs And Culture*).

The platform transforms manual, conflict-ridden timetable coordination into a high-performance, **Show-Driven Music Production Studio** powered by a **Constraint Satisfaction Problem (CSP) optimization engine**, an **Agile Practice SDLC**, and a **centralized dual-ownership instrument fleet**.

> 📖 **Looking for a user guide?** See [`USAGE.md`](file:///home/ahndunn/dev/csac-timetable/USAGE.md) for the step-by-step pilot team walkthrough.

---

## 📐 System Architecture

CSAC Timetable Studio is structured as a production-ready monorepo combining a high-density SvelteKit SSR web client with asynchronous Rust microservices:

```mermaid
flowchart TB
    subgraph Clients ["Client Layer (clients/)"]
        Web["Web Client (SvelteKit SSR / Svelte 5 Runes)<br/>Routes: /, /studio/*, /admin/*, /auth/*, /utils/*"]
    end

    subgraph GatewayLayer ["Reverse Proxy & Edge Gateway"]
        GW["API Gateway (Rust / Axum 0.8 + Tower)<br/>Argon2id Auth, JWT Middleware, RBAC, REST API (:8080)"]
    end

    subgraph EventAndCache ["Event Bus, Cache & Observability"]
        Redis[("Redis 7.4<br/>OTP Storage (TTL 10m), Token Rate Limiting")]
        Kafka{{"Apache Kafka 3.9 (KRaft Mode)<br/>Topics: user.created, otp.generated, event.status_changed"}}
        OpenObserve[("OpenObserve v0.14<br/>Structured JSON Telemetry & OTLP Traces (:5080)")]
        Mailpit[("Mailpit SMTP<br/>Dev/Prod Email Dispatcher (:1025 / Web :8025)")]
    end

    subgraph Microservices ["Backend Services & Workers (servers/)"]
        SchedSvc["Scheduler Service (Rust 1.85+ / Tonic gRPC)<br/>CSP Timetable Engine"]
        NotifyWorker["Notification Worker (Rust 1.85+ / rdkafka + lettre)<br/>Async SMTP Email Dispatcher"]
    end

    subgraph DataStore ["Persistence Layer"]
        PG[("PostgreSQL 17<br/>users, events, slots, votes, proposals, instruments, numbers, sprints")]
    end

    Web -->|HTTP / REST Proxy (:3000 -> :8080)| GW
    GW <-->|Check / Invalidate OTP & Rate Limits| Redis
    GW <-->|CRUD & Relational Schema Integrity| PG
    GW -->|Publish Domain Events| Kafka
    GW -->|Sync RPC: solve| SchedSvc
    GW -->|JSON Telemetry Logs| OpenObserve

    Kafka -->|Consume Events| NotifyWorker
    NotifyWorker -->|Send Activation Tokens & OTPs| Mailpit
    NotifyWorker -->|JSON Telemetry Logs| OpenObserve
    SchedSvc -->|JSON Telemetry Logs| OpenObserve
```

---

## 🛠️ Technology Stack

| Layer | Technology | Key Libraries & Details |
| :--- | :--- | :--- |
| **Web Frontend** | **SvelteKit SSR** (Svelte 5 Runes) | TypeScript 5.8+, Vite 8+, Tailwind CSS v4 (`@tailwindcss/vite`), shadcn-svelte (`bits-ui`), `@lucide/svelte`, `exceljs`, `xlsx`. |
| **API Gateway** | **Rust 1.85+** (Axum 0.8) | Tower, Hyper, Tokio, `sqlx` (PostgreSQL), `argon2`, `jsonwebtoken`, `rdkafka`, SSE streaming. |
| **Scheduler Engine** | **Rust 1.85+** (Tonic gRPC) | Constraint Satisfaction Problem (CSP) solver, backtracking search with forward-checking heuristics. |
| **Notification Worker**| **Rust 1.85+** (Async Worker) | `rdkafka` consumer, `lettre` SMTP client, dynamic email templating. |
| **Relational Database**| **PostgreSQL 17** | Strict relational integrity, custom enum types (`user_role`, `music_number_status`, `instrument_ownership`), JSONB payloads. |
| **Cache & In-Memory**  | **Redis 7.4** | Ephemeral OTP storage (TTL 10 min, max 3 attempts), distributed token rate limiting. |
| **Event Bus**          | **Apache Kafka 3.9** | KRaft mode (no Zookeeper), durable topics for event-driven decoupled messaging. |
| **Observability**      | **OpenObserve v0.14** | Structured JSON log ingestion and OTLP telemetry tracing (`:5080`). |
| **Mail Sandbox**       | **Mailpit** | Local SMTP mock server (`:1025`) with interactive web inbox (`:8025`). |
| **Container Engine**   | **Podman / Docker Compose** | Multi-stage Dockerfiles (`cargo-chef` dependency caching, Alpine Node.js runner), non-root execution. |

---

## 🗂️ Monorepo Structure

```text
csac-timetable/
├── clients/
│   └── web/                     # SvelteKit SSR Web Application
│       ├── src/
│       │   ├── lib/
│       │   │   ├── api/         # Type-safe Gateway REST/SSE client
│       │   │   ├── auth.ts      # Scoped RBAC rank & capability helpers
│       │   │   ├── engine/      # CSP scheduler, Excel parser & exporter
│       │   │   ├── i18n/        # Symmetric ISO 639-1 dictionaries (vi / en)
│       │   │   ├── stores/      # Svelte 5 runes stores (auth, BroadcastChannel)
│       │   │   └── types/       # Canonical TypeScript domain contracts
│       │   └── routes/
│       │       ├── +page.svelte           # Studio Hub landing page
│       │       ├── studio/                # Show-driven production workspaces
│       │       │   ├── gear/              # Dual-ownership instrument custody fleet
│       │       │   └── shows/[id]/        # Show sub-tabs (Overview, Numbers, Sprints, Roster)
│       │       ├── admin/                 # Administration (Shows, Users, Quorum Demotion)
│       │       ├── auth/                  # Login and self-service OTP activation
│       │       └── utils/                 # Timetable solver & Excel inspector tools
│       ├── test_suite.ts        # Comprehensive automated test suite (109 assertions)
│       └── package.json
├── servers/                     # Rust Microservices Backend
│   ├── gateway/                 # Axum API Gateway & Auth Service
│   ├── scheduler/               # Tonic gRPC CSP Scheduling Service
│   └── notifier/                # Kafka consumer & SMTP notification worker
├── deploy/                      # Infrastructure & Orchestration
│   ├── compose.yml              # Multi-container orchestration (PostgreSQL, Kafka, Redis, etc.)
│   ├── docker/                  # Multi-stage Dockerfiles (cargo-chef & node-alpine)
│   ├── seed_data.sql            # Database bootstrap & pilot accounts seed script
│   └── .env                     # Deployment environment variables
├── docs/                        # Single Source of Truth (SSOT) Specifications
│   ├── BUSINESS.md              # Domain rules, user flows, and business invariants
│   ├── TECHNICAL.md             # Complete architecture, database schema, and API contracts
│   ├── DOCUMENTATION_POLICY.md  # Governance & Documentation-Driven Development (DDD) policy
│   └── TEAM_ROLES_AND_MINDSETS.md # Roleplay framework and cross-team decision heuristics
├── package.json                 # Monorepo root workspace scripts
├── USAGE.md                     # Pilot User Team onboarding & client-facing guide
└── README.md                    # Technical documentation and developer handbook
```

---

## ⚡ Core Technical Subsystems

### 1. Constraint Satisfaction Problem (CSP) Scheduler Engine
* **Zero Double-Booking Invariant**: A performer is mathematically guaranteed never to be scheduled for two concurrent music numbers in the same day/time slot.
* **15-Minute Resolution Matrix**: Supports granular 15-minute intervals (from `08:00` to `22:45`), aggregating into rehearsal sessions with 100% member attendance validation.
* **Studio Capacity Limits**: Enforces room concurrency constraints (`maxRooms`), mapping band sessions to physical rehearsal spaces (Studio A, Studio B, Studio C).
* **Backtracking Heuristic**: Employs Minimum Remaining Values (MRV) variable ordering and forward checking to detect conflict bottlenecks and return ranked candidate alternative slots.

### 2. Show-Driven Agile SDLC & Music Number FSM
The music production lifecycle mirrors Agile software engineering through 5 Finite State Machine (FSM) states:
$$\text{draft} \longrightarrow \text{in\_practice} \longrightarrow \text{ready\_for\_qc} \longrightarrow \text{qc\_approved} \longrightarrow \text{stage\_ready}$$
* **Role Delegation**: 
  - **Show Delivery Manager (DM)**: Scoped to `(user_id, show_id)`, oversees show-wide lineup and triggers the sprint scheduler.
  - **Performance Manager (PM)**: Scoped to `(user_id, show_id, number_id)`, manages performer lineup across 5 instruments (Vocal Lead, Lead Guitar, Bass, Drums, Keys) and rehearsal tasks.
  - **Quality Reviewer (QC)**: Scoped to `(user_id, show_id, number_id)`, conducts mandatory audit reviews with formal verdicts (`passed` vs `revision_requested`).

### 3. Dual-Ownership Instrument Fleet & Custody Protocol (`/studio/gear`)
* **Ownership Distinction**: Centralized registry distinguishing between **CSAC Club Property** and **Member-Owned Gear**.
* **Owner Lending Policies**: Granular permissions (`open_to_all`, `approval_required`, `show_only`, `locked_private`) with sovereign owner revocation rights.
* **5-Phase Live Show Lifecycle**: Gear tracks physical movement across:
  $$\text{Allocated} \longrightarrow \text{Present at Venue} \longrightarrow \text{Active on Stage} \longrightarrow \text{Teardown / Retrieved} \longrightarrow \text{Safe Storage}$$
* **Proxy Retrieval Tracking**: Mandatory logging when equipment is picked up on another member's behalf, flagging the audit trail with a distinct proxy warning.
* **Orphan Gear Lockdown**: Post-show teardown locks the **"Close Show"** action while any equipment remains orphaned, resolved only through the **Adopt-a-Gear** safekeeping protocol.

### 4. Security, Cryptography & Governance
* **Argon2id Hashing**: Password hashing utilizing RFC 9106 recommended parameters ($m=19\text{ MiB}, t=2, p=1$).
* **Reactive Token Invalidation**: Server-Sent Events (`AUTH_INVALIDATED`) broadcast authority changes, prompting the web client to execute a silent token refresh coordinated via the **Web Locks API** and **`BroadcastChannel`** across browser tabs.
* **Multi-Admin Quorum Demotion Protocol (`/admin/approve`)**: Unilateral downgrade of an Administrator is prohibited. Requires cryptographic consensus where $M = \min(\lceil N/2 \rceil, 3)$ peer Admins verify an OTP dispatched via email to approve the demotion.

### 5. Internationalization (i18n)
* Standardized on ISO 639-1 language codes: Vietnamese (`vi` 🇻🇳) and English (`en` 🇺🇸).
* 100% symmetric key parity enforced by automated tests (729+ dictionary keys with zero missing entries).
* URL parameter synchronization (`?lang=vi` / `?lang=en`) and client-side reactive rendering via `$tStore`.

---

## 🚀 Getting Started & Local Development

### Prerequisites
* **Node.js**: `>= 20.0.0`
* **pnpm**: `>= 9.0.0`
* **Rust**: `>= 1.85.0` (for backend services development)
* **Podman** or **Docker**: With Compose support

---

### Option A: Running Full Infrastructure via Compose

The repository includes pre-configured Compose profiles for different development workflows:

```bash
# Clone the repository
git clone https://github.com/csac/csac-timetable.git
cd csac-timetable

# Launch full production stack (all 8 microservices & datastores)
podman compose -f deploy/compose.yml up -d

# Check service container health
podman compose -f deploy/compose.yml ps
```

Access points:
* **Web Client**: `http://localhost:3000` (or `http://localhost:5173` if running dev server)
* **API Gateway**: `http://localhost:8080`
* **OpenObserve Console**: `http://localhost:5080` (admin credentials in `deploy/.env`)
* **Mailpit Web UI**: `http://localhost:8025` (incoming OTPs and activation emails)

---

### Option B: Fast Frontend Iteration (Dev Profile)

Run backend infrastructure in containers while developing the SvelteKit frontend locally with instant Hot Module Replacement (HMR):

```bash
# 1. Start backend services and databases only
podman compose -f deploy/compose.yml --profile backend up -d

# 2. Install workspace dependencies
pnpm install

# 3. Start SvelteKit Vite dev server
pnpm run dev
```

The web client will launch at **`http://localhost:5173`** with proxy forwarding configured to the local Gateway at port 8080.

---

## 🔑 Pre-Seeded Pilot Test Accounts

The PostgreSQL initialization script ([`deploy/seed_data.sql`](file:///home/ahndunn/dev/csac-timetable/deploy/seed_data.sql)) seeds pre-configured accounts across all 6 role tiers:

| Email | Full Name | Role | Test Focus |
| :--- | :--- | :--- | :--- |
| `admin@csac.studio` | System Administrator | **Admin** | User onboarding, show setup, quorum demotion. |
| `grace@csac.studio` | Grace | **Delivery Manager (DM)** | Show roster management, auto-schedule execution. |
| `alice@csac.studio` | Alice | **Performance Manager (PM)** | Band lineup assignment, song stage progression. |
| `charlie@csac.studio` | Charlie | **Quality Reviewer (QC)** | Song audit workstation, QC feedback verdicts. |
| `bob@csac.studio` | Bob | **Member / Performer** | 15-min free time drag-selection, personal schedule. |
| `diana@csac.studio` | Diana | **Member / Performer** | Schedule attendance, availability matrix. |
| `eve@csac.studio` | Eve | **Member / Performer** | Guitarist profile, gear reservations. |

*(In local mock/dev mode, any password will authenticate successfully).*

---

## 🧪 Testing & Quality Assurance

Our engineering pipeline adheres to strict verification invariants:

```bash
# 1. Run full automated test suite (109 assertions)
pnpm run test

# 2. Verify TypeScript contracts & Svelte diagnostics
pnpm run check

# 3. Execute production Vite build
pnpm run build

# 4. Run fast Rust-based linter (oxlint)
pnpm run lint

# 5. Validate Podman/Docker Compose configuration
podman compose -f deploy/compose.yml config
```

### Automated Test Suite Overview (`test_suite.ts`)
The test runner executes **109 automated assertions** across 15 distinct domains:
* **Tests 1–3**: Sample data generation, CSP solver optimization, zero double-booking, and conflict explanations.
* **Tests 4–8**: Multi-tab Excel workbook parsing, round-trip matrix validation, and selective sheet importing.
* **Test 9**: Symmetric ISO 639-1 i18n parity across all 729 keys in English and Vietnamese.
* **Tests 10–12**: Show route topology, FSM state transitions, fatigue threshold calibration ($\ge 5$ numbers), and PostgreSQL seed integrity.
* **Tests 13–15**: Asynchronous Kafka/SSE sprint scheduling pipeline, compound scoped role resolution (`getEffectiveRole`), and type-safe API client contracts.

---

## 📜 Documentation-Driven Development (DDD) Policy

In this repository, **Documentation is the Single Source of Truth (SSOT)**. Code is a derived implementation byproduct.

1. **Rule 1**: Before inspecting or modifying code, consult [`docs/TECHNICAL.md`](file:///home/ahndunn/dev/csac-timetable/docs/TECHNICAL.md) and [`docs/BUSINESS.md`](file:///home/ahndunn/dev/csac-timetable/docs/BUSINESS.md).
2. **Rule 2**: When adding features or changing contracts, update the specification under `docs/` **first**, write the code **second**, and verify with automated tests **third**.
3. **Rule 3**: In any discrepancy between code behavior and documentation, the documentation is authoritative.

For full governance details, see [`docs/DOCUMENTATION_POLICY.md`](file:///home/ahndunn/dev/csac-timetable/docs/DOCUMENTATION_POLICY.md).

---

## 📄 License

This project is licensed under the terms of the **MIT License**.
Distributed by the **CSAC Engineering & Production Team**. 🎵🎸
