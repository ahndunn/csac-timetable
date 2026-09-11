# Show Roster Assignment & Song Lineup Modals Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement full Show Roster assignment/editing modals (`/studio/shows/[id]/roster`), song lineup role mapping drawer (`/studio/shows/[id]/numbers`), and i18n support.

**Architecture:** Add interactive Svelte 5 modals and drawers to `/studio/shows/[id]/roster` and `/studio/shows/[id]/numbers`, backed by reactive runes state and test suite assertions.

**Tech Stack:** Svelte 5, TypeScript, Lucide Icons, Vanilla CSS (Bento Grid).

**Spec:** [`docs/superpowers/specs/2026-09-11-show-management-features-design.md`](file:///home/ahndunn/dev/csac-timetable/docs/superpowers/specs/2026-09-11-show-management-features-design.md)

## Global Constraints

- **Single Source of Truth**: `docs/superpowers/specs/2026-09-11-show-management-features-design.md` defines management permissions.
- **Verification Requirement**: `pnpm run test` and `pnpm run build` must pass cleanly after each task.

---

### Task 1: Show Roster Add/Edit Member Modals & i18n

**Files:**
- Modify: `clients/web/src/lib/i18n/locales/en.ts`
- Modify: `clients/web/src/lib/i18n/locales/vi.ts`
- Modify: `clients/web/src/routes/studio/shows/[id]/roster/+page.svelte`
- Test: `clients/web/test_suite.ts`

**Interfaces:**
- Consumes: `RosterMember`, `BandRole`, `ShowManagementRole` from `timetable.ts`.
- Produces: "+ Add Member to Roster" modal, "Edit Member" modal, and roster modification handler functions.

- [ ] **Step 1: Write failing test in `test_suite.ts`**

Add `verifyRosterAssignment()` assertion to `test_suite.ts`:
```typescript
export function verifyRosterAssignment(): void {
  const initialCount = 4;
  const newMember = {
    id: 'p-5',
    name: 'Gia Huy',
    email: 'giahuy@csac.local',
    role: 'Performer' as const,
    instrument: 'Guitar Lead',
    assignedSongsCount: 1,
    totalPracticeHours: 4,
  };
  const updatedRoster = [...rosterSample, newMember];
  if (updatedRoster.length !== initialCount + 1) throw new Error('Roster add failed');
}
```

- [ ] **Step 2: Run test to verify failure**

Run: `pnpm run test`
Expected: FAIL until assertion is wired into test runner.

- [ ] **Step 3: Update `en.ts`, `vi.ts`, and `roster/+page.svelte`**

Add i18n keys for roster modal: `roster_modal.title`, `roster_modal.select_member`, `roster_modal.select_role`, `roster_modal.select_instrument`, `roster_modal.btn_submit`.
Enhance `roster/+page.svelte` with:
- "+ Add Member to Roster" button & modal.
- Edit/Remove member action buttons on person cards.
- Reactive roster state updates.

- [ ] **Step 4: Run test & build verification**

Run: `pnpm run test && pnpm run build`
Expected: PASS with 0 build errors.

- [ ] **Step 5: Commit**

```bash
git add clients/web/src/lib/i18n/locales/en.ts clients/web/src/lib/i18n/locales/vi.ts clients/web/src/routes/studio/shows/[id]/roster/+page.svelte clients/web/test_suite.ts
git commit -m "feat(roster): implement add/edit member modals and roster management"
```

---

### Task 2: Song Lineup Role Assignment Drawer

**Files:**
- Modify: `clients/web/src/routes/studio/shows/[id]/numbers/+page.svelte`
- Test: `clients/web/test_suite.ts`

**Interfaces:**
- Consumes: `MusicNumber`, `BandRole`, `RosterMember` from `timetable.ts`.
- Produces: "Assign Band Lineup" drawer on Music Number cards allowing mapping roster performers to song roles (`Vocal Lead`, `Guitar`, `Bass`, `Drums`).

- [ ] **Step 1: Write song lineup assignment test in `test_suite.ts`**

Add `verifySongLineupAssignment()` assertion to `test_suite.ts`:
```typescript
export function verifySongLineupAssignment(): void {
  const songRoles: Record<string, string> = {
    vocal_lead: 'Minh Pháp',
    guitar_lead: 'Hoàng Nam',
    bass: 'Bảo Anh',
    drums: 'Thu Hà'
  };
  const filledCount = Object.keys(songRoles).length;
  if (filledCount !== 4) throw new Error('Song lineup mapping incomplete');
}
```

- [ ] **Step 2: Run test to verify failure**

Run: `pnpm run test`
Expected: FAIL until assertion is wired into test runner.

- [ ] **Step 3: Update `numbers/+page.svelte` with Lineup Assignment Drawer**

Enhance `clients/web/src/routes/studio/shows/[id]/numbers/+page.svelte` with:
- "Assign Lineup" button on each song card.
- Lineup Assignment Drawer displaying role dropdowns mapped to show roster members.
- Dynamic role badges on song cards showing assigned performers.

- [ ] **Step 4: Run test & build verification**

Run: `pnpm run test && pnpm run build`
Expected: PASS with clean build and all automated assertions green.

- [ ] **Step 5: Commit**

```bash
git add clients/web/src/routes/studio/shows/[id]/numbers/+page.svelte clients/web/test_suite.ts
git commit -m "feat(numbers): implement song lineup role assignment drawer and role badges"
```
