# CSAC Timetable Studio — Pilot User Guide & UI Walkthrough 🎵📅

Welcome to the **CSAC Timetable Studio** pilot testing phase! This guide is created specifically for the **Pilot User Team** (Club Administrators, Delivery Managers, Performance Managers / Song Leads, QC Reviewers, and Performing Club Members).

CSAC Timetable Studio replaces manual spreadsheet scheduling with a modern, **Show-Driven Music Production Workspace** featuring automated CSP timetable optimization, an Agile rehearsal pipeline, and a physical instrument custody fleet.

---

## 📑 Table of Contents

1. [Quick Start & Launching the Web Client](#1-quick-start--launching-the-web-client)
2. [Pilot Test Accounts & Role Hierarchy](#2-pilot-test-accounts--role-hierarchy)
3. [Global Navigation & Interface Controls](#3-global-navigation--interface-controls)
4. [Touring the Show Production Studio (`/studio`)](#4-touring-the-show-production-studio-studio)
   - [4.1 Show Overview (`/studio/shows/[id]/overview`)](#41-show-overview-studioshowsidoverview)
   - [4.2 Music Numbers Pipeline (`/studio/shows/[id]/numbers`)](#42-music-numbers-pipeline-studioshowsidnumbers)
   - [4.3 Agile Practice Sprints & 15-Minute Grid (`/studio/shows/[id]/sprints`)](#43-agile-practice-sprints--15-minute-grid-studioshowsidsprints)
   - [4.4 Show Roster, Band Coverage & Fatigue Tracker (`/studio/shows/[id]/roster`)](#44-show-roster-band-coverage--fatigue-tracker-studioshowsidroster)
5. [Instrument Fleet & Physical Custody Tracking (`/studio/gear`)](#5-instrument-fleet--physical-custody-tracking-studiogear)
   - [5.1 Master Catalog & Lending Policies](#51-master-catalog--lending-policies)
   - [5.2 5-Phase Live Show Custody Checklist](#52-5-phase-live-show-custody-checklist)
   - [5.3 Proxy Retrieval Warning](#53-proxy-retrieval-warning)
   - [5.4 Orphan Gear Lockdown & "Adopt-a-Gear" Protocol](#54-orphan-gear-lockdown--adopt-a-gear-protocol)
6. [Administration & Governance Hub (`/admin/*`)](#6-administration--governance-hub-admin)
   - [6.1 Show Production Creation (`/admin/shows`)](#61-show-production-creation-adminshows)
   - [6.2 Member Onboarding & Self-Service Activation (`/admin/users`)](#62-member-onboarding--self-service-activation-adminusers)
   - [6.3 Multi-Admin Quorum Demotion Protocol (`/admin/approve`)](#63-multi-admin-quorum-demotion-protocol-adminapprove)
7. [Pilot Testing Feedback & Tips](#7-pilot-testing-feedback--tips)

---

## 1. Quick Start & Launching the Web Client

### Starting Local Development Server
To launch the frontend locally on your test machine:
```bash
# In the root repository directory
pnpm run dev
```
Open your browser to: **`http://localhost:5173`** (or port specified in terminal).

### Containerized Full-Stack Environment
If you are running the complete stack via Podman/Docker Compose:
```bash
podman compose -f deploy/compose.yml up -d
```
Access the application at: **`http://localhost:3000`** (or through the API Gateway at `http://localhost:8080`).

---

## 2. Pilot Test Accounts & Role Hierarchy

The application enforces a **scoped, key-based role hierarchy** (Admin $\rightarrow$ Moderator $\rightarrow$ Delivery Manager (DM) $\rightarrow$ Performance Manager (PM) $\rightarrow$ Quality Reviewer (QC) $\rightarrow$ Performer / Member). Action buttons and modals adapt dynamically depending on your user account and active show assignment.

The local database and mock store are pre-seeded with the following pilot accounts:

| User Email | Full Name | Global Role | Scoped Context | Recommended Pilot Test Scenarios |
| :--- | :--- | :--- | :--- | :--- |
| `admin@csac.studio` | System Administrator | **Admin** | System-Wide | User onboarding, show creation, quorum demotion, full control over all numbers. |
| `grace@csac.studio` | Grace | **Delivery Manager (DM)** | Show: Annual Concert | Show roster leadership, triggering auto-schedules, overall stage readiness inspection. |
| `alice@csac.studio` | Alice | **Performance Manager (PM)** | Song Leader (Phonecert) | Assigning band lineup (vocals, guitar, bass, drums), moving song stages, managing practice tasks. |
| `charlie@csac.studio` | Charlie | **Quality Reviewer (QC)** | Quality Auditor | Reviewing music numbers in *Ready for QC*, submitting formal audit verdicts & feedback. |
| `bob@csac.studio` | Bob | **Member / Performer** | Drums & Percussion | Registering 15-minute free time availability, viewing auto-scheduled rehearsals. |
| `diana@csac.studio` | Diana | **Member / Performer** | Keys & Synth | Testing free time matrix, viewing personal timetable. |
| `eve@csac.studio` | Eve | **Member / Performer** | Lead Guitar | Testing gear lending requests and rehearsal timetable. |

> [!TIP]
> **Switching Accounts During Testing**:
> You can sign in at [`/auth/login`](file:///home/ahndunn/dev/csac-timetable/clients/web/src/routes/auth/login/+page.svelte) using any seeded email with any password (offline/mock mode accepts demo input). To test simultaneous multi-user interactions (e.g. Performer submitting free time while DM triggers the scheduler), open an **Incognito / Private Window** or a secondary browser.

---

## 3. Global Navigation & Interface Controls

### 3.1 The Top Navigation Bar (`Navbar.svelte`)
Present at the top of every screen:
* **Brand Logo ("CSAC Studio")**: One-click return to the central Hub (`/`).
* **Main Navigation Links**:
  - **Show Studio** (`/studio`): Access active concert productions and workspaces.
  - **Instrument Fleet** (`/studio/gear`): Dual-ownership gear catalog and show custody tracking.
  - **Admin Shows** (`/admin/shows`): Show management (visible to Admins & Moderators).
  - **Governance** (`/admin/users`): User directory & invitations (Admin only).
* **Language Switcher (🇻🇳 / 🇺🇸)**:
  - Toggle between **Tiếng Việt** and **English** with zero page reload.
  - All labels, tooltips, dialogs, and error messages update immediately.
  - URL query state synchronizes automatically (`?lang=vi` / `?lang=en`).
* **User Profile & Sign Out**:
  - Displays your active role badge (e.g. `Admin`, `DM`, `Member`).
  - Dropdown contains profile info and the **Sign Out** button.

---

## 4. Touring the Show Production Studio (`/studio`)

Access the studio from the top bar or via [`/studio`](file:///home/ahndunn/dev/csac-timetable/clients/web/src/routes/studio/+page.svelte).

On the Studio Hub, click **"Enter Annual Concert 2026"** to enter the flagship pilot production workspace (`/studio/shows/show-2026-annual/overview`).

The show workspace features a banner header displaying production dates, venue, real-time stage readiness (`75% Stage Ready`), and **4 sub-page navigation tabs**:
1. **Overview** (`/overview`)
2. **Music Numbers** (`/numbers`)
3. **Practice Sprints** (`/sprints`)
4. **Show Roster** (`/roster`)

---

### 4.1 Show Overview (`/studio/shows/[id]/overview`)

The Show Overview serves as the executive mission control dashboard for the production:
* **Stage Readiness Funnel**: Visual progress bar tracking the percentage of numbers that have achieved `stage_ready` status.
* **Metric Quick-Filter Pills**: Interactive cards showing totals for *Total Numbers*, *In Practice*, *Ready for QC*, and *Stage Ready*.
  > **Try It**: Click the **"Ready for QC"** metric pill. The app immediately navigates to `/numbers?stage=ready_for_qc` with the filter pre-applied!
* **Production Milestones**: Chronological timeline showing milestone deadlines (Song Selection $\rightarrow$ Lineup Lock $\rightarrow$ Mid-Sprint QC $\rightarrow$ Dress Rehearsal $\rightarrow$ Live Concert).
* **Recent Activity Feed**: Real-time log of song status transitions, lineup edits, and audit verdicts.

---

### 4.2 Music Numbers Pipeline (`/studio/shows/[id]/numbers`)

This screen manages the 10–20 songs comprising the concert setlist.

#### A. Three Switchable View Modes
In the upper toolbar, toggle between:
1. **Bento Grid View (Default)**: Visual cards displaying song genre, assigned Performance Manager (PM), Quality Reviewer (QC), band lineup avatar chips, and responsive action buttons.
2. **Kanban Board View**: 5-column stage progression board (`Draft` $\rightarrow$ `In Practice` $\rightarrow$ `Ready for QC` $\rightarrow$ `QC Approved` $\rightarrow$ `Stage Ready`).
3. **Compact Table View**: High-density table designed for DMs and PMs to scan and audit all numbers simultaneously without scrolling fatigue.

#### B. Pipeline Funnel Filter & Search
* Click any stage pill in the top filter bar (`Draft (1)`, `In Practice (2)`, `Ready for QC (1)`, etc.) to instantly filter the setlist.
* Type in the live **Search** box to search across song titles, PM names, or performer names.

#### C. Testing the Music Number Lifecycle
Follow this step-by-step workflow as a pilot tester:

1. **Add a New Number**:
   - Click the **"+ New Music Number"** button.
   - Enter Title (e.g. *"Nàng Thơ"*), Genre (*Acoustic Pop*), PM Leader (*Alice*), and initial QC Reviewer (*Charlie*).
   - Click **"Create Music Number"**. The song appears in the `Draft` stage.

2. **Assign the Band Lineup**:
   - On the song card, click **"Assign Lineup"**.
   - Select performers for the 5 core musical positions:
     - **Vocal Lead**
     - **Lead Guitar**
     - **Bass Guitar**
     - **Drum Kit**
     - **Keyboards / Synth**
   - Click **"Save Lineup"**. The assigned performer avatars appear on the card.

3. **Advance to Practice**:
   - Click **"Start Practice"**. The status moves from `draft` to `in_practice`.

4. **Submit for Quality Audit**:
   - Once rehearsals begin, click **"Submit for QC"**. The status moves to `ready_for_qc`.

5. **Conduct QC Review (Role: Charlie / QC Auditor)**:
   - Click the **"Audit & Submit QC"** button on a song in `ready_for_qc`.
   - The **QC Audit Workstation Modal** opens.
   - Choose a verdict:
     - **Pass (Approve for Stage)**: Confirms vocal harmony, rhythm precision, and overall polish. Song transitions to `qc_approved` or `stage_ready`.
     - **Revision Requested**: Requires further practice. Input constructive feedback in the notes field (e.g., *"Bridge transition timing needs tighter drums-bass sync"*). Clicking submit returns the song to `in_practice` with the notes attached!

6. **Deep Link to Sprint Timetable**:
   - On any song card, click the link icon or **"View in Sprint Calendar"**. The system jumps to `/sprints?song=[Title]`, immediately filtering the rehearsal timetable for that specific song.

---

### 4.3 Agile Practice Sprints & 15-Minute Grid (`/studio/shows/[id]/sprints`)

This is the automated scheduling engine where performers submit free time and the system generates zero-conflict rehearsal slots.

#### A. 15-Minute Precision Free-Time Matrix
* **Precision Granularity**: Displays days of the week (Monday through Sunday) across 15-minute increments from **08:00 to 22:45**.
* **Ergonomic Soft-Tint Palette**: Designed with an eye-friendly soft brand tint (`bg-primary/15` with micro-dot indicators) to prevent visual fatigue over dense grids.
* **2D Bounding-Box Mouse Drag Selection**:
  - Click and hold on any cell (e.g. *Monday 18:00*).
  - Drag diagonally across multiple days and times (e.g. down to *Tuesday 20:30*).
  - A real-time bounding box highlights the selected area.
  - Release the mouse button to toggle all enclosed 15-minute sub-slots at once!
* **Quick Presets**:
  - **"Peak Evenings"**: 1-click select 18:00–21:00 across all weekdays.
  - **"Afternoons"**: 1-click select 13:00–17:00 on weekends.
  - **"Clear All"**: Reset availability to blank.
* **Save Availability**:
  - Click **"Save Availability"**. A success toast confirms the total registered hours.

#### B. Auto-Scheduling Sprint Rehearsals (Role: Grace / DM or Admin)
1. In the Rehearsal Schedule header, click **"Auto-Schedule Sprint Rehearsals"**.
2. The CSP Solver executes:
   - Guarantees **Zero Double-Booking**: No performer is scheduled in two different rooms or songs at the same time.
   - Allocates dedicated rehearsal studios (e.g., *Studio Room A*, *Studio Room B*).
   - Enforces 100% full band attendance per session.
3. The generated sessions are displayed instantly in two view options:
   - **Bento Calendar View (`view = 'grid'`)**: 7-day visual card layout with room badges, session indices (e.g. *Session #1 of 2*), and performer chips.
   - **Compact Timeline View (`view = 'timeline'`)**: Chronological table grouping sessions by day, time, assigned room, and song title.
4. **Rehearsal Quota Tracker**:
   - Real-time header metrics indicate scheduled sessions vs. weekly targets (e.g. `15/16 Rehearsals Scheduled • 100% Quota Fulfilled`).
5. **History Drawer**:
   - Click the **History** button to open the audit drawer.
   - Toggle between **Compute Runs** (solver duration, score, conflict count) and **Registration History** (member slot change audit trail).

---

### 4.4 Show Roster, Band Coverage & Fatigue Tracker (`/studio/shows/[id]/roster`)

Manage the production cast and protect performers from burnout.

#### A. Roster Directory & Role Delegation
* Displays all performers attached to the show with their contact details, primary instrument, and assigned songs.
* **Scoped Leadership Badges**: Clearly flags members as `Show DM`, `PM (2 Songs)`, `QC (1 Song)`, or `Performer`.
* **Add / Edit Cast Member**:
  - Authorized leadership can click **"+ Add Cast Member"** to onboard a new musician or edit their instrument doubling capability.

#### B. Band Role Coverage Matrix
At the top of the roster, check the real-time instrumentation coverage:
* **Vocals**: Lead Vocal & Harmony counts.
* **Strings**: Lead Guitar, Rhythm Guitar, Bass Guitar counts.
* **Rhythm**: Drum Kit & Percussion counts.
* **Keys & Tech**: Stage Piano/Synth and Sound Engineers.
> Any deficient musical section is visually flagged so leadership can recruit before rehearsals begin.

#### C. Fatigue Risk & Workload Warning
The system automatically monitors performer workload:
* **Optimal Workload (1–2 Songs)**: Safe green status.
* **Moderate Workload (3–4 Songs)**: Balanced load.
* **Fatigue Alert (5+ Songs or $\ge 20$ practice hours)**:
  - The member card displays a high-visibility **"Fatigued"** warning badge.
  - Cross-linked to song cards in the Numbers page to alert PMs not to overload the performer.

---

## 5. Instrument Fleet & Physical Custody Tracking (`/studio/gear`)

Navigate to [`/studio/gear`](file:///home/ahndunn/dev/csac-timetable/clients/web/src/routes/studio/gear/+page.svelte) via the top navigation bar.

University clubs frequently suffer from damaged or misplaced equipment. CSAC Timetable Studio provides physical custody tracking with strict conflict invariants.

The gear interface is organized into **3 main tabs**:
1. **Master Catalog** (`catalog`)
2. **Show Checklist** (`show_checklist`)
3. **Orphan & Recovery** (`orphans`)

---

### 5.1 Master Catalog & Lending Policies
* **Dual Ownership Distinction**:
  - 🏛️ **Club Property**: Official club assets (e.g., *Fender Player Stratocaster*, *Yamaha Drum Set*, *Roland Stage Piano*).
  - 👤 **Member-Owned Gear**: Personally owned instruments lent by club members (e.g., *Yamaha TRBX504 Bass* owned by Hoàng Nam).
* **Owner Lending Policies**:
  - `open_to_all`: Freely reservable by any club rehearsal.
  - `approval_required`: Requires explicit confirmation from the owner.
  - `show_only`: Available solely for official show rehearsals and live stages.
  - `locked_private`: Visible in registry but strictly reserved for the owner's personal performances.
* **Physical Custody Tracking**:
  - Every piece of gear explicitly states **Custodian Name** ("Kept by whom") and physical location (e.g., *"Studio Locker A"*, *"Kept by Minh Pháp"*).
* **Zero Double-Booking Invariant**: An instrument cannot be booked for two simultaneous rehearsals in different rooms.

---

### 5.2 5-Phase Live Show Custody Checklist
For live concerts, click the **"Show Run Sheet & Custody Checklist"** tab to monitor all allocated gear through 5 distinct operational stages:

$$\text{1. Allocated} \longrightarrow \text{2. Present at Venue} \longrightarrow \text{3. Active on Stage} \longrightarrow \text{4. Teardown / Retrieved} \longrightarrow \text{5. Returned to Safe Custody}$$

* **Advancing Phases**: Click the action button on any gear item (e.g. click **"Mark Present"**, then **"Deploy to Stage"**, then **"Mark Teardown"**).
* **Owner Revocation**: Instrument owners retain the right to revoke gear if personal circumstances change. Revoking flags the item with a replacement warning to the Show DM.

---

### 5.3 Proxy Retrieval Warning
During post-show teardown, members frequently collect gear on behalf of a friend.
1. On an item in teardown, choose **"Retrieve on Behalf / Proxy"**.
2. Input the proxy retriever's name and note (e.g. *"Gia Huy picked up Hoàng Nam's bass pedalboard for safe transport"*).
3. The item displays a distinct **"Proxy Retrieval"** badge, preventing disputes over who has physical possession of the equipment.

---

### 5.4 Orphan Gear Lockdown & "Adopt-a-Gear" Protocol
Click the **"Orphan Gear & Recovery"** tab.
* **Orphan Status**: Any equipment left unclaimed during post-show teardown is classified as `orphan`.
* **Show Finalization Hard Lock**: While any gear item remains in `orphan` status, the **"Close Show / Finalize Production"** feature is strictly disabled. The show cannot be archived until all gear is accounted for!
* **Adopt-a-Gear Workflow**:
  - A responsible team member can click **"Adopt Gear"** to take it home temporarily.
  - The modal collects the adopter's full name, phone number, and agreed return date.
  - The system tracks custody and clears the show finalization block.

---

## 6. Administration & Governance Hub (`/admin/*`)

*(Accessible to users with Admin or Moderator roles)*

---

### 6.1 Show Production Creation (`/admin/shows`)
* View all active and upcoming music shows.
* Click **"+ Create Music Show"** to launch a new production workspace:
  - Enter Show Title, Description, Venue, Date Range, and Target Number Quota.
  - Submitting creates the new show and initializes its `/studio/shows/[id]/*` route structure immediately.

---

### 6.2 Member Onboarding & Self-Service Activation (`/admin/users`)

CSAC employs a **Zero-Friction Invitation** pattern:

1. **Inviting a Member (Admin Flow)**:
   - Go to [`/admin/users`](file:///home/ahndunn/dev/csac-timetable/clients/web/src/routes/admin/users/+page.svelte) and click **"+ Invite User"**.
   - **Email** is the only mandatory field. Full Name and Initial Role are optional.
   - Click **"Send Invitation"**. The user is created in `pending_activation` status, and an activation token + 6-digit OTP are issued.

2. **Activating the Account (New Member Flow)**:
   - Navigate to [`/auth/activate`](file:///home/ahndunn/dev/csac-timetable/clients/web/src/routes/auth/activate/+page.svelte).
   - Enter your email and the 6-digit OTP received via email (in mock test mode, any 6-digit code e.g. `123456` will pass).
   - Complete your profile: verify prefilled details and set a secure password (minimum 8 characters with upper, numeric, and symbol strength validation).
   - Click **"Activate Account"**. You are logged in immediately and redirected to the Studio Hub!

3. **Dynamic Role Promotion**:
   - Admins can instantly promote any Member to Moderator or Admin via the user action menu in the directory table.

---

### 6.3 Multi-Admin Quorum Demotion Protocol (`/admin/approve`)

To prevent unilateral hostile takeovers or accidental lockouts, downgrading an Administrator requires **multi-admin quorum consensus**:

1. **Initiating Demotion**:
   - In `/admin/users`, an Admin selects an Admin user and clicks **"Propose Role Downgrade"**.
   - The required approval count $M$ is calculated as $M = \min(\lceil N/2 \rceil, 3)$ where $N$ is the active Admin count.
2. **Reviewing and Voting at [`/admin/approve`](file:///home/ahndunn/dev/csac-timetable/clients/web/src/routes/admin/approve/+page.svelte)**:
   - Peer Admins navigate to the Quorum Approval portal.
   - Select the pending proposal and click **"Request OTP"** (sent to the peer Admin's email, TTL 10 minutes).
   - Enter the 6-digit OTP and submit either **"Approve Demotion"** or **"Reject"**.
   - Once $M$ approvals are collected, the target user's role is safely updated in PostgreSQL.

---

## 7. Pilot Testing Feedback & Tips

### Key Test Scenarios to Verify
When exploring the client application, please prioritize testing these core workflows:
* [ ] **Cross-Screen Deep Linking**: Test clicking stage metric pills on Overview $\rightarrow$ verify Numbers page filters correctly; click song links in Numbers $\rightarrow$ verify Sprints page filters by song.
* [ ] **15-Minute Grid Interaction**: Test click-and-drag 2D rectangular box selection across multiple days and verify responsiveness.
* [ ] **QC Workflow**: Move a song to `ready_for_qc`, open the QC audit modal as Charlie, and verify that choosing "Revision" returns the song to `in_practice` with critique notes.
* [ ] **Gear Checklist**: Allocate an instrument to a show, move it through all 5 custody phases, test a proxy retrieval, and test the orphan adoption dialog.
* [ ] **Language Consistency**: Toggle to Vietnamese (🇻🇳) and verify that no untranslated text strings or broken layout clippings occur.

### Running Verification Tests Locally
If you have node/pnpm installed, you can run the full automated test suite to confirm backend and frontend invariants:
```bash
# Runs all 109 automated assertions (CSP solver, parser, RBAC, FSM, and i18n checks)
pnpm run test

# Verifies production build
pnpm run build
```

### Submitting Feedback & Reporting Bugs
If you encounter visual clipping, unintuitive workflows, or scheduling anomalies during pilot testing, please note:
1. The active URL / route where the issue occurred.
2. Your active user account and role.
3. Steps to reproduce the issue.
4. Language mode (VI / EN) and browser engine (e.g. Firefox, Chrome, Safari).

Thank you for helping test and refine **CSAC Timetable Studio**! 🎵🚀
