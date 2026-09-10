# Infrastructure & Deployment Guidelines

This directory (`deploy/`) manages container definitions and orchestration.

## Key Instructions
1. **Container Engine & Compose**:
   - The user uses **Podman** and `podman compose`.
   - Run `podman compose -f deploy/compose.yml up -d` to spin up local development infrastructure (PostgreSQL 16, Redis 7, Kafka in KRaft mode).
2. **Containerization**:
   - Use multi-stage Dockerfiles under `deploy/docker/`.
   - Never run service processes as root.

