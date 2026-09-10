# Backend Services Developer & Agent Guidelines

This directory (`servers/`) contains the Rust microservices workspace.

## Key Instructions
1. **Workspace Layout**:
   - `services/gateway`: Edge Reverse Proxy & API Gateway (Axum + Tower).
   - `services/scheduler-service`: CSP Timetable Solver (Tonic gRPC).
   - `services/vote-ingestion-service`: Vote & Sheet Processing (Tonic gRPC + Kafka).
   - `crates/proto`: Protocol Buffers definitions and generated code.
   - `crates/common`: Shared models, errors, and telemetry.
2. **Communication Discipline**:
   - Synchronous inter-service requests must use Tonic gRPC.
   - Asynchronous inter-service workflows must publish and consume messages via Kafka.
3. **Verification**:
   - Run `cargo check --workspace` to verify types and compilation.
