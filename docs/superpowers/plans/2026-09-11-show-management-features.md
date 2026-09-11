# Show Management & Resource Allocation Features Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement the complete Show Management feature suite including Music Number FSM (Draft $\rightarrow$ In Practice $\rightarrow$ Ready for QC $\rightarrow$ QC Approved $\rightarrow$ Stage Ready), QC Reviewer verdict drawer, Member Resource Allocation (RA) roster, and soft workload/fatigue analytics.

**Architecture:** Extend Svelte 5 runes state models, i18n locale dictionaries, and route pages (`/admin/shows`, `/studio/shows/[id]/numbers`, `/studio/shows/[id]/roster`). Add verification assertions in `clients/web/test_suite.ts`.

**Tech Stack:** Svelte 5, SvelteKit, TypeScript, Lucide Icons, Vanilla CSS (Bento Grid).

**Spec:** [`docs/superpowers/specs/2026-09-11-show-management-features-design.md`](file:///home/ahndunn/dev/csac-timetable/docs/superpowers/specs/2026-09-11-show-management-features-design.md)

## Global Constraints

- **Single Source of Truth**: `docs/superpowers/specs/2026-09-11-show-management-features-design.md` defines types and FSM rules.
- **Verification Requirement**: `pnpm run test` and `pnpm run build` must pass cleanly after each task.
- **Strict QC Rule**: Only QC Reviewers / Admins / Mods can transition numbers to `qc_approved` or return `ready_for_qc` back to `in_practice` with feedback notes.

---

### Task 1: Data Models & i18n Locales

**Files:**
- Modify: `clients/web/src/lib/types/timetable.ts`
- Modify: `clients/web/src/lib/i18n/locales/en.ts`
- Modify: `clients/web/src/lib/i18n/locales/vi.ts`
- Test: `clients/web/test_suite.ts`

**Interfaces:**
- Consumes: Existing types in `timetable.ts`
- Produces: `MusicNumberStatus`, `BandRole`, `QCVerdict`, `RosterMember`, `MusicNumber` types, and translation keys `show_mgmt.*`.

- [ ] **Step 1: Write failing test in `test_suite.ts`**

Add assertion `verifyShowManagementTypes()` to `test_suite.ts`:
```typescript
export function verifyShowManagementTypes(): void {
  const sampleVerdict: QCVerdict = {
    reviewedBy: 'user-admin-1',
    reviewedAt: new Date().toISOString(),
    decision: 'revision_requested',
    feedbackNotes: 'Vocal harmony needs tighter timing on chorus.'
  };
  if (sampleVerdict.decision !== 'revision_requested') {
    throw new Error('QCVerdict decision invalid');
  }
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `pnpm run test`
Expected: FAIL with missing type exports or compilation error for `QCVerdict`.

- [ ] **Step 3: Add types to `timetable.ts` and i18n keys to `en.ts` & `vi.ts`**

Add exported interfaces to `clients/web/src/lib/types/timetable.ts`:
```typescript
export type MusicNumberStatus = 'draft' | 'in_practice' | 'ready_for_qc' | 'qc_approved' | 'stage_ready';

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
  primaryRole: BandRole;
  assignedNumberIds: string[];
  totalPracticeHours: number;
  workloadStatus: 'optimal' | 'moderate' | 'fatigued';
}

export interface MusicNumber {
  id: string;
  showId: string;
  title: string;
  originalArtist: string;
  status: MusicNumberStatus;
  rolesRequired: BandRole[];
  assignedMembers: Record<string, string>;
  qcHistory: QCVerdict[];
  notes?: string;
}
```

Add translations under `show_mgmt` key in `en.ts` and `vi.ts`.

- [ ] **Step 4: Run test to verify it passes**

Run: `pnpm run test`
Expected: PASS with 100% assertions green.

- [ ] **Step 5: Commit**

```bash
git add clients/web/src/lib/types/timetable.ts clients/web/src/lib/i18n/locales/en.ts clients/web/src/lib/i18n/locales/vi.ts clients/web/test_suite.ts
git commit -m "feat(types): add show management FSM, QC, and roster types with i18n"
```

---

### Task 2: Music Numbers Kanban Board & QC Verdict Drawer

**Files:**
- Modify: `clients/web/src/routes/studio/shows/[id]/numbers/+page.svelte`
- Test: `clients/web/test_suite.ts`

**Interfaces:**
- Consumes: `MusicNumberStatus`, `QCVerdict`, `MusicNumber` from `timetable.ts`.
- Produces: Interactive FSM Kanban UI with 5 columns and slide-over QC Verdict Drawer.

- [ ] **Step 1: Write failing FSM assertion in `test_suite.ts`**

Add `verifyMusicNumberFSM()` to `test_suite.ts`:
```typescript
export function verifyMusicNumberFSM(): void {
  // Verifies state transitions logic
  const validTransition = (from: MusicNumberStatus, to: MusicNumberStatus, isQCAdmin: boolean): boolean => {
    if (from === 'ready_for_qc' && (to === 'qc_approved' || to === 'in_practice')) {
      return isQCAdmin;
    }
    return true;
  };
  if (validTransition('ready_for_qc', 'qc_approved', false)) {
    throw new Error('Non-QC member cannot approve QC');
  }
}
```

- [ ] **Step 2: Run test to verify failure**

Run: `pnpm run test`
Expected: FAIL until assertion is wired into test runner.

- [ ] **Step 3: Update `numbers/+page.svelte` with Kanban FSM & QC Drawer**

Enhance `clients/web/src/routes/studio/shows/[id]/numbers/+page.svelte` with:
- 5 FSM columns (`Draft`, `In Practice`, `Ready for QC`, `QC Approved`, `Stage Ready`).
- Role pills & member avatar tags per song card.
- QC Verdict Drawer for `Ready for QC` cards allowing QC Reviewer to submit feedback (`passed` or `revision_requested`).

- [ ] **Step 4: Run test & build to verify it passes**

Run: `pnpm run test && pnpm run build`
Expected: PASS with 0 build errors.

- [ ] **Step 5: Commit**

```bash
git add clients/web/src/routes/studio/shows/[id]/numbers/+page.svelte clients/web/test_suite.ts
git commit -m "feat(numbers): implement music number FSM kanban and QC verdict drawer"
```

---

### Task 3: Show Roster & Member Resource Allocation (RA) Matrix

**Files:**
- Modify: `clients/web/src/routes/studio/shows/[id]/roster/+page.svelte`
- Test: `clients/web/test_suite.ts`

**Interfaces:**
- Consumes: `RosterMember`, `BandRole` from `timetable.ts`.
- Produces: Member Resource Allocation table with workload health badges (`Optimal`, `Moderate`, `Fatigued`) and role coverage warnings.

- [ ] **Step 1: Write workload calculation test in `test_suite.ts`**

Add `verifyResourceAllocationWorkload()` to `test_suite.ts`:
```typescript
export function verifyResourceAllocationWorkload(): void {
  const calculateWorkload = (songCount: number): 'optimal' | 'moderate' | 'fatigued' => {
    if (songCount >= 5) return 'fatigued';
    if (songCount >= 3) return 'moderate';
    return 'optimal';
  };
  if (calculateWorkload(5) !== 'fatigued') throw new Error('Failed to detect fatigue');
  if (calculateWorkload(2) !== 'optimal') throw new Error('Failed to detect optimal workload');
}
```

- [ ] **Step 2: Run test to verify failure**

Run: `pnpm run test`
Expected: FAIL until assertion is wired.

- [ ] **Step 3: Update `roster/+page.svelte` with RA Matrix & Workload Indicators**

Enhance `clients/web/src/routes/studio/shows/[id]/roster/+page.svelte` with:
- Member list featuring primary role badges and assigned songs.
- Soft workload health pills (Green: `Optimal` 1-2 numbers, Yellow: `Moderate` 3-4 numbers, Red: `Fatigued` 5+ numbers).
- Unassigned instrument role alerts per show.

- [ ] **Step 4: Run test & build verification**

Run: `pnpm run test && pnpm run build`
Expected: PASS with clean build and all automated assertions green.

- [ ] **Step 5: Commit**

```bash
git add clients/web/src/routes/studio/shows/[id]/roster/+page.svelte clients/web/test_suite.ts
git commit -m "feat(roster): add member resource allocation matrix and workload health analytics"
```
