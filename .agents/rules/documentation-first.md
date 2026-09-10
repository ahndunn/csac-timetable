---
trigger: always_on
description: Mandatory Documentation-Driven Development rule (Documentation is SSOT, Code is Byproduct)
---

# Documentation-Driven Development (DDD) Rule

## Core Principle
**DOCUMENTATION IS THE ULTIMATE SINGLE SOURCE OF TRUTH (SSOT). CODE IS A DERIVED IMPLEMENTATION BYPRODUCT.**

## Instructions for Agent
1. **Always Read Documentation First**: Before proposing fixes or modifying files in `src/`, read `docs/BUSINESS.md` and `docs/TECHNICAL.md`.
2. **Update Docs Before Code**: Any changes to data models (`src/types/timetable.ts`), CSP solver logic (`src/services/scheduler.ts`), Excel parsing (`src/services/excelParser.ts`), or UI features MUST be documented in `docs/` FIRST.
3. **Verify Compliance**: Run `npm run build` and `npx tsx test_system.ts` after any code edit. Ensure all documentation links remain valid and aligned.
