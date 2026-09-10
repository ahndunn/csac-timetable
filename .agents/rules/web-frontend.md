---
trigger: model_decision
description: Svelte 5, SvelteKit SSR, and Bento Grid design system rules for web frontend development under clients/web/
---

# Web Frontend Engineering Rules (SvelteKit + Svelte 5 + Bento Grid UI)

## Scope
These rules apply exclusively to web client code under `clients/web/`. They MUST NOT be applied to backend Rust services or container infrastructure configurations.

---

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

### 3. Bento Grid Design System Rules
- **70/20/10 Color Rule**:
  - 70% Base Canvas: Deep clean neutral background (`#f8fafc`).
  - 20% Elevated Bento Cards: Crisp white surfaces (`#ffffff`) with subtle 1px border lines (`rgba(0, 0, 0, 0.08)`) and multi-layer soft shadows.
  - 10% True Orange Accent: Vivid energetic True Orange (`#ff6b00` / `#f97316`) for primary CTAs, active pills, badges, key metrics, and hover highlights.
- **Consistent Radius**: Uniform `16px`–`20px` border-radius across all Bento cells and modules.
- **Micro-Animations**: Stagger entrance animations and spring-easing hover lifts (`cubic-bezier(0.34, 1.56, 0.64, 1)`).
- **Skill Reference**: Consult `bento-design` (`.agents/skills/bento-design/SKILL.md`) for complete component patterns.
