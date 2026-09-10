# ISO 639-1 Internationalization (i18n) Adaptation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement first-class ISO 639-1 internationalization (`vi`, `en`) in `clients/web` with top-right language switcher dropdown, machine locale auto-detection, and global URL search parameter synchronization in `+layout.svelte`.

**Architecture:** A zero-dependency, Svelte 5 rune-based reactive i18n store (`$state`, `$derived`) with dot-notation lookup and interpolation. Global locale state is synchronized with the URL search query parameter (`?lang=vi` / `?lang=en`) and client `navigator.language` in `+layout.svelte`. All UI components and modals consume reactive `t()` translation keys.

**Tech Stack:** SvelteKit 2, Svelte 5 (Runes), TypeScript, Lucide Icons, Vite.

**Spec:** [`docs/superpowers/specs/2026-09-10-i18n-adaptation-design.md`](file:///home/ahndunn/dev/csac-timetable/docs/superpowers/specs/2026-09-10-i18n-adaptation-design.md)

## Global Constraints
- Supported language identifiers MUST strictly comply with ISO 639-1 (`vi`, `en`).
- All URL search parameter synchronization MUST use `replaceState` without triggering page refreshes.
- If `?lang=` is missing in the URL, the client MUST fall back to `navigator.language` (defaulting to `vi` if starting with `vi`, otherwise `en`).
- Zero external i18n npm dependencies added to `package.json`.
- All automated tests in `test_suite.ts` and `pnpm run build` must pass cleanly.

---

### Task 1: Core i18n Subsystem & Dictionaries

**Files:**
- Create: `clients/web/src/lib/i18n/types.ts`
- Create: `clients/web/src/lib/i18n/locales/vi.ts`
- Create: `clients/web/src/lib/i18n/locales/en.ts`
- Create: `clients/web/src/lib/i18n/index.ts`
- Modify: `clients/web/test_suite.ts`

**Interfaces:**
- Produces:
  - `type Iso639_1Locale = 'vi' | 'en'`
  - `SUPPORTED_LANGUAGES: Record<Iso639_1Locale, LanguageOption>`
  - `i18nState: { currentLang: Iso639_1Locale }`
  - `t(key: string, params?: Record<string, string | number>): string`
  - `setLocale(locale: Iso639_1Locale): void`
  - `detectMachineLocale(): Iso639_1Locale`

- [ ] **Step 1: Write the failing tests in `test_suite.ts`**
Add TEST 9 testing dictionary parity, fallback, interpolation, and ISO 639-1 validation.

- [ ] **Step 2: Run test to verify it fails**
Run: `pnpm run test`
Expected: FAIL with module `./src/lib/i18n` not found.

- [ ] **Step 3: Implement `types.ts`, `vi.ts`, `en.ts`, and `index.ts`**
Define complete dictionaries for all application views with high-fidelity wording.

- [ ] **Step 4: Run test to verify it passes**
Run: `pnpm run test`
Expected: PASS (all assertions pass including TEST 9).

- [ ] **Step 5: Commit**
```bash
git add clients/web/src/lib/i18n clients/web/test_suite.ts
git commit -m "feat(i18n): implement core reactive translation store and dictionaries"
```

---

### Task 2: Global URL Parameter Sync & Locale Detection in `+layout.svelte`

**Files:**
- Modify: `clients/web/src/routes/+layout.svelte`

**Interfaces:**
- Consumes:
  - `i18nState`, `setLocale`, `detectMachineLocale`, `t` from `src/lib/i18n`
  - `$page` from `$app/state` or `$app/stores`
  - `replaceState` from `$app/navigation`

- [ ] **Step 1: Update `+layout.svelte` with reactive URL search parameter synchronization**
Read `page.url.searchParams.get('lang')`. On mount or route navigation, if valid ISO 639-1 code (`vi` or `en`), call `setLocale()`. If absent, detect machine locale and sync URL search param with `replaceState`.
Set `<svelte:head><title>{t('app.page_title')}</title></svelte:head>`.

- [ ] **Step 2: Verify Svelte check & build**
Run: `pnpm run check` and `pnpm run build`
Expected: Clean build without errors.

- [ ] **Step 3: Commit**
```bash
git add clients/web/src/routes/+layout.svelte
git commit -m "feat(i18n): sync URL search parameter and machine locale in +layout.svelte"
```

---

### Task 3: Navbar Language Switcher & Actions Translation

**Files:**
- Modify: `clients/web/src/lib/components/Navbar.svelte`

**Interfaces:**
- Consumes:
  - `t`, `i18nState`, `setLocale`, `SUPPORTED_LANGUAGES` from `src/lib/i18n`
- Produces:
  - Language dropdown button with flag (`🇻🇳` / `🇺🇸`), native name, and language switch callbacks.

- [ ] **Step 1: Refactor `Navbar.svelte`**
Replace all hardcoded strings with `t(...)`.
Add language dropdown container to both desktop action group and mobile action menu.

- [ ] **Step 2: Verify build**
Run: `pnpm run build`
Expected: PASS

- [ ] **Step 3: Commit**
```bash
git add clients/web/src/lib/components/Navbar.svelte
git commit -m "feat(i18n): add top-right language switcher dropdown to Navbar"
```

---

### Task 4: Calendar Grid & Day/Slot Localization

**Files:**
- Modify: `clients/web/src/lib/components/CalendarGrid.svelte`
- Modify: `clients/web/src/lib/constants/timetableDefaults.ts`

**Interfaces:**
- Consumes: `t`, `i18nState`

- [ ] **Step 1: Refactor `CalendarGrid.svelte` & constants**
Provide localized day header strings (`t('days.mon')` - `t('days.sun')`), slot room labels (`t('calendar.room', { room: num })`), empty slot buttons, and conflict badges.

- [ ] **Step 2: Verify build & tests**
Run: `pnpm run test` && `pnpm run build`
Expected: PASS

- [ ] **Step 3: Commit**
```bash
git add clients/web/src/lib/components/CalendarGrid.svelte clients/web/src/lib/constants/timetableDefaults.ts
git commit -m "feat(i18n): localize calendar grid headers, slots, and session cards"
```

---

### Task 5: Sidebar Repertoire & Scheduler Settings Localization

**Files:**
- Modify: `clients/web/src/lib/components/Sidebar.svelte`

**Interfaces:**
- Consumes: `t`, `i18nState`

- [ ] **Step 1: Refactor `Sidebar.svelte`**
Translate search placeholder, filter tabs ("All", "Scheduled", "Unresolved"), solver configuration options (rooms, attendance leeway, day spreading), target session counts, badges, and empty states.

- [ ] **Step 2: Verify build**
Run: `pnpm run build`
Expected: PASS

- [ ] **Step 3: Commit**
```bash
git add clients/web/src/lib/components/Sidebar.svelte
git commit -m "feat(i18n): localize sidebar repertoire and solver settings"
```

---

### Task 6: Modals Translation & Copy Polish

**Files:**
- Modify: `clients/web/src/lib/components/UploadModal.svelte`
- Modify: `clients/web/src/lib/components/SheetSelectionModal.svelte`
- Modify: `clients/web/src/lib/components/ConflictResolverModal.svelte`
- Modify: `clients/web/src/lib/components/EventDetailModal.svelte`
- Modify: `clients/web/src/lib/components/SlotAddModal.svelte`
- Modify: `clients/web/src/lib/components/RawVoteModal.svelte`
- Modify: `clients/web/src/routes/+page.svelte`

**Interfaces:**
- Consumes: `t`, `i18nState`

- [ ] **Step 1: Replace all modal copy with `t(...)`**
Refactor all 6 modal dialogs and `+page.svelte` notifications/confirm dialogs with polished Vietnamese and context-preserving English translations.

- [ ] **Step 2: Verify build**
Run: `pnpm run build`
Expected: PASS

- [ ] **Step 3: Commit**
```bash
git add clients/web/src/lib/components/*.svelte clients/web/src/routes/+page.svelte
git commit -m "feat(i18n): localize all modal dialogues and notifications"
```

---

### Task 7: Full Verification & E2E Validation

**Files:**
- Verify: `clients/web/test_suite.ts`

- [ ] **Step 1: Run comprehensive test suite**
Run: `pnpm run test`
Expected: All tests pass cleanly (100% assertions green).

- [ ] **Step 2: Run production build**
Run: `pnpm run build`
Expected: Clean Vite / SvelteKit production build.

- [ ] **Step 3: Commit and summarize**
```bash
git commit -m "chore(i18n): complete first-class English and Vietnamese localization"
```
