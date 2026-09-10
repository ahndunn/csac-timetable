# Scheduler Microservice Specification (`scheduler-service`)

**Subsystem**: `servers/services/scheduler-service`  
**Document Status**: Active / Single Source of Truth (SSOT)  
**Technology Stack**: Rust (`tonic`, `prost`, `tokio`, `redis`)  

---

## 1. Overview & Responsibilities
The Scheduler Service is a dedicated, stateless computational microservice implementing the Constraint Satisfaction Problem (CSP) timetable solver in Rust.
- Operates primarily via **gRPC (Tonic)** on port `50051`.
- Receives song votes, availability matrices, room capacities, and constraint parameters.
- Solves weekly practice schedules using heuristic CSP search (MRV + LCV + Backtracking).
- Caches immutable input matrices and solved schedule results in Redis 7 with keyed TTLs.

---

## 2. gRPC Protocol Definition (`proto/scheduler.proto`)

```protobuf
syntax = "proto3";

package csac.timetable.v1;

message SolveTimetableRequest {
  repeated SongVoteData songs = 1;
  SolverSettings settings = 2;
  repeated string days = 3;
  repeated string time_slots = 4;
  repeated ScheduledSession manual_sessions = 5;
}

message SolveTimetableResponse {
  repeated ScheduledSession schedule = 1;
  repeated UnresolvedSong unresolved = 2;
  repeated ConflictItem conflicts = 3;
  SolverStats stats = 4;
}

service SchedulerService {
  rpc SolveTimetable(SolveTimetableRequest) returns (SolveTimetableResponse);
  rpc HealthCheck(HealthCheckRequest) returns (HealthCheckResponse);
}
```

---

## 3. High-Performance Heuristics
- **MRV (Most Constrained Variable First)**: Priority queue ordered by candidate slot count ascending, band size descending.
- **LCV (Least Constraining Value)**: Slot evaluation minimizing shared member contention across other pending songs.
- **Deterministic Outcome**: Produces identical schedule assignments for given inputs and random seed.
