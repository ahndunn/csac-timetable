<!-- AGENTS.md: Universal Harness-Agnostic Rules for CSAC Timetable Studio -->

# CSAC Timetable Studio — Agent & Developer Instructions

## 🚨 CRITICAL RULE: Documentation-Driven Development (DDD)

**DOCUMENTATION IS THE ULTIMATE SINGLE SOURCE OF TRUTH (SSOT). CODE IS A DERIVED IMPLEMENTATION BYPRODUCT.**

### 1. Mandatory Pre-Flight Checklist
Before inspecting, modifying, or generating any code in `src/` or `test_system.ts`:
1. Read the relevant documentation in `docs/`:
   - [`docs/BUSINESS.md`](file:///home/ahndunn/dev/csac-timetable/docs/BUSINESS.md) for business logic, constraints, user flows, and export formats.
   - [`docs/TECHNICAL.md`](file:///home/ahndunn/dev/csac-timetable/docs/TECHNICAL.md) for architecture, TypeScript contracts, CSP solver heuristics, and test suite specs.
   - [`docs/DOCUMENTATION_POLICY.md`](file:///home/ahndunn/dev/csac-timetable/docs/DOCUMENTATION_POLICY.md) for governance policy.

### 2. Documentation-First Change Workflow
When asked to implement any feature, bug fix, algorithm tweak, or schema edit:
1. **Update Documentation FIRST**: Edit the specification in `docs/` (`docs/BUSINESS.md`, `docs/TECHNICAL.md`, or a new spec file in `docs/`).
2. **Implement Code SECOND**: Write implementation code in `src/` strictly conforming to the updated documentation.
3. **Verify THIRD**: Run `npx tsx test_system.ts` and `npm run build` to ensure 100% test pass rate and clean build.

### 3. Verification Commands
Always run verification after making changes:
```bash
# Verify TypeScript & Vite build
npm run build

# Run automated test suite (38+ assertions)
npx tsx test_system.ts
```

### 4. Single Source of Truth Enforcement
If code behavior diverges from documentation, **the documentation is authoritative**. Fix the code to conform to the documentation, or update the documentation first if requirements have intentionally changed.
