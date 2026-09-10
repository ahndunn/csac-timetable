## Project Configuration

- **Language**: TypeScript
- **Package Manager**: npm
- **Add-ons**: none

---

# Web Client Developer & Agent Guidelines

This directory (`clients/web/`) contains the SvelteKit-based client application.

## Key Instructions
1. **Framework**: SvelteKit with Svelte 5 runes (`$state`, `$derived`, `$props`).
2. **SSR & Progressive Enhancement**:
   - Utilize SvelteKit Form Actions (`+page.server.ts`) for file uploads and data mutations.
   - Use `use:enhance` on all forms to provide smooth client transitions without page reloading.
3. **Core Engine Integrity**:
   - The CSP algorithm (`lib/engine/scheduler.ts`), Excel parser (`lib/engine/excelParser.ts`), and Excel exporter (`lib/engine/excelExporter.ts`) must maintain 100% test compatibility with the specifications in `docs/TECHNICAL.md`.
4. **Verification**:
   - Run `npm run build` inside this directory before submitting changes.
