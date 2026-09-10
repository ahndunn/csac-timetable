# CSAC Timetable Studio — Documentation Hub

Welcome to the official documentation repository for **CSAC Timetable Studio 🎵📅**.

In accordance with our **Documentation-Driven Development (DDD)** policy, **documentation is the ultimate Single Source of Truth (SSOT)** for this project, and code is considered a implementation byproduct derived directly from these specifications.

---

## 📚 Documentation Index

1. [**Business & Product Specification** (`docs/BUSINESS.md`)](file:///home/ahndunn/dev/csac-timetable/docs/BUSINESS.md)
   * Product Vision & Problem Statement
   * Target User Personas & Value Proposition
   * Functional Requirements & Business Rules
   * User Journeys & Workflow Specifications

2. [**Technical Architecture & Engineering Specification** (`docs/TECHNICAL.md`)](file:///home/ahndunn/dev/csac-timetable/docs/TECHNICAL.md)
   * Technology Stack & Architecture Principles
   * System Topology & Component Diagram
   * Data Models & TypeScript Contracts
   * CSP Scheduling Algorithm Design (MRV + LCV Heuristics)
   * Excel Parsing & Multi-Tab Inspection Engine
   * Excel Exporter Engine (3-Sheet Report Generation)
   * Testing & Quality Assurance Suite

3. [**Documentation-Driven Development Policy** (`docs/DOCUMENTATION_POLICY.md`)](file:///home/ahndunn/dev/csac-timetable/docs/DOCUMENTATION_POLICY.md)
   * Core Governance Principle: *Documentation is SSOT, Code is Byproduct*
   * Change Management Workflow (Doc-First Rule)
   * Harness-Agnostic AI & Developer Rules
   * Verification & Linting Standards

---

## ⚡ Quick Navigation & Commands

```bash
# Start local development server
npm run dev

# Run full automated test suite (38 assertions)
npx tsx test_system.ts

# Production build check
npm run build
```
