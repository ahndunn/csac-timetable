---
trigger: model_decision
description: Svelte 5 and SvelteKit SSR best practices for web client development in clients/web/
---

# Web Frontend Engineering Rules (SvelteKit + Svelte 5)

## Scope
These rules apply exclusively to code under `clients/web/`. They must NOT be applied to backend Rust services or infrastructure configurations.

## Core Rules & Patterns

### 1. Svelte 5 Runes Architecture
- **State**: Use `$state(initialValue)` for local component reactivity.
- **Derived**: Use `$derived(expression)` and `$derived.by(() => { ... })` for computed values. Never use legacy `$:` reactive declarations.
- **Props**: Declare incoming props using `$props()`, e.g., `let { title, sessions = [] }: Props = $props();`.
- **Effects**: Use `$effect(() => { ... })` strictly for browser side-effects (DOM manipulation, canvas, event listeners), never for state synchronization.

### 2. SvelteKit SSR & Progressive Enhancement
- **Forms**: Always write native `<form method="POST">`.
- **Form Actions**: Define mutation handlers in `+page.server.ts` using `actions = { default: async ({ request }) => { ... } }`.
- **Enhancement**: Attach `use:enhance` from `$app/forms` on forms to provide seamless client-side single-page transitions with progressive enhancement fallback.
- **Store Isolation**: Never instantiate writable/readable Svelte stores in module scope on the server to prevent cross-request state leakage during SSR.

### 3. Styling & Aesthetics
- Use CSS Variables, modern flexbox, and CSS grid.
- Follow the Google Calendar-inspired pastel color scheme defined in `PASTEL_PALETTE`.
- Ensure responsive layouts and accessible semantic HTML elements.
