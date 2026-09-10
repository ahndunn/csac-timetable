# Reverse Proxy & API Gateway Specification (`gateway`)

**Subsystem**: `servers/services/gateway`  
**Document Status**: Active / Single Source of Truth (SSOT)  
**Technology Stack**: Rust (`axum`, `tower`, `tower-http`, `tonic`, `rdkafka`, `redis`)  

---

## 1. Overview & Responsibilities
The API Gateway is the single public entry point for all client platforms (`clients/web`, `clients/ios`, `clients/android`, `clients/mobile-cross`). It implements edge cross-cutting concerns:
- **Rate Limiting**: Distributed token-bucket algorithm backed by Redis 7.
- **Authentication & Authorization**: Bearer JWT verification and role checks.
- **Security Headers & CORS**: Strict CSP, HSTS, X-Content-Type-Options, and configurable CORS origins.
- **RESTful API Surface**: Exposes idiomatic HTTP endpoints to clients.
- **Protocol Translation**:
  - **Synchronous**: Dispatches RPC requests to downstream services (e.g. `scheduler-service`) via **Tonic (gRPC)**.
  - **Asynchronous**: Produces events onto **Apache Kafka** topics for long-running workflows.

---

## 2. HTTP REST Endpoints

### 2.1 Health & Metrics
- `GET /healthz`: Liveness probe (200 OK).
- `GET /readyz`: Readiness probe checking connections to Redis and downstream gRPC services.
- `GET /metrics`: Prometheus telemetry metrics.

### 2.2 Timetable & Scheduling (`/api/v1/schedule`)
- `POST /api/v1/schedule/solve`
  - **Request**: JSON payload conforming to `SolveTimetableRequest`.
  - **Routing**: Dispatches synchronous gRPC call `SchedulerClient::SolveTimetable` to `scheduler-service:50051`.
  - **Response**: JSON `SolveTimetableResponse` with scheduled sessions, unresolved items, and conflict diagnostics.

### 2.3 Vote & File Ingestion (`/api/v1/votes`)
- `POST /api/v1/votes/upload-async`
  - **Request**: Multipart Excel file upload.
  - **Routing**: Ingests raw payload, assigns correlation ID, publishes `VoteUploadRequested` event to Kafka topic `vote.upload.events`.
  - **Response**: `202 Accepted` with `{ "task_id": "uuid", "status": "QUEUED" }`.

---

## 3. Middleware Pipeline (Tower Layers)
1. **TraceLayer**: Distributed tracing via `tracing-opentelemetry`.
2. **CorsLayer**: Origin verification and header management.
3. **SecurityHeadersLayer**: Secure HTTP header injection.
4. **RateLimitLayer**: Token-bucket algorithm checking Redis counters.
5. **AuthLayer**: JWT validation and context propagation.
