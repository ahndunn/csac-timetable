# Design Specification: Web Frontend Migration to shadcn-svelte + Tailwind CSS v4

**Status**: Draft for User Approval  
**Author**: Migration Team (SA, Lead Dev, QC, UI Designer)  
**Date**: 2026-09-12  

---

## 1. Executive Summary & Goals
This project migrates `clients/web/` from legacy Vanilla CSS (`src/app.css` + inline `<style>` tags) to **shadcn-svelte (Svelte 5 Runes edition)** and **Tailwind CSS v4** with `@tailwindcss/vite`.
Every single component and page route will use utility classes and shadcn-svelte UI primitives while strictly maintaining:
1. The **70/20/10 Bento Grid palette** (70% neutral canvas `#f8fafc`, 20% elevated white bento cards `#ffffff`, 10% vivid True Orange `#ff6b00` / `#f97316` accent).
2. The **Documentation-Driven Development (DDD)** policy (updating `docs/TECHNICAL.md` and `.agents/rules/` first).
3. **100% test suite pass rate** (`pnpm run test`, 104 assertions) and clean SSR build (`pnpm run build`).
4. **Complete i18n key preservation** across Vietnamese (`vi`) and English (`en`).

---

## 2. Architecture & Styling System

### 2.1 Tailwind CSS v4 & Theme Variable Tokens
Tailwind CSS v4 will be integrated directly into `vite.config.ts` using `@tailwindcss/vite`.
`src/app.css` will configure `@import "tailwindcss";` and `@theme` mappings:
- `--color-background`, `--color-foreground`, `--color-card`, `--color-card-foreground`
- `--color-primary: #ff6b00`, `--color-primary-hover: #ea580c`, `--color-primary-foreground: #ffffff`
- `--color-muted`, `--color-muted-foreground`, `--color-border`, `--color-ring`
- `--radius-bento: 18px`, `--radius-lg: 14px`, `--radius-md: 10px`, `--radius-sm: 6px`

### 2.2 shadcn-svelte Configuration
- `components.json` configured for Svelte 5 runes and Tailwind v4.
- Utilities located at `$lib/utils.ts` (`cn(...)` via `clsx` and `tailwind-merge`).
- UI Primitives created in `$lib/components/ui/*`:
  - `Button`, `Badge`, `Card`, `Dialog`, `DropdownMenu`, `Input`, `Label`, `Select`, `Table`, `Tabs`, `Separator`, `ScrollArea`, `Sheet`, `Tooltip`, `AlertDialog`.

---

## 3. Automated Rule Extraction (`.agents/rules/shadcn-svelte.md`)
Create `.agents/rules/shadcn-svelte.md` and update `.agents/rules/web-frontend.md` to establish automatic styling rules for all `**/*.svelte` files:
- **No inline `<style>` tags**: All component styling must be expressed through Tailwind utility classes or shadcn-svelte component props.
- **shadcn-svelte Component First**: Use `$lib/components/ui/button`, `$lib/components/ui/card`, `$lib/components/ui/dialog`, etc. instead of raw unstyled elements.
- **Theme Consistency**: Use semantic color utilities (`text-primary`, `bg-card`, `border-border`, `hover:bg-accent`) preserving the True Orange `#ff6b00` brand accent.
- **Svelte 5 Runes Compatibility**: All component wrappers and consumers use `$props()`, `$state()`, and snippet children `{@render children?.()}`.

---

## 4. Component & Route Redesign Plan

### 4.1 Modals & Dialogs
- **Current**: Custom floating overlay `div.modal-backdrop` and `div.modal-card`.
- **shadcn-svelte Migration**: Replaced with `Dialog` / `DialogContent` / `DialogHeader` / `DialogTitle` / `DialogFooter` with accessible escape handlers, focus trapping, and smooth spring transitions.
- Files: `ConflictResolverModal.svelte`, `EventDetailModal.svelte`, `SlotAddModal.svelte`, `UploadModal.svelte`, `SheetSelectionModal.svelte`, `RawVoteModal.svelte`.

### 4.2 Bento Cards & Hub
- **Current**: `.bento-card` with custom box shadows.
- **shadcn-svelte Migration**: `Card` with Tailwind classes `rounded-[18px] border border-black/[0.08] dark:border-white/[0.08] shadow-sm hover:shadow-md transition-all duration-200`.
- Routes: `/` (Hub), `/studio`, `/studio/gear`, `/studio/shows/[id]/overview`, `/numbers`, `/sprints`, `/roster`.

### 4.3 Tables & Data Lists
- **Current**: Raw HTML tables with custom border/padding CSS.
- **shadcn-svelte Migration**: `Table`, `TableHeader`, `TableHead`, `TableBody`, `TableRow`, `TableCell`.
- Routes: `/studio/shows/[id]/numbers` (table view), `/studio/shows/[id]/roster`, `/studio/gear`, `/admin/users`, `/admin/shows`, `/admin/approve`.

### 4.4 Navigation & App Shell
- **Current**: Custom CSS flexbox bars with fixed heights.
- **shadcn-svelte Migration**: Clean Tailwind utility layout, `DropdownMenu` for Language and User Profile switchers, responsive sheet/drawer for mobile navigation.
- Files: `Navbar.svelte`, `Sidebar.svelte`, `+layout.svelte`.

---

## 5. Migration Execution Strategy
1. **Documentation & Rules First (DDD)**:
   - Create `.agents/rules/shadcn-svelte.md`.
   - Update `.agents/rules/web-frontend.md`.
   - Update `docs/TECHNICAL.md`.
2. **Infrastructure & Tooling**:
   - Configure `@tailwindcss/vite` in `vite.config.ts`.
   - Setup `components.json` and install core shadcn-svelte UI primitives into `$lib/components/ui/`.
   - Refactor `src/app.css` to use Tailwind v4 `@theme` and brand color tokens.
3. **Migrate Modals & Shared Components**:
   - Transform all 10 shared components to shadcn-svelte + Tailwind.
4. **Migrate All Routes (12 pages/layouts)**:
   - Landing (`/`), Studio Hub (`/studio`), Gear (`/studio/gear`).
   - Show Studio sub-screens (`overview`, `numbers`, `sprints`, `roster`).
   - Admin routes (`shows`, `users`, `approve`) & Auth (`login`).
5. **Purge Legacy Styling Artifacts**:
   - Strip all `<style>` blocks and legacy classes across all `.svelte` files.
6. **QC Verification & Test Suite**:
   - Run `pnpm run test` (104/104 passing).
   - Run `pnpm run build` (Clean SSR production bundle).
