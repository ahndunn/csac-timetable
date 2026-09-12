# Web Migration to shadcn-svelte + Tailwind CSS v4 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Completely migrate `clients/web/` from Vanilla CSS to `shadcn-svelte` (Svelte 5 runes) and `Tailwind CSS v4`, preserving the True Orange 70/20/10 Bento Grid palette, removing all legacy `<style>` tags, and extracting auto-applied rules.

**Architecture:** Tailwind CSS v4 integrated via `@tailwindcss/vite` in `vite.config.ts`, theme variables in `src/app.css`, shadcn-svelte UI primitives generated under `$lib/components/ui/`, utility function in `$lib/utils.ts`.

**Tech Stack:** SvelteKit 2, Svelte 5, Tailwind CSS v4, `@tailwindcss/vite`, `bits-ui`, `tailwind-merge`, `clsx`, `tailwind-variants`, `@lucide/svelte`.

**Spec:** [`docs/superpowers/specs/2026-09-12-shadcn-svelte-tailwind-migration-design.md`](file:///home/ahndunn/dev/csac-timetable/docs/superpowers/specs/2026-09-12-shadcn-svelte-tailwind-migration-design.md)

## Global Constraints
- **Zero Vanilla CSS**: Every single `.svelte` component must have its `<style>` block removed and replaced with Tailwind CSS utility classes and shadcn-svelte primitives.
- **Palette Integrity**: Preserve 70% slate canvas (`#f8fafc`), 20% white bento cards (`#ffffff`), 10% True Orange (`#ff6b00` / `#f97316`).
- **DDD Enforcement**: Rules and documentation in `docs/` and `.agents/rules/` must be updated first.
- **100% Test Passing**: All 104 assertions in `test_suite.ts` must pass after migration.
- **i18n Preservation**: Zero lost translation keys.

---

### Task 1: Documentation-Driven Development & Rules Extraction
**Files:**
- Create: `.agents/rules/shadcn-svelte.md`
- Modify: `.agents/rules/web-frontend.md`
- Modify: `docs/TECHNICAL.md`

- [ ] **Step 1: Create shadcn-svelte rule file**
  Extract full shadcn-svelte + Tailwind CSS conventions to `.agents/rules/shadcn-svelte.md`.
- [ ] **Step 2: Update web-frontend rules and TECHNICAL.md**
  Update `.agents/rules/web-frontend.md` and `docs/TECHNICAL.md` to establish Tailwind v4 + shadcn-svelte as standard.
- [ ] **Step 3: Verify rules syntax & docs alignment**

---

### Task 2: Tailwind CSS v4 & shadcn-svelte Infrastructure
**Files:**
- Modify: `clients/web/vite.config.ts`
- Create: `clients/web/components.json`
- Create: `clients/web/src/lib/utils.ts`
- Modify: `clients/web/src/app.css`
- Modify: `clients/web/src/routes/+layout.svelte`

- [ ] **Step 1: Configure `@tailwindcss/vite` in `vite.config.ts`**
- [ ] **Step 2: Create `components.json` & `$lib/utils.ts`**
- [ ] **Step 3: Modernize `src/app.css` with `@import "tailwindcss";` and `@theme` brand variables**
- [ ] **Step 4: Verify initial build with `pnpm run build`**

---

### Task 3: Install & Setup Core shadcn-svelte UI Primitives
**Files:**
- Create: `clients/web/src/lib/components/ui/button/*`
- Create: `clients/web/src/lib/components/ui/badge/*`
- Create: `clients/web/src/lib/components/ui/card/*`
- Create: `clients/web/src/lib/components/ui/dialog/*`
- Create: `clients/web/src/lib/components/ui/dropdown-menu/*`
- Create: `clients/web/src/lib/components/ui/input/*`
- Create: `clients/web/src/lib/components/ui/label/*`
- Create: `clients/web/src/lib/components/ui/select/*`
- Create: `clients/web/src/lib/components/ui/table/*`
- Create: `clients/web/src/lib/components/ui/tabs/*`
- Create: `clients/web/src/lib/components/ui/separator/*`
- Create: `clients/web/src/lib/components/ui/scroll-area/*`
- Create: `clients/web/src/lib/components/ui/tooltip/*`

- [ ] **Step 1: Scaffold standard Svelte 5 / bits-ui primitives in `$lib/components/ui/`**
- [ ] **Step 2: Verify component TypeScript definitions and exports**

---

### Task 4: Migrate Shared Components & Modals
**Files:**
- Modify: `clients/web/src/lib/components/Navbar.svelte`
- Modify: `clients/web/src/lib/components/Sidebar.svelte`
- Modify: `clients/web/src/lib/components/TaskStatusSignal.svelte`
- Modify: `clients/web/src/lib/components/CalendarGrid.svelte`
- Modify: `clients/web/src/lib/components/ConflictResolverModal.svelte`
- Modify: `clients/web/src/lib/components/EventDetailModal.svelte`
- Modify: `clients/web/src/lib/components/SlotAddModal.svelte`
- Modify: `clients/web/src/lib/components/UploadModal.svelte`
- Modify: `clients/web/src/lib/components/SheetSelectionModal.svelte`
- Modify: `clients/web/src/lib/components/RawVoteModal.svelte`

- [ ] **Step 1: Migrate `Navbar.svelte`, `Sidebar.svelte`, and `TaskStatusSignal.svelte` to Tailwind + DropdownMenu**
- [ ] **Step 2: Migrate all 6 modal dialogs to `Dialog` primitive with Tailwind classes**
- [ ] **Step 3: Migrate `CalendarGrid.svelte` to Tailwind CSS grid and eliminate `<style>` block**
- [ ] **Step 4: Verify test suite pass**

---

### Task 5: Migrate Route Pages (Landing, Studio Hub, Gear, Admin, Auth)
**Files:**
- Modify: `clients/web/src/routes/+page.svelte`
- Modify: `clients/web/src/routes/studio/+page.svelte`
- Modify: `clients/web/src/routes/studio/gear/+page.svelte`
- Modify: `clients/web/src/routes/auth/login/+page.svelte`
- Modify: `clients/web/src/routes/admin/shows/+page.svelte`
- Modify: `clients/web/src/routes/admin/users/+page.svelte`
- Modify: `clients/web/src/routes/admin/approve/+page.svelte`

- [ ] **Step 1: Migrate Landing page `+page.svelte` to Bento Grid with `Card` & Tailwind**
- [ ] **Step 2: Migrate Studio Hub `studio/+page.svelte` and Gear `studio/gear/+page.svelte`**
- [ ] **Step 3: Migrate Auth Login `auth/login/+page.svelte`**
- [ ] **Step 4: Migrate Admin pages (`admin/shows`, `admin/users`, `admin/approve`) using `Table` and `Badge`**
- [ ] **Step 5: Run tests and build check**

---

### Task 6: Migrate Show Studio Sub-Screens
**Files:**
- Modify: `clients/web/src/routes/studio/shows/[id]/+layout.svelte`
- Modify: `clients/web/src/routes/studio/shows/[id]/overview/+page.svelte`
- Modify: `clients/web/src/routes/studio/shows/[id]/numbers/+page.svelte`
- Modify: `clients/web/src/routes/studio/shows/[id]/sprints/+page.svelte`
- Modify: `clients/web/src/routes/studio/shows/[id]/roster/+page.svelte`

- [ ] **Step 1: Migrate Show Studio Layout `+layout.svelte` using `Tabs`**
- [ ] **Step 2: Migrate Show Overview `overview/+page.svelte` with metric cards and deep links**
- [ ] **Step 3: Migrate Scalable Numbers `numbers/+page.svelte` (Bento Grid, Kanban, Table)**
- [ ] **Step 4: Migrate Sprint Calendar & Matrix `sprints/+page.svelte`**
- [ ] **Step 5: Migrate Show Roster `roster/+page.svelte`**

---

### Task 7: Final Cleanup, CSS Purge & Verification
**Files:**
- Modify: `clients/web/src/app.css` (remove legacy classes, verify zero unused CSS)
- Review: `clients/web/src/**/*.svelte` (ensure 0 `<style>` tags remain)

- [ ] **Step 1: Purge any remaining vanilla CSS from `src/app.css`**
- [ ] **Step 2: Run automated check: `grep -r "<style>" src/` must return 0 occurrences**
- [ ] **Step 3: Run `pnpm run test` (104 tests passing)**
- [ ] **Step 4: Run `pnpm run build` and `pnpm run check`**
