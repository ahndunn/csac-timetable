# Technical Design Specification: Show Management & Resource Allocation Features

**Project**: CSAC Timetable Studio 🎵📅  
**Document Status**: Approved Design Spec / SSOT  
**Date**: September 2026  
**Author**: PO, System Architect, UI Designer & Dev Team  

---

## 1. Executive Summary & Core Objectives

This specification details the deep features, management modals, and UX architecture for managing **Music Shows** across `/admin/shows` and `/studio/shows/[id]/*`. 

### Key Goals
1. **Strict Governance QC FSM**: Implement a 5-stage Finite State Machine for Music Numbers (`Draft` $\rightarrow$ `In Practice` $\rightarrow$ `Ready for QC` $\rightarrow$ `QC Approved` $\rightarrow$ `Stage Ready`). Enforce role-restricted transitions where only assigned Admins/Moderators/QC Reviewers can approve or fail a number (returning `Ready for QC` back to `In Practice` with mandatory feedback notes).
2. **Member Resource Allocation (RA) & Fatigue Analytics**: Track member involvement across music numbers, displaying soft workload warning indicators (`Optimal`, `Moderate`, `Fatigued`) based on active song counts and rehearsal hours.
3. **Show Roster & Song Lineup Management Modals**: Empower authorized users (Admins, Delivery Managers, Performance Managers) to add/edit/remove Show Roster members, assign primary instruments, and map roster performers to specific song roles (`Vocal Lead`, `Bass`, `Drums`, etc.).
4. **Decoupled Bento-Grid Hub**: Seamlessly integrate `/admin/shows` (executive management & capacity creation) with `/studio/shows/[id]/*` (operational show deep workspaces).
5. **Localization (i18n)**: Fully support bilingual strings (English `en` & Vietnamese `vi`) across all show management UI surfaces.

---

## 2. Technical Data Contracts & Governance Scope

### 2.1 Role-Based Management Scope

| Role | Scope & Permissions | Modals / Actions |
| :--- | :--- | :--- |
| **Admin & Delivery Manager (DM)** | Full Show Governance | Create/Edit Show, Add/Edit/Remove Roster Members, Assign Show Roles (`DM`/`PM`/`Performer`), Assign Song Roles, Perform QC Audits. |
| **Performance Manager (PM)** | Song & Lineup Operations | Add/Edit Music Numbers, Assign Roster Performers to Song Roles, Manage Practice Status, Submit for QC Audit. |
| **Performer / Member** | Practice Participation | Submit 1-Click Free Time, Advance Song Status (`Draft` $\rightarrow$ `In Practice` $\rightarrow$ `Ready for QC`), View Roster & Lineups. |

---

### 2.2 Data Types (`clients/web/src/lib/types/timetable.ts`)

```typescript
export type MusicNumberStatus = 
  | 'draft' 
  | 'in_practice' 
  | 'ready_for_qc' 
  | 'qc_approved' 
  | 'stage_ready';

export type BandRole = 
  | 'vocal_lead' 
  | 'vocal_harmony' 
  | 'guitar_lead' 
  | 'guitar_rhythm' 
  | 'bass' 
  | 'keys' 
  | 'drums' 
  | 'percussion' 
  | 'sound_tech';

export type ShowManagementRole = 'DM' | 'PM' | 'Performer';

export interface QCVerdict {
  reviewedBy: string;
  reviewedAt: string;
  decision: 'passed' | 'revision_requested';
  feedbackNotes: string;
  actionItems?: string[];
}

export interface RosterMember {
  userId: string;
  fullName: string;
  email: string;
  role: ShowManagementRole;
  primaryRole: BandRole;
  assignedNumberIds: string[];
  totalPracticeHours: number;
  workloadStatus: 'optimal' | 'moderate' | 'fatigued'; // Green (1-2), Yellow (3-4), Red (5+)
}

export interface MusicNumber {
  id: string;
  showId: string;
  title: string;
  originalArtist: string;
  status: MusicNumberStatus;
  rolesRequired: BandRole[];
  assignedMembers: Record<string, string>; // BandRole -> userId
  qcHistory: QCVerdict[];
  notes?: string;
}
```

---

## 3. Workflow & Modals Specification

### 3.1 Assign Member to Show Roster Modal (`/studio/shows/[id]/roster`)
* **Trigger**: "+ Add Member to Roster" button (Admins & DMs).
* **Form Inputs**:
  - Member Picker: Directory user search/select.
  - Show Management Role: `Delivery Manager (DM)`, `Performance Manager (PM)`, `Performer`.
  - Primary Band Instrument: `Vocal Lead`, `Guitar Lead`, `Bass`, `Drums`, etc.
* **Output**: Appends new `RosterMember` to show state and updates roster grid & workload health cards.

### 3.2 Edit / Remove Roster Member Modal (`/studio/shows/[id]/roster`)
* **Trigger**: "Edit" / "Remove" buttons on member card.
* **Function**: Update management role, change instrument, or remove member from show.

### 3.3 Song Lineup Role Assignment Drawer (`/studio/shows/[id]/numbers`)
* **Trigger**: "Assign Band Lineup" button on Song Card (Admins, DMs, PMs).
* **Function**: Slide-over drawer listing required instrument roles for the song. Provides dropdowns listing roster members to map performers to roles (`Vocal Lead` $\rightarrow$ Minh Pháp, `Bass` $\rightarrow$ Bảo Anh).

---

## 4. Verification & Test Plan

1. **Automated Test Suite (`clients/web/test_suite.ts`)**:
   - `verifyMusicNumberFSM()`: Tests FSM transitions and QC authority.
   - `verifyResourceAllocationWorkload()`: Asserts correct calculation of `workloadStatus` given assigned number counts.
   - `verifyRosterManagement()`: Verifies roster member addition, role edits, and song role mapping.
2. **Build Verification**:
   - `pnpm run test` must execute 100% cleanly with zero assertion errors.
   - `pnpm run build` must produce a clean SvelteKit production build without TypeScript or SSR errors.
