# Business & Product Specification

**Project**: CSAC Timetable Studio 🎵📅  
**Document Status**: Active / Single Source of Truth (SSOT)  
**Last Updated**: September 2026

---

## 1. Product Vision & Value Proposition

### 1.1 Executive Summary
**CSAC Timetable Studio** is an enterprise-grade automated scheduling and member management system created for university music clubs, bands, and cultural performance groups (specifically CSAC — Club of Songs And Culture). 

The platform provides:
1. **Show-Driven Music Production Studio (`/studio`)**: Centered around active Music Shows (`/studio/shows/[id]/*`) with dedicated sub-pages for Show Overview, Music Numbers (Kanban & QC audit), Practice Sprints (1-click free-time registration), and Show Roster.
2. **Independent Instrument Fleet & Custody (`/studio/gear`)**: Centralized dual-ownership instrument registry (club property & member-owned gear) and physical location tracking.
3. **Show Administration & Live Monitoring (`/admin/shows`)**: Show design, capacity planning, create modal, and live rehearsal monitor dashboard.
4. **User Governance Directory (`/admin/users`)**: Role-based user onboarding (Argon2id auth) and dynamic promotion/demotion authority.
5. **Multi-Admin Quorum Demotion Protocol (`/admin/approve`)**: Cryptographically verified peer-review governance requiring OTP approval from peer administrators before an Admin can be downgraded.
6. **Legacy Code Deprecation**: Standalone utility routes (`/utils/*`) and voting event routes (`/events/*`) are eliminated and replaced by show-driven practice sprints.

---

## 2. Role-Based Access Control (RBAC) & Governance Rules

### 2.1 Role Hierarchy & Capabilities

The system organizes access across 6 distinct role levels, structured as strict supersets where higher levels inherit all capabilities of lower levels:

| Role | Scope & Permissions | Key User Flows & Viewable Features | SSR Action Stripping Policy |
| :--- | :--- | :--- | :--- |
| **Admin** | Full system governance: Manage users, promote/demote roles, quorum demotion, event creation, trigger scheduler, view full history. | User onboarding, quorum ballot reviews (`/admin/approve`), system audit inspection, event lifecycle (`/admin/events`), all Studio views. | **None**: All action controls rendered. |
| **Moderator** | Event operations & scheduling (Superset of DM): Create/close voting events, define time slots, trigger sprint scheduler. | Event creation (`/admin/events`), schedule finalization, voting monitoring, all DM/PM/QC/Member views. | **Stripped**: User demotion quorum approval (`/admin/approve`). |
| **Delivery Manager (DM)** | Show Lineup & Fleet Director (Superset of PM): Oversee all show music numbers, trigger sprint scheduler, manage instrument allocations. | **Trigger Sprint Scheduler**, inspect full audit history (compute & free-time registration history), manage all numbers in Bento/Kanban/Table views. | **Stripped**: User onboarding & role demotion (`/admin/users`, `/admin/approve`). |
| **Performance Manager (PM)** | Music Number Leader (Superset of QC): Lead assigned Music Numbers, assign performers & gear, manage Study/Create/Review tasks. | Manage assigned numbers, performer lineup configuration, inspect compute & registration history, submit task reviews. | **Stripped**: Show-wide scheduler trigger, admin pages (`/admin/*`). |
| **Quality Reviewer (QC)** | Quality Auditor (Superset of Member): Review practice tasks and submit milestone audit verdicts. | Member views + QC Audit Workstation tab in Music Numbers. | **Stripped**: Number creation, performer assignments, scheduler trigger, user administration. Enabled ONLY for QC task reviews. |
| **Member** | Performer baseline: Register 15-minute practice sprint free-time grid, view assigned numbers & practice schedule calendar. | Show Studio overview, Music Numbers view, Practice Sprint free-time registration grid & auto-scheduled calendar, Instrument Fleet. | **Stripped**: All action buttons for create/edit/delete numbers, instruments, tasks, scheduler trigger, or admin pages. Non-actionable elements only. |

### 2.2 Business Rules for Administration

* **BR-ADM-01 (User Onboarding & Credential Dispatch)**:
  * When an Admin creates a user (Name, Email, Role), the system generates a secure initial password.
  * The password is encrypted with **Argon2id** for database storage.
  * An asynchronous event is dispatched to send the user their login credentials via SMTP email.
* **BR-ADM-02 (Promotion Authority)**:
  * Any active Admin can promote a Member to Moderator or Admin immediately.
  * Any active Admin can promote a Moderator to Admin immediately.
* **BR-ADM-03 (Admin Downgrade Quorum Protocol)**:
  * Downgrading an Admin (to Moderator or Member) **cannot** be executed unilaterally.
  * Initiating a downgrade creates an **Admin Downgrade Proposal** with status `PENDING`.
  * The required approval count $M$ is calculated as:
    $$M = \min\left(\left\lceil \frac{N}{2} \right\rceil, 3\right)$$
    where $N$ is the total count of active Admins at the time of proposal creation.
  * **Sole Admin Protection**: If $N = 1$, downgrade proposals are strictly prohibited.
  * **Peer Review & Self-Resignation**: An Admin can initiate a demotion on any Admin or on themselves (self-resignation).
  * **OTP Verification**: To approve/reject, each peer Admin requests a 6-digit one-time password (OTP) sent to their email (TTL: 10 minutes) and submits it at `/admin/approve`.
  * Once $M$ approvals are collected, the target user's role is downgraded in PostgreSQL and audit logs are recorded.

---

## 3. Event Management & Voting Rules

* **BR-EVT-01 (Event Creation & Date Range)**:
  * Admins and Moderators can create voting events with a title, description, start date, end date, and customizable time slots (e.g., 17h-18h, 18h-19h).
  * Events are created in status `OPEN` (or `DRAFT`).
* **BR-EVT-02 (Voting Window & Invariant)**:
  * While status is `OPEN`, registered Members can cast/update their availability.
* **BR-EVT-03 (Event Closing)**:
  * Admins and Moderators can transition an event status to `CLOSED`.
  * Once `CLOSED`, all subsequent vote submissions are rejected immediately.
  * Closed events serve as input data to generate optimized rehearsal timetables.

---

## 4. Standalone Utilities Suite (`/utils/*`)

All original client-side timetable generation and Excel tools reside under `/utils/*`:
* **BR-UTL-01 (`/utils/timetable`)**: The primary automated CSP timetable solver, interactive pastel calendar grid, manual slot override, and conflict resolver modal.
* **BR-UTL-02 (`/utils/inspector`)**: Multi-sheet workbook inspector and sheet selector.
* **BR-UTL-03 (Zero Double-Booking Guarantee)**: A member **shall never** be scheduled for two different songs in the same day/time slot.
* **BR-UTL-04 (Room Allocation Limit)**: Total concurrent sessions **shall not** exceed `maxRooms` (default: 1 room).
* **BR-UTL-05 (Excel Master Report)**: Generates 3-sheet `.xlsx` workbook (`LỊCH TẬP TUẦN`, `CHI TIẾT BÀI HÁT`, `LỊCH CÁ NHÂN`).

---

## 5. User Journey & Workflow Specifications

```mermaid
sequenceDiagram
    autonumber
    actor Admin as Admin
    actor Peer as Peer Admin
    participant Gateway as Axum Gateway
    participant DB as PostgreSQL 16
    participant Redis as Redis 7
    participant Kafka as Apache Kafka
    participant Mailer as SMTP Mailer

    Admin->>Gateway: POST /api/v1/admin/users/downgrade (Target: Admin B)
    Gateway->>DB: Check N (Active Admins count)
    Gateway->>DB: Insert Proposal (Req Approvals = min(ceil(N/2), 3))
    Gateway->>Kafka: Publish admin.downgrade.requested
    
    Peer->>Gateway: POST /api/v1/admin/approve/request-otp
    Gateway->>Redis: Store OTP (6-digits, 10 min TTL)
    Gateway->>Kafka: Publish admin.otp.generated
    Kafka->>Mailer: Send OTP Email to Peer Admin
    
    Peer->>Gateway: POST /api/v1/admin/approve/verify-and-vote (Proposal ID, OTP, Decision: APPROVE)
    Gateway->>Redis: Validate and Invalidate OTP
    Gateway->>DB: Record Admin Vote
    alt Quorum Reached (Approvals >= M)
        Gateway->>DB: UPDATE users SET role = 'moderator' WHERE id = Target
        Gateway->>DB: UPDATE proposals SET status = 'approved'
        Gateway->>Kafka: Publish admin.downgraded
    end
```

---

## 6. CSAC Music Production, Agile Practice SDLC & Instrument Fleet Governance

### 6.1 Music Numbers & Leadership Model
* **Delivery Manager (DM)**: Oversees the overall music event lineup, cross-number rehearsals, instrument allocation heatmap, and final stage-readiness audits across all numbers.
* **Performance Manager (PM)**: Directly responsible for a single **Music Number** (song/performance). Assigns performers, sets song target frequencies, coordinates rehearsal objectives, and assigns Quality Check (QC) reviewers.
* **Performers / Band Members**: Club musicians assigned to specific musical roles (e.g., Lead Vocal, Backing Vocal, Electric Guitar, Acoustic Guitar, Bass, Keyboard/Piano, Drum Kit, Percussion).
* **Scalable Music Numbers Interface (`/studio/shows/[id]/numbers`)**:
  - Concert productions naturally feature **10 to 20 numbers**. To eliminate cognitive overload and extreme vertical scrolling in Kanban columns, the interface provides:
    1. **Multi-View Modes**:
       - **Bento Grid (Default)**: Responsive multi-column layout showing rich cards with band lineups, PM assignments, QC notes, and status actions.
       - **Kanban Board**: 5-stage workflow board with horizontal scrolling and stage headers.
       - **Compact Table**: High-density tabular overview optimized for DMs/PMs to inspect and update all 10–20 numbers at once.
    2. **Pipeline Stage Funnel & Metrics**: Real-time summary header displaying the distribution across all 5 stages (`draft`, `in_practice`, `ready_for_qc`, `qc_approved`, `stage_ready`) with one-click filtering.
    3. **Live Search & Filter Toolbar**: Real-time text search (song title, PM, lineup members) and stage dropdown filtering.
    4. **Add Music Number Modal**: Form for creating new show numbers with title, PM leader, genre, and initial QC reviewer.

### 6.2 Agile Practice SDLC (Software Development Life Cycle for Music)
Each music event is divided into **Practice Sprints** (typically 1 to 2 weeks per sprint) mirroring the Agile SDLC:
1. **Sprint Planning & Free-Time Registration**:
   - **15-Minute Precision Grid**: Members register available time slots for active practice sprints using a fine-grained 15-minute resolution matrix (e.g. 08:00, 08:15, 08:30 up to 22:45).
   - **Click-and-Drag Selection**: Supports intuitive click-and-drag mouse painting across days and 15-minute sub-slots for rapid multi-slot availability entry, alongside preset selection shortcuts (Peak Evenings, Afternoons, Clear).
   - **Auto-Scheduled Sprint Rehearsals Calendar Display & High-Density Multi-View**: Clicking "Auto-Schedule Sprint Rehearsals" triggers CSP optimization to solve practice session assignments. Scheduled rehearsals are rendered on an interactive Calendar Display grid showing exact 15-minute start/end times, assigned music numbers, session index (e.g. Session #1 of 2, #2 of 3), PMs, performer lineups, studio room allocations (e.g. Studio Room A / Studio Room B), and conflict indicators.
   - **High-Density Concert Production Scaling (10–20 Numbers, 1–3 Rehearsals Weekly)**: To support 20–40 weekly scheduled sessions across active numbers without visual overload or page scrolling bloat:
     1. **Rehearsal Quota Tracker**: Real-time summary header displaying target rehearsal fulfillment (e.g. `15/16 Rehearsals Scheduled`, `100% Quota Fulfilled`, `Rooms Utilized`).
     2. **Multi-View Modes**:
        - **Bento Grid View (`view = 'grid'`)**: Visual 7-day card grid with room color indicators, stage badges, and lineup tags.
        - **Compact Timeline Table View (`view = 'timeline'`)**: Dense chronological table grouping by Day/Time with room badges, PM leaders, and performer lineups for DMs/PMs to inspect and manage 20–40 sessions at once.
     3. **Multi-Dimension Filters**: Instant filtering by Day of Week, Music Number Title, or Studio Room.
   - PMs define weekly sprint objectives and requested rehearsal sessions.
2. **Study & Create Tasks**:
   - **Study Task**: Individual member homework (e.g., memorizing vocal melodies, studying guitar chords/tabs, mastering drum fills).
   - **Create Task**: Collaborative arrangement tasks (e.g., recording demo scratch tracks, creating backing tracks, harmonizing vocal parts, band jamming).
3. **Review Task (Quality Check / QC)**:
   - **BR-PRC-01 (Mandatory QC Reviewer Assignment)**: Every review task must have at least one designated QC Reviewer assigned by the PM (or DM).
   - **BR-PRC-02 (Verifiable Quality Verdict)**: A song cannot be approved for stage performance without passing its milestone QC audit. The assigned QC Reviewer submits an explicit decision (`passed`, `in_progress`, or `blocked`) accompanied by constructive critique and feedback notes.
   - **Music Number Lifecycle**: `draft` $\rightarrow$ `in_practice` $\rightarrow$ `ready_for_qc` $\rightarrow$ `qc_approved` $\rightarrow$ `stage_ready`.

### 6.3 Dual-Ownership Instrument Fleet Management & Conflict Invariant
To prevent showstopper rehearsal clashes and equipment loss, the organization maintains a centralized equipment registry:
* **BR-INS-01 (Ownership Classification)**:
  - **CSAC Property (`club_property`)**: Instruments and audio equipment owned by the club (e.g., club drums, stage mics, master keyboards, PA gear).
  - **Member-Owned Gear (`member_owned`)**: Personal instruments brought by members (e.g., member's custom bass guitar, boutique amplifier, synthesizers). The owner is explicitly identified by `owner_user_id`.
* **BR-INS-02 (Borrowing Policy & Status Flags)**:
  - `free_to_borrow`: Available for any music number in the club to reserve during practice or stage sessions.
  - `in_use`: Currently allocated to an ongoing rehearsal or live performance.
  - `unavailable`: Strictly reserved for the owner's personal numbers or private use; not open for general club loan.
  - `in_maintenance`: Damaged, undergoing string replacement, tuning, or repair.
* **BR-INS-03 (Custody & Physical Location Tracking)**:
  - Every piece of gear tracks `custody_user_id` ("kept by whom") or location note (e.g., "Club Studio Locker A", "Kept by Minh Pháp") to ensure total physical accountability and eliminate missing gear after late-night rehearsals.
* **BR-INS-04 (Zero Double-Booking Conflict Invariant)**:
  - A physical instrument **shall never** be concurrently reserved for two different music numbers in the same day and time slot, whether during practice rehearsals or live stage performances. Any conflicting reservation attempt is rejected with a conflict error.

---

## 7. Internationalization (i18n) & Dual-Language Policy

* **Target Audience Inclusion**: CSAC includes performers, mentors, and international exchange members. Consequently, 100% of the platform interface must be natively available in both Vietnamese (`vi`) and English (`en`).
* **Zero Missing Copy Mandate**: All pages—including Studio Hub, User Governance, Quorum Approvals, Event Lifecycle, Member Voting Portal, Workbook Inspector, and Timetable Solver—must provide 100% complete, contextual translations. No raw English strings may leak into the Vietnamese experience, and no Vietnamese strings may leak into the English experience.
* **Persistent User Choice**: The selected language is remembered and synchronized via top-level URL state (`?lang=vi` or `?lang=en`) and machine environment detection, allowing easy sharing and consistent presentation.


