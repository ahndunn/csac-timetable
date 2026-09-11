# Technical Design Specification: Show-Driven Architecture & UI Streamlining

**Project**: CSAC Timetable Studio 🎵📅  
**Document Status**: Approved Design Spec / SSOT  
**Date**: September 2026  
**Author**: PO, SA, Designer, Developer, QC & User Team  

---

## 1. Executive Summary & Goals

This specification details the structural evolution of **CSAC Timetable Studio** into a **Show-Driven Architecture**. All app functionalities and rehearsal workflows are centered around **Music Shows** (e.g., *CSAC Annual Concert 2026*, *Acoustic Night Vol. 4*).

### Key Business & Technical Goals
1. **Show-Driven Pivot**: Refactor `/studio` to be structured around active Music Shows and their sub-pages (`/studio/shows/[id]/*`).
2. **Independent Equipment Management**: Establish `/studio/gear` as an independent, concise route for Instrument Fleet & Custody Management, decoupled from nested show sub-pages.
3. **Admin Show Studio**: Implement `/admin/shows` for Show Creation, Design, Capacity Planning, and Live Monitoring.
4. **Header UX Clean-Up**: Remove the redundant top-left "CSAC Studio" dropdown button from `Navbar.svelte`. Replace with static branding and direct, uncluttered top-bar navigation.
5. **Legacy Elimination**: Completely purge legacy, non-show routes (`/events/*`, `/utils/*`). All historical context is archived in legacy documentation.

---

## 2. Route Topology & Page Specifications

### 2.1 Route Map

```
clients/web/src/routes/
├── +layout.svelte                          (Global top-bar: Brand title, direct nav, i18n, profile)
├── +page.svelte                            (Landing page redirecting to /studio or active shows)
├── admin/
│   ├── shows/                              (Admin Show Design, Creation & Live Monitoring)
│   │   └── +page.svelte
│   ├── users/                              (User Directory & Onboarding)
│   │   └── +page.svelte
│   └── approve/                            (Multi-Admin Quorum Demotion Approval)
│       └── +page.svelte
├── studio/
│   ├── +page.svelte                        (Studio Home & Active Shows Directory)
│   ├── gear/                               (Independent Instrument Fleet & Custody Management)
│   │   └── +page.svelte
│   └── shows/
│       └── [id]/                           (Show Deep Workspace)
│           ├── +layout.svelte              (Show Header & Sub-Page Navigation Tabs)
│           ├── overview/                   (Show Readiness Heatmap, Key Milestones & Lineup)
│           ├── numbers/                    (Music Numbers / Songs Kanban Board & QC Approval)
│           ├── sprints/                    (Practice Sprint Planning & 1-Click Free-Time Grid)
│           └── roster/                     (Show Band Members & Assigned Roles)
└── auth/
    └── login/                              (Authentication)
```

---

## 3. Sub-Page Functional Specifications & Acceptance Criteria (ACs)

### 3.1 Global Top Bar Clean-up (`Navbar.svelte`)
* **Specification**:
  * Remove top-left brand dropdown button and menu.
  * Static emblem: `Sparkles` icon + "CSAC Studio" text.
  * Desktop top links: `Studio` (`/studio`), `Instrument Fleet` (`/studio/gear`), and for Admins: `Admin Shows` (`/admin/shows`), `Users` (`/admin/users`), `Approvals` (`/admin/approve`).
  * Mobile drawer updated to match top-level routes.
* **AC-NAV-01**: Top-left button is unclickable. Navigation links route directly to target pages without dropdown steps.

### 3.2 Admin Show Studio (`/admin/shows`)
* **Specification**:
  * **Create Show Modal**: Input Title, Description, Date Range (Start/End), Venue, Target Number Count.
  * **Monitor Dashboard**: Metrics cards for total scheduled practice hours, QC pass rate %, equipment reservation conflicts, and rehearsal bottlenecks.
* **AC-ADM-SHOW-01**: Admins and Moderators can create and edit music shows.
* **AC-ADM-SHOW-02**: Live monitor dashboard updates dynamically with total numbers and active sprint metrics.

### 3.3 Studio Shows Sub-Pages (`/studio/shows/[id]/*`)

#### 3.3.1 Layout & Sub-Nav (`/studio/shows/[id]/+layout.svelte`)
* Shows show banner, date range, overall readiness progress bar, and 4 sub-page tabs:
  - `Overview` (`/studio/shows/[id]/overview`)
  - `Music Numbers` (`/studio/shows/[id]/numbers`)
  - `Practice Sprints` (`/studio/shows/[id]/sprints`)
  - `Show Roster` (`/studio/shows/[id]/roster`)

#### 3.3.2 Music Numbers (`/studio/shows/[id]/numbers`)
* **Kanban Stages**: `Draft` $\rightarrow$ `In Practice` $\rightarrow$ `Ready for QC` $\rightarrow$ `QC Approved` $\rightarrow$ `Stage Ready`.
* **QC Verdict Drawer**: Assigned QC Reviewer selects decision (`passed`, `in_progress`, `blocked`) and leaves mandatory feedback notes.
* **AC-NUM-01**: Song cards can be moved across practice stages.
* **AC-NUM-02**: Advancing to `QC Approved` requires an explicit QC verdict from an assigned QC Reviewer.

#### 3.3.3 Practice Sprints (`/studio/shows/[id]/sprints`)
* **1-Click Free-Time Grid**: Performers toggle their weekly availability slots with simple clicks or drag selection.
* **Sprint Rehearsal Schedule**: Auto-generates conflict-free rehearsal sessions for show numbers.
* **AC-SPR-01**: Member free-time registration updates instant reactivity.
* **AC-SPR-02**: Zero double-booking invariant enforced across all performer schedules.

#### 3.3.4 Show Roster (`/studio/shows/[id]/roster`)
* Displays Delivery Manager (DM), Performance Managers (PMs), Band Members, Instrument Roles, and overall attendance stats.
* **AC-ROS-01**: Filter roster by music number or instrument role.

### 3.4 Independent Instrument Fleet & Custody (`/studio/gear`)
* **Specification**:
  * Accessible directly via `/studio/gear`.
  * Classifies gear by ownership: `CSAC Property` (`club_property`) vs `Member-Owned` (`member_owned`).
  * Custody tracking: Displays current custodian (`custody_user_id`), physical location note (e.g. "Studio Locker B"), and status (`free_to_borrow`, `in_use`, `unavailable`, `in_maintenance`).
* **AC-GEAR-01**: Gear can be searched and filtered by ownership type and custody status.
* **AC-GEAR-02**: Custody location and custodian can be updated with single-click modal.
* **AC-GEAR-03**: Instrument reservation conflicts produce clear error notices.

---

## 4. Legacy Code Elimination Plan

### Deleted Files/Folders:
- `clients/web/src/routes/events/*`
- `clients/web/src/routes/utils/*`

### Updated References:
- `clients/web/src/lib/i18n/locales/en.json` & `vi.json` updated with new navigation and sub-page keys.
- All 38 automated assertions in `clients/web/test_suite.ts` updated to target new route topology.

---

## 5. Verification & Quality Assurance

1. `pnpm run test` must pass 100% of test assertions.
2. `pnpm run build` must complete cleanly with zero SvelteKit route or TypeScript errors.
