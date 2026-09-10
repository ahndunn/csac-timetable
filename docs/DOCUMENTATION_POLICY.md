# Documentation-Driven Development (DDD) Policy & Governance

**Project**: CSAC Timetable Studio 🎵📅  
**Policy Status**: Mandatory Core Governance Rule  
**Enforcement Scope**: All Human Developers, AI Coding Assistants, and Automated Agents  

---

## 🎯 1. Core Principle

> **DOCUMENTATION IS THE ULTIMATE SINGLE SOURCE OF TRUTH (SSOT). CODE IS A DERIVED IMPLEMENTATION BYPRODUCT OF DOCUMENTATION.**

In this repository:
1. **No Code Without Specification**: No feature, refactoring, API change, data model modification, or algorithm tweak may be written in code without first being specified in the corresponding documentation under `docs/`.
2. **Docs Lead, Code Follows**: When adding functionality or fixing bugs, documentation in `docs/` must be updated **before or simultaneously with** code edits—never as an afterthought.
3. **Discrepancy Resolution**: In any conflict between code behavior and documentation specifications, **the documentation is presumed correct**, and the code is treated as buggy until proven otherwise or until documentation is explicitly amended first.

---

## 📐 2. Harness-Agnostic AI Agent Rules

These rules apply universally to any AI agent harness (Google Antigravity, Claude Code, Cursor, Windsurf, GitHub Copilot, custom scripts) and human contributors:

### Rule 1: Read Documentation Before Modifying Code
Before making any changes to `clients/web/src/` or `clients/web/test_suite.ts`:
* Consult [`docs/TECHNICAL.md`](file:///home/ahndunn/dev/csac-timetable/docs/TECHNICAL.md) and [`docs/BUSINESS.md`](file:///home/ahndunn/dev/csac-timetable/docs/BUSINESS.md).
* Verify existing specifications, data model contracts, and algorithm invariants.

### Rule 2: Update Documentation First (Doc-First Workflow)
Whenever introducing a change (new component, modified algorithm, changed data structure, new export option):
1. **Step 1 — Document**: Update the relevant markdown specification in `docs/` (`docs/BUSINESS.md`, `docs/TECHNICAL.md`, or a new spec file).
2. **Step 2 — Implement**: Write or modify the code in `clients/web/src/` to strictly match the updated documentation.
3. **Step 3 — Verify**: Update or add automated assertions in `clients/web/test_suite.ts` and verify that `pnpm run build` and `pnpm run test` pass cleanly.

### Rule 3: Single Source of Truth Synchronization
* If an architectural decision or business rule changes during implementation, the corresponding `docs/` file **must** be updated in the same commit/pull request.
* Code comments and docstrings must point directly to the authoritative `docs/` section rather than duplicating detailed specification text.

---

## 🔄 3. Documentation Structure & Map

| Document | Scope & Responsibilities | Update Trigger |
| :--- | :--- | :--- |
| [`docs/BUSINESS.md`](file:///home/ahndunn/dev/csac-timetable/docs/BUSINESS.md) | Business requirements, user personas, domain rules, export formats, UI sequence flows. | Changes in business logic, user features, file standards, or UI workflows. |
| [`docs/TECHNICAL.md`](file:///home/ahndunn/dev/csac-timetable/docs/TECHNICAL.md) | Architecture, tech stack, data models (`timetable.ts`), CSP algorithm rules, parser/exporter designs, test specifications. | Changes in TypeScript types, algorithm heuristics, project layout, dependencies, or test suite. |
| [`docs/DOCUMENTATION_POLICY.md`](file:///home/ahndunn/dev/csac-timetable/docs/DOCUMENTATION_POLICY.md) | Governance rules, DDD policy, AI agent instructions. | Changes in project governance or development guidelines. |

---

## 🧪 4. Automated Verification & CI Rules

1. **Pre-Commit Verification**:
   ```bash
   # 1. Verify build
   pnpm run build

   # 2. Run automated test suite
   pnpm run test
   ```
2. **Documentation Completeness Checklist**:
   - [ ] Did you update `docs/BUSINESS.md` or `docs/TECHNICAL.md` if data models, features, or algorithms changed?
   - [ ] Do all TypeScript types in `src/types/timetable.ts` match `docs/TECHNICAL.md` Section 2?
   - [ ] Do all automated tests in `test_system.ts` validate documented business rules?
