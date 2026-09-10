---
trigger: model_decision
description: Rust microservices patterns, Axum, Tonic gRPC, and Kafka conventions for servers/
---

# Backend Microservices Engineering Rules (Rust)

## Scope
These rules apply exclusively to code under `servers/`. They must NOT be applied to frontend client code.

## Core Rules & Patterns

### 1. Unified Architecture & Cargo Workspace
- Group all services in the `servers/` Cargo workspace.
- Shared domain types, error handling, and telemetry live in `servers/crates/common`.
- Protobuf definitions and generated Tonic traits live in `servers/crates/proto`.
- Kafka event definitions live in `servers/crates/kafka-events`.

### 2. Edge Gateway & Axum Patterns
- Expose RESTful endpoints conforming strictly to OpenAPI / REST standards.
- Build layered middleware with **Tower** (`TraceLayer`, `CorsLayer`, `RateLimitLayer`).
- Translate incoming client requests to synchronous **Tonic** gRPC client calls or publish asynchronous events to **rdkafka**.

### 3. gRPC & Inter-Service Communication
- Keep services decoupled and stateless.
- Define service contracts in `proto/*.proto` using proto3 syntax.
- Use `tonic-build` in build scripts (`build.rs`).
- Implement proper gRPC status codes (`Status::invalid_argument`, `Status::not_found`, `Status::internal`).

### 4. Async Safety & Reliability
- Use `tokio` multi-threaded runtime.
- Apply backpressure when consuming messages with `rdkafka`.
- Structure logging with `tracing` and structured spans (`#[tracing::instrument]`).
- Do not use `unwrap()` or `expect()` in production service request handlers; handle errors with `Result<T, AppError>` and `thiserror`.
