---
trigger: model_decision
description: DevOps, Dockerfile multi-stage builds, and Docker Compose orchestration rules for deploy/
---

# DevOps & Infrastructure Engineering Rules

## Scope
These rules apply to files under `deploy/`, container definitions, and CI/CD pipelines.

## Core Rules & Patterns

### 1. Multi-Stage Container Builds
- Use minimal base images: `alpine` or `distroless` for production stages.
- For Rust services:
  - Cache dependency compilation via `cargo-chef` or staged recipe manifests.
  - Compile with `--release` flags and strip debug symbols.
  - Run as non-root user (`nobody` or dedicated user ID `10001`).
- For Node / SvelteKit:
  - Use `node:22-alpine`.
  - Install only production dependencies for the runner stage (`npm ci --omit=dev`).
  - Run via non-root `node` user.

### 2. Docker Compose Guidelines (`deploy/compose.yml`)
- Use Kafka in KRaft mode to avoid external Zookeeper containers.
- Always include explicit `healthcheck` declarations for Redis, Postgres, and Kafka.
- Use explicit networks (`csac-internal`, `csac-public`) for security isolation.
- Ensure stateful containers mount persistent named volumes (`pgdata`, `redisdata`, `kafkadata`).
