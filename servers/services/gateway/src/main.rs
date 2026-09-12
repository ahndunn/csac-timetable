use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    middleware::{self, Next},
    response::{sse::{Event as SseEvent, Sse}, Response},
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::{DateTime, Utc};
use csac_common::{
    calculate_required_approvals, generate_jwt, generate_otp_code, generate_random_password,
    hash_password, verify_jwt, verify_password, AdminDowngradeProposal, Claims,
    Event, EventStatus, OpenObserveClient, ProposalStatus, User, UserRole,
    VoteDecision,
};
use csac_kafka_events::{
    AdminOtpGeneratedEvent, EventStatusChangedEvent, UserCreatedEvent,
    TOPIC_ADMIN_OTP_GENERATED, TOPIC_EVENT_STATUS_CHANGED, TOPIC_USER_CREATED,
};
use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    ClientConfig,
};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{net::SocketAddr, sync::Arc, time::Instant};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub redis: redis::aio::ConnectionManager,
    pub kafka_producer: Option<FutureProducer>,
    pub jwt_secret: String,
    pub oo_client: OpenObserveClient,
}

// ==========================================
// Authentication Middleware & Extractors
// ==========================================

async fn extract_claims(headers: &HeaderMap, secret: &str) -> Option<Claims> {
    if let Some(auth_header) = headers.get("Authorization").and_then(|h| h.to_str().ok()) {
        if auth_header.starts_with("Bearer ") {
            let token = &auth_header[7..];
            return verify_jwt(token, secret).ok();
        }
    }
    None
}

async fn telemetry_middleware(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    request: axum::extract::Request,
    next: Next,
) -> Response {
    let start = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();
    let path = uri.path().to_string();

    let claims = extract_claims(&headers, &state.jwt_secret).await;
    let user_id = claims.as_ref().map(|c| c.sub.to_string());
    let role = claims.as_ref().map(|c| c.role.as_str());

    let response = next.run(request).await;
    let latency_ms = start.elapsed().as_millis() as u64;
    let status = response.status().as_u16();

    let meta = json!({
        "method": method.as_str(),
        "path": path,
        "status": status,
        "latency_ms": latency_ms,
        "user_id": user_id,
        "role": role,
    });

    state
        .oo_client
        .emit_log(
            if status >= 400 { "WARN" } else { "INFO" },
            &format!("HTTP {} {}", method, path),
            meta,
        )
        .await;

    response
}

// ==========================================
// API Handlers
// ==========================================

async fn healthz() -> &'static str {
    "OK"
}

// ------------------------------------------
// Auth Handlers
// ------------------------------------------

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
    user: User,
}

async fn login_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<Value>)> {
    let user = sqlx::query_as::<_, User>(
        "SELECT id, email, full_name, password_hash, role, status, created_at, updated_at FROM users WHERE email = $1 AND status = 'active'"
    )
    .bind(&payload.email)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid email or password"}))))?;

    let is_valid = verify_password(&payload.password, &user.password_hash)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    if !is_valid {
        return Err((StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid email or password"}))));
    }

    let token = generate_jwt(&user, &state.jwt_secret, 86400 * 7)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(LoginResponse { token, user }))
}

async fn me_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<User>, (StatusCode, Json<Value>)> {
    let claims = extract_claims(&headers, &state.jwt_secret)
        .await
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(json!({"error": "Unauthorized"}))))?;

    let user = sqlx::query_as::<_, User>(
        "SELECT id, email, full_name, password_hash, role, status, created_at, updated_at FROM users WHERE id = $1"
    )
    .bind(claims.sub)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "User not found"}))))?;

    Ok(Json(user))
}

// ------------------------------------------
// Admin User Management
// ------------------------------------------

#[derive(Deserialize)]
struct CreateUserRequest {
    email: String,
    full_name: String,
    role: UserRole,
}

async fn list_users_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<Vec<User>>, (StatusCode, Json<Value>)> {
    let claims = extract_claims(&headers, &state.jwt_secret)
        .await
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(json!({"error": "Unauthorized"}))))?;

    if claims.role != UserRole::Admin {
        return Err((StatusCode::FORBIDDEN, Json(json!({"error": "Admin role required"}))));
    }

    let users = sqlx::query_as::<_, User>(
        "SELECT id, email, full_name, password_hash, role, status, created_at, updated_at FROM users ORDER BY created_at DESC"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(users))
}

async fn create_user_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    let claims = extract_claims(&headers, &state.jwt_secret)
        .await
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(json!({"error": "Unauthorized"}))))?;

    if claims.role != UserRole::Admin {
        return Err((StatusCode::FORBIDDEN, Json(json!({"error": "Admin role required"}))));
    }

    let initial_password = generate_random_password(10);
    let password_hash = hash_password(&initial_password)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    let new_user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (email, full_name, password_hash, role, status)
        VALUES ($1, $2, $3, $4, 'active')
        RETURNING id, email, full_name, password_hash, role, status, created_at, updated_at
        "#,
    )
    .bind(&payload.email)
    .bind(&payload.full_name)
    .bind(&password_hash)
    .bind(payload.role)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({"error": format!("Could not create user: {}", e)}))))?;

    // Publish Kafka Event for Async Email Dispatch
    if let Some(producer) = &state.kafka_producer {
        let event = UserCreatedEvent {
            user_id: new_user.id,
            email: new_user.email.clone(),
            full_name: new_user.full_name.clone(),
            initial_password: initial_password.clone(),
            role: new_user.role.as_str().to_string(),
            created_at: Utc::now(),
        };
        let payload_str = serde_json::to_string(&event).unwrap_or_default();
        let user_key = new_user.id.to_string();
        let record = FutureRecord::to(TOPIC_USER_CREATED)
            .key(&user_key)
            .payload(&payload_str);
        let _ = producer.send(record, std::time::Duration::from_secs(3)).await;
    }

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "user": new_user,
            "initial_password": initial_password
        })),
    ))
}

#[derive(Deserialize)]
struct UpdateRoleRequest {
    new_role: UserRole,
}

async fn update_user_role_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(target_id): Path<Uuid>,
    Json(payload): Json<UpdateRoleRequest>,
) -> Result<Json<User>, (StatusCode, Json<Value>)> {
    let claims = extract_claims(&headers, &state.jwt_secret)
        .await
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(json!({"error": "Unauthorized"}))))?;

    if claims.role != UserRole::Admin {
        return Err((StatusCode::FORBIDDEN, Json(json!({"error": "Admin role required"}))));
    }

    let target = sqlx::query_as::<_, User>(
        "SELECT id, email, full_name, password_hash, role, status, created_at, updated_at FROM users WHERE id = $1"
    )
    .bind(target_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Target user not found"}))))?;

    if target.role == UserRole::Admin && payload.new_role != UserRole::Admin {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Cannot unilaterally downgrade an Admin. Please create an Admin Downgrade Proposal at /api/v1/admin/users/:id/downgrade-proposal"
            })),
        ));
    }

    let updated = sqlx::query_as::<_, User>(
        "UPDATE users SET role = $1, updated_at = NOW() WHERE id = $2 RETURNING id, email, full_name, password_hash, role, status, created_at, updated_at"
    )
    .bind(payload.new_role)
    .bind(target_id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(updated))
}

// ------------------------------------------
// Admin Downgrade Quorum Proposal & OTP
// ------------------------------------------

#[derive(Deserialize)]
struct CreateProposalRequest {
    target_role: UserRole,
    reason: String,
}

async fn create_downgrade_proposal_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(target_id): Path<Uuid>,
    Json(payload): Json<CreateProposalRequest>,
) -> Result<(StatusCode, Json<AdminDowngradeProposal>), (StatusCode, Json<Value>)> {
    let claims = extract_claims(&headers, &state.jwt_secret)
        .await
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(json!({"error": "Unauthorized"}))))?;

    if claims.role != UserRole::Admin {
        return Err((StatusCode::FORBIDDEN, Json(json!({"error": "Admin role required"}))));
    }

    let target = sqlx::query_as::<_, User>(
        "SELECT id, email, full_name, password_hash, role, status, created_at, updated_at FROM users WHERE id = $1"
    )
    .bind(target_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Target user not found"}))))?;

    if target.role != UserRole::Admin {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Target user is not an Admin"})),
        ));
    }

    let total_admins: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM users WHERE role = 'admin' AND status = 'active'"
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    let total_n = total_admins.0 as usize;
    let required_approvals = calculate_required_approvals(total_n)
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({"error": e.to_string()}))))?;

    let proposal = sqlx::query_as::<_, AdminDowngradeProposal>(
        r#"
        INSERT INTO admin_downgrade_proposals (
            target_admin_id, target_role, initiated_by, reason,
            total_admins_at_proposal, required_approvals, current_approvals, status
        )
        VALUES ($1, $2, $3, $4, $5, $6, 0, 'pending')
        RETURNING id, target_admin_id, target_role, initiated_by, reason,
                  total_admins_at_proposal, required_approvals, current_approvals,
                  status, created_at, expires_at
        "#
    )
    .bind(target_id)
    .bind(payload.target_role)
    .bind(claims.sub)
    .bind(&payload.reason)
    .bind(total_n as i32)
    .bind(required_approvals as i32)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok((StatusCode::CREATED, Json(proposal)))
}

#[derive(Serialize, sqlx::FromRow)]
struct ProposalDetailRow {
    id: Uuid,
    target_admin_id: Uuid,
    target_name: String,
    target_email: String,
    target_role: UserRole,
    initiated_by: Uuid,
    initiator_name: String,
    reason: String,
    total_admins_at_proposal: i32,
    required_approvals: i32,
    current_approvals: i32,
    status: ProposalStatus,
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
}

async fn list_proposals_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<Vec<ProposalDetailRow>>, (StatusCode, Json<Value>)> {
    let claims = extract_claims(&headers, &state.jwt_secret)
        .await
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(json!({"error": "Unauthorized"}))))?;

    if claims.role != UserRole::Admin {
        return Err((StatusCode::FORBIDDEN, Json(json!({"error": "Admin role required"}))));
    }

    let rows = sqlx::query_as::<_, ProposalDetailRow>(
        r#"
        SELECT p.id, p.target_admin_id, u.full_name as target_name, u.email as target_email,
               p.target_role, p.initiated_by, i.full_name as initiator_name,
               p.reason, p.total_admins_at_proposal, p.required_approvals, p.current_approvals,
               p.status, p.created_at, p.expires_at
        FROM admin_downgrade_proposals p
        JOIN users u ON p.target_admin_id = u.id
        JOIN users i ON p.initiated_by = i.id
        ORDER BY p.created_at DESC
        "#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(rows))
}

async fn request_proposal_otp_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(proposal_id): Path<Uuid>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let claims = extract_claims(&headers, &state.jwt_secret)
        .await
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(json!({"error": "Unauthorized"}))))?;

    if claims.role != UserRole::Admin {
        return Err((StatusCode::FORBIDDEN, Json(json!({"error": "Admin role required"}))));
    }

    let proposal = sqlx::query_as::<_, AdminDowngradeProposal>(
        "SELECT * FROM admin_downgrade_proposals WHERE id = $1 AND status = 'pending'"
    )
    .bind(proposal_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Active pending proposal not found"}))))?;

    let target_user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(proposal.target_admin_id)
        .fetch_one(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    let otp_code = generate_otp_code();
    let ttl_seconds = 600u64;

    let redis_key = format!("otp:admin:{}:{}", claims.sub, proposal_id);
    let mut redis_conn = state.redis.clone();
    let _: () = redis_conn
        .set_ex(&redis_key, &otp_code, ttl_seconds)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    if let Some(producer) = &state.kafka_producer {
        let event = AdminOtpGeneratedEvent {
            admin_id: claims.sub,
            email: claims.email.clone(),
            proposal_id,
            target_admin_name: target_user.full_name,
            target_role: proposal.target_role.as_str().to_string(),
            otp_code: otp_code.clone(),
            ttl_seconds,
            generated_at: Utc::now(),
        };
        let payload_str = serde_json::to_string(&event).unwrap_or_default();
        let key_str = claims.sub.to_string();
        let record = FutureRecord::to(TOPIC_ADMIN_OTP_GENERATED)
            .key(&key_str)
            .payload(&payload_str);
        let _ = producer.send(record, std::time::Duration::from_secs(3)).await;
    }

    Ok(Json(json!({
        "message": "OTP has been generated and dispatched to your email address",
        "ttl_seconds": ttl_seconds,
        "proposal_id": proposal_id
    })))
}

#[derive(Deserialize)]
struct VoteProposalRequest {
    otp: String,
    decision: VoteDecision,
}

async fn vote_proposal_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(proposal_id): Path<Uuid>,
    Json(payload): Json<VoteProposalRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let claims = extract_claims(&headers, &state.jwt_secret)
        .await
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(json!({"error": "Unauthorized"}))))?;

    if claims.role != UserRole::Admin {
        return Err((StatusCode::FORBIDDEN, Json(json!({"error": "Admin role required"}))));
    }

    let redis_key = format!("otp:admin:{}:{}", claims.sub, proposal_id);
    let mut redis_conn = state.redis.clone();
    let stored_otp: Option<String> = redis_conn
        .get(&redis_key)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    if stored_otp.as_deref() != Some(payload.otp.trim()) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid or expired OTP code"})),
        ));
    }

    let _: () = redis_conn.del(&redis_key).await.unwrap_or(());

    let existing_vote: Option<(Uuid,)> = sqlx::query_as(
        "SELECT id FROM admin_downgrade_votes WHERE proposal_id = $1 AND admin_id = $2"
    )
    .bind(proposal_id)
    .bind(claims.sub)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    if existing_vote.is_some() {
        return Err((
            StatusCode::CONFLICT,
            Json(json!({"error": "You have already voted on this proposal"})),
        ));
    }

    sqlx::query(
        "INSERT INTO admin_downgrade_votes (proposal_id, admin_id, decision) VALUES ($1, $2, $3)"
    )
    .bind(proposal_id)
    .bind(claims.sub)
    .bind(payload.decision)
    .execute(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    let proposal = sqlx::query_as::<_, AdminDowngradeProposal>(
        "SELECT * FROM admin_downgrade_proposals WHERE id = $1"
    )
    .bind(proposal_id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    let mut current_approvals = proposal.current_approvals;
    if payload.decision == VoteDecision::Approve {
        current_approvals += 1;
    }

    let mut new_status = proposal.status;
    let mut role_downgraded = false;

    if current_approvals >= proposal.required_approvals {
        new_status = ProposalStatus::Approved;
        sqlx::query("UPDATE users SET role = $1, updated_at = NOW() WHERE id = $2")
            .bind(proposal.target_role)
            .bind(proposal.target_admin_id)
            .execute(&state.db)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;
        role_downgraded = true;
    }

    sqlx::query(
        "UPDATE admin_downgrade_proposals SET current_approvals = $1, status = $2 WHERE id = $3"
    )
    .bind(current_approvals)
    .bind(new_status)
    .bind(proposal_id)
    .execute(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({
        "message": "Vote recorded successfully",
        "current_approvals": current_approvals,
        "required_approvals": proposal.required_approvals,
        "proposal_status": new_status,
        "role_downgraded": role_downgraded
    })))
}

// ------------------------------------------
// Event Management & Voting
// ------------------------------------------

#[derive(Deserialize)]
struct CreateEventRequest {
    title: String,
    description: Option<String>,
    start_date: chrono::NaiveDate,
    end_date: chrono::NaiveDate,
    time_slots: Vec<CreateSlotItem>,
}

#[derive(Deserialize)]
struct CreateSlotItem {
    day_of_week: String,
    slot_label: String,
    sort_order: Option<i32>,
}

async fn list_events_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Event>>, (StatusCode, Json<Value>)> {
    let events = sqlx::query_as::<_, Event>("SELECT * FROM events ORDER BY created_at DESC")
        .fetch_all(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(events))
}

async fn create_event_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<CreateEventRequest>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    let claims = extract_claims(&headers, &state.jwt_secret)
        .await
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(json!({"error": "Unauthorized"}))))?;

    if claims.role != UserRole::Admin && claims.role != UserRole::Moderator {
        return Err((StatusCode::FORBIDDEN, Json(json!({"error": "Admin or Moderator role required"}))));
    }

    let event = sqlx::query_as::<_, Event>(
        r#"
        INSERT INTO events (title, description, start_date, end_date, status, created_by)
        VALUES ($1, $2, $3, $4, 'open', $5)
        RETURNING *
        "#
    )
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(payload.start_date)
    .bind(payload.end_date)
    .bind(claims.sub)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    for (idx, slot) in payload.time_slots.into_iter().enumerate() {
        let sort_order = slot.sort_order.unwrap_or(idx as i32);
        sqlx::query(
            "INSERT INTO event_time_slots (event_id, day_of_week, slot_label, sort_order) VALUES ($1, $2, $3, $4)"
        )
        .bind(event.id)
        .bind(slot.day_of_week)
        .bind(slot.slot_label)
        .bind(sort_order)
        .execute(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;
    }

    Ok((StatusCode::CREATED, Json(json!(event))))
}

async fn close_event_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(event_id): Path<Uuid>,
) -> Result<Json<Event>, (StatusCode, Json<Value>)> {
    let claims = extract_claims(&headers, &state.jwt_secret)
        .await
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(json!({"error": "Unauthorized"}))))?;

    if claims.role != UserRole::Admin && claims.role != UserRole::Moderator {
        return Err((StatusCode::FORBIDDEN, Json(json!({"error": "Admin or Moderator role required"}))));
    }

    let updated = sqlx::query_as::<_, Event>(
        "UPDATE events SET status = 'closed', closed_at = NOW() WHERE id = $1 RETURNING *"
    )
    .bind(event_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Event not found"}))))?;

    if let Some(producer) = &state.kafka_producer {
        let event_msg = EventStatusChangedEvent {
            event_id: updated.id,
            title: updated.title.clone(),
            old_status: "open".to_string(),
            new_status: "closed".to_string(),
            changed_by: Some(claims.sub),
            changed_at: Utc::now(),
        };
        let payload_str = serde_json::to_string(&event_msg).unwrap_or_default();
        let event_key = updated.id.to_string();
        let record = FutureRecord::to(TOPIC_EVENT_STATUS_CHANGED)
            .key(&event_key)
            .payload(&payload_str);
        let _ = producer.send(record, std::time::Duration::from_secs(3)).await;
    }

    Ok(Json(updated))
}

#[derive(Deserialize)]
struct SubmitVoteRequest {
    slot_id: Uuid,
    is_available: bool,
    note: Option<String>,
}

async fn submit_vote_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(event_id): Path<Uuid>,
    Json(payload): Json<SubmitVoteRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let claims = extract_claims(&headers, &state.jwt_secret)
        .await
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(json!({"error": "Unauthorized"}))))?;

    let event = sqlx::query_as::<_, Event>("SELECT * FROM events WHERE id = $1")
        .bind(event_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Event not found"}))))?;

    if event.status == EventStatus::Closed {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({"error": "Voting for this event is closed"})),
        ));
    }

    sqlx::query(
        r#"
        INSERT INTO votes (event_id, user_id, slot_id, is_available, note, updated_at)
        VALUES ($1, $2, $3, $4, $5, NOW())
        ON CONFLICT (event_id, user_id, slot_id)
        DO UPDATE SET is_available = EXCLUDED.is_available, note = EXCLUDED.note, updated_at = NOW()
        "#
    )
    .bind(event_id)
    .bind(claims.sub)
    .bind(payload.slot_id)
    .bind(payload.is_available)
    .bind(payload.note)
    .execute(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({"status": "VOTE_RECORDED"})))
}

async fn get_event_details_handler(
    State(state): State<Arc<AppState>>,
    Path(event_id): Path<Uuid>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let event = sqlx::query_as::<_, Event>("SELECT * FROM events WHERE id = $1")
        .bind(event_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Event not found"}))))?;

    let slots = sqlx::query_as::<_, csac_common::EventTimeSlot>(
        "SELECT * FROM event_time_slots WHERE event_id = $1 ORDER BY sort_order ASC, created_at ASC"
    )
    .bind(event_id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({
        "event": event,
        "time_slots": slots,
    })))
}

// ------------------------------------------
// Timetable Solver Proxy
// ------------------------------------------

async fn solve_proxy(Json(payload): Json<Value>) -> Json<Value> {
    Json(json!({
        "status": "FORWARDED_TO_SCHEDULER",
        "mock_result": {
            "schedule": [],
            "unresolved": [],
            "stats": { "totalRequested": 0, "totalScheduled": 0 }
        },
        "payload": payload
    }))
}

// ------------------------------------------
// Music Numbers & Fleet Management Handlers
// ------------------------------------------

#[derive(Deserialize)]
struct CreateMusicNumberRequest {
    event_id: Option<Uuid>,
    title: String,
    genre: Option<String>,
    pm_user_id: Option<Uuid>,
    target_sessions_per_week: Option<i32>,
    description: Option<String>,
}

async fn list_music_numbers_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let numbers = sqlx::query_as::<_, csac_common::MusicNumber>(
        "SELECT * FROM music_numbers ORDER BY created_at DESC"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    // Attach member lineups
    let mut result = Vec::new();
    for num in numbers {
        let members = sqlx::query_as::<_, csac_common::MusicNumberMember>(
            "SELECT * FROM music_number_members WHERE music_number_id = $1 ORDER BY is_lead DESC, created_at ASC"
        )
        .bind(num.id)
        .fetch_all(&state.db)
        .await
        .unwrap_or_default();

        result.push(json!({
            "number": num,
            "members": members,
        }));
    }

    Ok(Json(json!(result)))
}

async fn create_music_number_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<CreateMusicNumberRequest>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    let claims = extract_claims(&headers, &state.jwt_secret).await
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(json!({"error": "Unauthorized"}))))?;

    let pm_id = payload.pm_user_id.unwrap_or(claims.sub);

    let number = sqlx::query_as::<_, csac_common::MusicNumber>(
        "INSERT INTO music_numbers (event_id, title, genre, pm_user_id, target_sessions_per_week, description)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING *"
    )
    .bind(payload.event_id)
    .bind(&payload.title)
    .bind(payload.genre)
    .bind(pm_id)
    .bind(payload.target_sessions_per_week.unwrap_or(2))
    .bind(payload.description)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok((StatusCode::CREATED, Json(json!(number))))
}

// ------------------------------------------
// Instrument Fleet Handlers
// ------------------------------------------

#[derive(Deserialize)]
struct RegisterInstrumentRequest {
    name: String,
    code: String,
    category: String,
    ownership_type: csac_common::InstrumentOwnership,
    owner_user_id: Option<Uuid>,
    custody_user_id: Option<Uuid>,
    custody_location: Option<String>,
    availability_status: Option<csac_common::InstrumentAvailability>,
    notes: Option<String>,
}

#[derive(Deserialize)]
struct UpdateInstrumentStatusRequest {
    availability_status: Option<csac_common::InstrumentAvailability>,
    custody_user_id: Option<Uuid>,
    custody_location: Option<String>,
    notes: Option<String>,
}

#[derive(Deserialize)]
struct ReserveInstrumentRequest {
    instrument_id: Uuid,
    music_number_id: Uuid,
    day_of_week: String,
    slot_label: String,
    notes: Option<String>,
}

async fn list_instruments_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let instruments = sqlx::query_as::<_, csac_common::Instrument>(
        "SELECT * FROM instruments ORDER BY category ASC, name ASC"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    let reservations = sqlx::query_as::<_, csac_common::InstrumentReservation>(
        "SELECT * FROM instrument_reservations ORDER BY created_at DESC"
    )
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "instruments": instruments,
        "reservations": reservations,
    })))
}

async fn register_instrument_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<RegisterInstrumentRequest>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    let claims = extract_claims(&headers, &state.jwt_secret).await
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(json!({"error": "Unauthorized"}))))?;

    let owner_id = match payload.ownership_type {
        csac_common::InstrumentOwnership::MemberOwned => Some(payload.owner_user_id.unwrap_or(claims.sub)),
        csac_common::InstrumentOwnership::ClubProperty => None,
    };

    let custody_id = payload.custody_user_id.or(owner_id);
    let availability = payload.availability_status.unwrap_or(csac_common::InstrumentAvailability::FreeToBorrow);

    let inst = sqlx::query_as::<_, csac_common::Instrument>(
        "INSERT INTO instruments (name, code, category, ownership_type, owner_user_id, custody_user_id, custody_location, availability_status, notes)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
         RETURNING *"
    )
    .bind(&payload.name)
    .bind(&payload.code)
    .bind(&payload.category)
    .bind(payload.ownership_type)
    .bind(owner_id)
    .bind(custody_id)
    .bind(payload.custody_location.as_deref().unwrap_or("Club Studio Locker"))
    .bind(availability)
    .bind(payload.notes)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok((StatusCode::CREATED, Json(json!(inst))))
}

async fn update_instrument_status_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
    Json(payload): Json<UpdateInstrumentStatusRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let _claims = extract_claims(&headers, &state.jwt_secret).await
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(json!({"error": "Unauthorized"}))))?;

    let inst = sqlx::query_as::<_, csac_common::Instrument>(
        "UPDATE instruments
         SET availability_status = COALESCE($1, availability_status),
             custody_user_id = COALESCE($2, custody_user_id),
             custody_location = COALESCE($3, custody_location),
             notes = COALESCE($4, notes),
             updated_at = NOW()
         WHERE id = $5
         RETURNING *"
    )
    .bind(payload.availability_status)
    .bind(payload.custody_user_id)
    .bind(payload.custody_location)
    .bind(payload.notes)
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Instrument not found"}))))?;

    Ok(Json(json!(inst)))
}

async fn reserve_instrument_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<ReserveInstrumentRequest>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    let claims = extract_claims(&headers, &state.jwt_secret).await
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(json!({"error": "Unauthorized"}))))?;

    // Check if already reserved in this slot
    let conflict = sqlx::query(
        "SELECT id FROM instrument_reservations WHERE instrument_id = $1 AND day_of_week = $2 AND slot_label = $3"
    )
    .bind(payload.instrument_id)
    .bind(&payload.day_of_week)
    .bind(&payload.slot_label)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    if conflict.is_some() {
        return Err((
            StatusCode::CONFLICT,
            Json(json!({
                "error": "Instrument is already reserved for this day and time slot by another music number",
                "instrument_id": payload.instrument_id,
                "day_of_week": payload.day_of_week,
                "slot_label": payload.slot_label,
            })),
        ));
    }

    let res = sqlx::query_as::<_, csac_common::InstrumentReservation>(
        "INSERT INTO instrument_reservations (instrument_id, music_number_id, reserved_by, day_of_week, slot_label, notes)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING *"
    )
    .bind(payload.instrument_id)
    .bind(payload.music_number_id)
    .bind(claims.sub)
    .bind(payload.day_of_week)
    .bind(payload.slot_label)
    .bind(payload.notes)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok((StatusCode::CREATED, Json(json!(res))))
}

// ------------------------------------------
// Agile Practice Sprint & QC Handlers
// ------------------------------------------

#[derive(Deserialize)]
struct CreatePracticeTaskRequest {
    music_number_id: Uuid,
    task_type: csac_common::TaskType,
    title: String,
    description: Option<String>,
    assigned_to: Option<Uuid>,
    qc_reviewer_id: Option<Uuid>,
}

#[derive(Deserialize)]
struct ReviewTaskRequest {
    status: csac_common::TaskStatus,
    qc_feedback: Option<String>,
}

async fn list_sprints_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let sprints = sqlx::query_as::<_, csac_common::PracticeSprint>(
        "SELECT * FROM practice_sprints ORDER BY start_date ASC"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!(sprints)))
}

async fn list_sprint_tasks_handler(
    State(state): State<Arc<AppState>>,
    Path(sprint_id): Path<Uuid>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let tasks = sqlx::query_as::<_, csac_common::PracticeTask>(
        "SELECT * FROM practice_tasks WHERE sprint_id = $1 ORDER BY created_at ASC"
    )
    .bind(sprint_id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!(tasks)))
}

async fn create_sprint_task_handler(
    State(state): State<Arc<AppState>>,
    Path(sprint_id): Path<Uuid>,
    headers: HeaderMap,
    Json(payload): Json<CreatePracticeTaskRequest>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    let _claims = extract_claims(&headers, &state.jwt_secret).await
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(json!({"error": "Unauthorized"}))))?;

    let task = sqlx::query_as::<_, csac_common::PracticeTask>(
        "INSERT INTO practice_tasks (sprint_id, music_number_id, task_type, title, description, assigned_to, qc_reviewer_id)
         VALUES ($1, $2, $3, $4, $5, $6, $7)
         RETURNING *"
    )
    .bind(sprint_id)
    .bind(payload.music_number_id)
    .bind(payload.task_type)
    .bind(&payload.title)
    .bind(payload.description)
    .bind(payload.assigned_to)
    .bind(payload.qc_reviewer_id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok((StatusCode::CREATED, Json(json!(task))))
}

async fn review_task_handler(
    State(state): State<Arc<AppState>>,
    Path((_sprint_id, task_id)): Path<(Uuid, Uuid)>,
    headers: HeaderMap,
    Json(payload): Json<ReviewTaskRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let _claims = extract_claims(&headers, &state.jwt_secret).await
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(json!({"error": "Unauthorized"}))))?;

    let task = sqlx::query_as::<_, csac_common::PracticeTask>(
        "UPDATE practice_tasks
         SET status = $1,
             qc_feedback = $2,
             reviewed_at = NOW(),
             updated_at = NOW()
         WHERE id = $3
         RETURNING *"
    )
    .bind(payload.status)
    .bind(payload.qc_feedback)
    .bind(task_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Task not found"}))))?;

    Ok(Json(json!(task)))
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct SprintAvailabilitySlot {
    day_of_week: String,
    slot_label: String,
    is_available: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct SprintAvailabilityRequest {
    slots: Vec<SprintAvailabilitySlot>,
}

async fn submit_sprint_availability_handler(
    State(state): State<Arc<AppState>>,
    Path(sprint_id): Path<String>,
    headers: HeaderMap,
    Json(payload): Json<SprintAvailabilityRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let claims = extract_claims(&headers, &state.jwt_secret).await;
    let user_id = claims.map(|c| c.sub).unwrap_or_else(Uuid::nil);

    if let Ok(parsed_sprint_uuid) = Uuid::parse_str(&sprint_id) {
        if user_id != Uuid::nil() {
            for slot in &payload.slots {
                let _ = sqlx::query(
                    "INSERT INTO member_sprint_availabilities (sprint_id, user_id, day_of_week, slot_label, is_available)
                     VALUES ($1, $2, $3, $4, $5)
                     ON CONFLICT (sprint_id, user_id, day_of_week, slot_label)
                     DO UPDATE SET is_available = EXCLUDED.is_available, updated_at = NOW()"
                )
                .bind(parsed_sprint_uuid)
                .bind(user_id)
                .bind(&slot.day_of_week)
                .bind(&slot.slot_label)
                .bind(slot.is_available)
                .execute(&state.db)
                .await;
            }
        }
    }

    let active_slots_count = payload.slots.iter().filter(|s| s.is_available).count();
    let total_hours = (active_slots_count as f64) * 0.25;

    Ok(Json(json!({
        "status": "saved",
        "message": "Sprint availability recorded",
        "sprint_id": sprint_id,
        "total_slots": payload.slots.len(),
        "active_slots": active_slots_count,
        "total_hours": total_hours
    })))
}

async fn sprint_schedule_handler(
    Path(sprint_id): Path<String>,
) -> (StatusCode, Json<Value>) {
    let run_id = Uuid::new_v4();
    (
        StatusCode::ACCEPTED,
        Json(json!({
            "run_id": run_id,
            "sprint_id": sprint_id,
            "status": "queued",
            "message": "Sprint schedule calculation job enqueued asynchronously"
        })),
    )
}

async fn sprint_schedule_sse_handler(
    Path(sprint_id): Path<String>,
) -> Sse<impl futures_util::stream::Stream<Item = Result<SseEvent, std::convert::Infallible>>> {
    let stream = futures_util::stream::iter(vec![
        Ok(SseEvent::default().event("schedule_status").data(format!(r#"{{"sprint_id":"{}","status":"queued"}}"#, sprint_id))),
        Ok(SseEvent::default().event("schedule_status").data(format!(r#"{{"sprint_id":"{}","status":"processing"}}"#, sprint_id))),
        Ok(SseEvent::default().event("schedule_updated").data(format!(r#"{{"sprint_id":"{}","status":"completed","score":96.5,"conflict_count":0}}"#, sprint_id))),
    ]);

    Sse::new(stream)
}

// ==========================================
// Show Studio Handlers (/api/v1/shows/*)
// ==========================================

#[derive(Serialize, Deserialize, Clone)]
pub struct ShowOverviewData {
    pub id: String,
    pub title: String,
    pub venue: String,
    pub dates: String,
    pub readiness_percent: i32,
    pub total_numbers: usize,
    pub total_hours: i32,
    pub qc_approved_count: usize,
    pub highlights: Vec<Value>,
    pub milestones: Vec<Value>,
}

async fn get_show_overview_handler(
    Path(show_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let overview = json!({
        "id": show_id,
        "title": "CSAC Annual Concert 2026",
        "venue": "CSAC Main Auditorium",
        "dates": "Oct 1 - Oct 15, 2026",
        "readiness_percent": 75,
        "total_numbers": 12,
        "total_hours": 48,
        "qc_approved_count": 9,
        "highlights": [
            {
                "title": "\"Hào Khí Việt Nam\" (Grand Symphony)",
                "meta": "Leader (PM): Minh Pháp • Band: Full Orchestra",
                "stage": "stage_ready",
                "badge": "Stage Ready"
            },
            {
                "title": "\"Đi Giữa Trời Rực Rỡ\" (Pop Rock)",
                "meta": "Leader (PM): Hoàng Nam • Drums: Thu Hà",
                "stage": "qc_approved",
                "badge": "QC Approved"
            },
            {
                "title": "\"Giọt Sương Trên Mí Mắt\" (Acoustic Quartet)",
                "meta": "Leader (PM): Bảo Anh • Guitar: Tùng Dương",
                "stage": "in_practice",
                "badge": "In Practice"
            }
        ],
        "milestones": [
            {
                "title": "Sprint 1: Song Arrangement & Scratch Demo",
                "date": "Completed Sept 15, 2026",
                "status": "done"
            },
            {
                "title": "Sprint 2: Band Rehearsals & Vocal Harmonies",
                "date": "Completed Sept 25, 2026",
                "status": "done"
            },
            {
                "title": "Sprint 3: Quality Check (QC) Stage Audits",
                "date": "In Progress (Ends Oct 02)",
                "status": "active"
            }
        ]
    });

    Ok(Json(overview))
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SongNumberDto {
    pub id: String,
    pub title: String,
    pub genre: String,
    pub pmName: String,
    pub stage: String,
    pub qcReviewer: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qcNotes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lineup: Option<Value>,
}

async fn list_show_numbers_handler(
    Path(_show_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let numbers = json!([
        {
            "id": "num-1",
            "title": "Hào Khí Việt Nam",
            "genre": "Epic Symphony Rock",
            "pmName": "Minh Pháp",
            "stage": "stage_ready",
            "qcReviewer": "Hoàng Nam",
            "qcNotes": "Flawless vocal harmonies and drum fills. Stage ready.",
            "lineup": { "vocalLead": "Minh Pháp", "guitarLead": "Hoàng Nam", "bass": "Bảo Anh", "drums": "Thu Hà" }
        },
        {
            "id": "num-2",
            "title": "Đi Giữa Trời Rực Rỡ",
            "genre": "Pop Rock",
            "pmName": "Hoàng Nam",
            "stage": "qc_approved",
            "qcReviewer": "Thu Hà",
            "qcNotes": "Lead guitar solo approved. Dynamic balance is balanced.",
            "lineup": { "vocalLead": "Gia Huy", "guitarLead": "Hoàng Nam", "bass": "Bảo Anh", "drums": "Thu Hà" }
        },
        {
            "id": "num-3",
            "title": "Giọt Sương Trên Mí Mắt",
            "genre": "Acoustic Quartet",
            "pmName": "Bảo Anh",
            "stage": "ready_for_qc",
            "qcReviewer": "Minh Pháp",
            "lineup": { "vocalLead": "Minh Pháp", "guitarLead": "Tùng Dương", "bass": "Bảo Anh" }
        },
        {
            "id": "num-4",
            "title": "Nối Vòng Tay Lớn",
            "genre": "Choral Folk Rock",
            "pmName": "Thu Hà",
            "stage": "in_practice",
            "qcReviewer": "Bảo Anh",
            "qcNotes": "Need tighter drum transitions in Chorus 2.",
            "lineup": { "vocalLead": "Anh Pha", "guitarLead": "Hoàng Nam", "bass": "Bảo Anh", "drums": "Thu Hà" }
        },
        {
            "id": "num-5",
            "title": "Túy Âm",
            "genre": "Future Bass Rock Fusion",
            "pmName": "Gia Huy",
            "stage": "stage_ready",
            "qcReviewer": "Minh Pháp",
            "qcNotes": "Synthesizer pads and bass groove calibrated perfectly.",
            "lineup": { "vocalLead": "Gia Huy", "bass": "Bảo Anh", "keys": "Phương Nhi", "drums": "Thu Hà" }
        },
        {
            "id": "num-6",
            "title": "Để Mị Nói Cho Mà Nghe",
            "genre": "Ethnic Pop Punk",
            "pmName": "Phương Nhi",
            "stage": "qc_approved",
            "qcReviewer": "Thu Hà",
            "qcNotes": "Flute & keyboard blend sounds crisp.",
            "lineup": { "vocalLead": "Phương Nhi", "guitarLead": "Hoàng Nam", "bass": "Bảo Anh" }
        },
        {
            "id": "num-7",
            "title": "Bài Ca Hy Vọng",
            "genre": "Chamber Vocal Ensemble",
            "pmName": "Minh Pháp",
            "stage": "ready_for_qc",
            "qcReviewer": "Hoàng Nam",
            "lineup": { "vocalLead": "Minh Pháp", "keys": "Phương Nhi" }
        },
        {
            "id": "num-8",
            "title": "Ngẫu Hứng Sông Hồng",
            "genre": "Progressive Folk Rock",
            "pmName": "Hoàng Nam",
            "stage": "in_practice",
            "qcReviewer": "Minh Pháp",
            "lineup": { "vocalLead": "Anh Pha", "guitarLead": "Hoàng Nam", "drums": "Thu Hà" }
        },
        {
            "id": "num-9",
            "title": "Góc Ban Công",
            "genre": "Indie Pop Ballad",
            "pmName": "Bảo Anh",
            "stage": "in_practice",
            "qcReviewer": "Thu Hà",
            "lineup": { "vocalLead": "Bảo Anh", "guitarLead": "Tùng Dương" }
        },
        {
            "id": "num-10",
            "title": "Mặt Trời Bé Con",
            "genre": "Acoustic Duo",
            "pmName": "Tùng Dương",
            "stage": "stage_ready",
            "qcReviewer": "Bảo Anh",
            "qcNotes": "Acoustic fingerstyle guitar approved for stage soundcheck.",
            "lineup": { "vocalLead": "Thu Hà", "guitarLead": "Tùng Dương" }
        },
        {
            "id": "num-11",
            "title": "Tháng Mười Hai",
            "genre": "Alternative Rock",
            "pmName": "Gia Huy",
            "stage": "draft",
            "qcReviewer": "Hoàng Nam",
            "lineup": { "vocalLead": "Gia Huy", "guitarLead": "Hoàng Nam" }
        },
        {
            "id": "num-12",
            "title": "Đất Nước Trọn Niềm Vui",
            "genre": "Orchestral Overture",
            "pmName": "Minh Pháp",
            "stage": "ready_for_qc",
            "qcReviewer": "Thu Hà",
            "lineup": { "vocalLead": "Minh Pháp", "keys": "Phương Nhi", "drums": "Thu Hà" }
        },
        {
            "id": "num-13",
            "title": "Khát Vọng Tuổi Trẻ",
            "genre": "Youth Anthem Pop",
            "pmName": "Anh Pha",
            "stage": "draft",
            "qcReviewer": "Minh Pháp",
            "lineup": { "vocalLead": "Anh Pha" }
        },
        {
            "id": "num-14",
            "title": "Khoảnh Khắc",
            "genre": "Acoustic Soul",
            "pmName": "Thu Hà",
            "stage": "in_practice",
            "qcReviewer": "Gia Huy",
            "lineup": { "vocalLead": "Thu Hà", "guitarLead": "Tùng Dương", "bass": "Bảo Anh" }
        }
    ]);

    Ok(Json(numbers))
}

#[derive(Deserialize)]
struct CreateShowNumberReq {
    title: String,
    genre: Option<String>,
    pm_name: Option<String>,
    qc_reviewer: Option<String>,
}

async fn create_show_number_handler(
    Path(_show_id): Path<String>,
    Json(payload): Json<CreateShowNumberReq>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    let new_song = json!({
        "id": format!("num-{}", Utc::now().timestamp_millis()),
        "title": payload.title,
        "genre": payload.genre.unwrap_or_else(|| "Live Performance".to_string()),
        "pmName": payload.pm_name.unwrap_or_else(|| "Minh Pháp".to_string()),
        "stage": "draft",
        "qcReviewer": payload.qc_reviewer.unwrap_or_else(|| "Hoàng Nam".to_string()),
        "lineup": {}
    });

    Ok((StatusCode::CREATED, Json(new_song)))
}

#[derive(Deserialize)]
struct UpdateStageReq {
    stage: String,
}

async fn update_show_number_stage_handler(
    Path((_show_id, number_id)): Path<(String, String)>,
    Json(payload): Json<UpdateStageReq>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    Ok(Json(json!({
        "id": number_id,
        "stage": payload.stage,
        "updated_at": Utc::now()
    })))
}

#[derive(Deserialize)]
struct UpdateLineupReq {
    #[serde(rename = "vocalLead")]
    vocal_lead: Option<String>,
    #[serde(rename = "guitarLead")]
    guitar_lead: Option<String>,
    bass: Option<String>,
    drums: Option<String>,
    keys: Option<String>,
}

async fn update_show_number_lineup_handler(
    Path((_show_id, number_id)): Path<(String, String)>,
    Json(payload): Json<UpdateLineupReq>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    Ok(Json(json!({
        "id": number_id,
        "lineup": {
            "vocalLead": payload.vocal_lead,
            "guitarLead": payload.guitar_lead,
            "bass": payload.bass,
            "drums": payload.drums,
            "keys": payload.keys
        },
        "updated_at": Utc::now()
    })))
}

#[derive(Deserialize)]
struct SubmitQcReq {
    verdict: String,
    notes: Option<String>,
}

async fn submit_show_number_qc_handler(
    Path((_show_id, number_id)): Path<(String, String)>,
    Json(payload): Json<SubmitQcReq>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let next_stage = if payload.verdict == "pass" {
        "qc_approved"
    } else {
        "in_practice"
    };

    Ok(Json(json!({
        "id": number_id,
        "stage": next_stage,
        "qcNotes": payload.notes,
        "reviewed_at": Utc::now()
    })))
}

async fn list_show_roster_handler(
    Path(_show_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let roster = json!([
        {
            "id": "mem-1",
            "userId": "u-101",
            "fullName": "Minh Pháp",
            "email": "minhphap@csac.local",
            "phone": "+84 901 234 567",
            "showRole": "DM",
            "isDM": true,
            "pmSongTitles": ["Hào Khí Việt Nam", "Bài Ca Hy Vọng"],
            "qcSongTitles": ["Giọt Sương Trên Mí Mắt", "Túy Âm"],
            "primaryInstrument": "vocal_lead",
            "secondaryInstruments": ["guitar_rhythm"],
            "assignedSongCount": 5,
            "assignedSongTitles": ["Hào Khí Việt Nam", "Đi Giữa Trời Rực Rỡ", "Nối Vòng Tay Lớn", "Bài Ca Hy Vọng", "Dấu Chân Phía Trước"],
            "totalPracticeHours": 24,
            "workloadStatus": "fatigued",
            "attendanceRate": 98,
            "joinedAt": "2026-08-15"
        },
        {
            "id": "mem-2",
            "userId": "u-102",
            "fullName": "Hoàng Nam",
            "email": "hoangnam@csac.local",
            "phone": "+84 912 345 678",
            "showRole": "PM",
            "isDM": false,
            "pmSongTitles": ["Đi Giữa Trời Rực Rỡ", "Ngẫu Hứng Sông Hồng"],
            "qcSongTitles": ["Hào Khí Việt Nam"],
            "primaryInstrument": "guitar_lead",
            "secondaryInstruments": ["guitar_rhythm"],
            "assignedSongCount": 3,
            "assignedSongTitles": ["Hào Khí Việt Nam", "Đi Giữa Trời Rực Rỡ", "Khát Vọng Tuổi Trẻ"],
            "totalPracticeHours": 16,
            "workloadStatus": "moderate",
            "attendanceRate": 94,
            "joinedAt": "2026-08-18"
        },
        {
            "id": "mem-3",
            "userId": "u-103",
            "fullName": "Bảo Anh",
            "email": "baoanh@csac.local",
            "phone": "+84 934 567 890",
            "showRole": "PM",
            "isDM": false,
            "pmSongTitles": ["Giọt Sương Trên Mí Mắt", "Góc Ban Công"],
            "qcSongTitles": ["Nối Vòng Tay Lớn"],
            "primaryInstrument": "bass",
            "secondaryInstruments": ["guitar_lead"],
            "assignedSongCount": 4,
            "assignedSongTitles": ["Hào Khí Việt Nam", "Đi Giữa Trời Rực Rỡ", "Nối Vòng Tay Lớn", "Rock Vầng Trăng"],
            "totalPracticeHours": 18,
            "workloadStatus": "moderate",
            "attendanceRate": 92,
            "joinedAt": "2026-08-20"
        },
        {
            "id": "mem-4",
            "userId": "u-104",
            "fullName": "Thu Hà",
            "email": "thuha@csac.local",
            "phone": "+84 945 678 901",
            "showRole": "QC",
            "isDM": false,
            "pmSongTitles": ["Nối Vòng Tay Lớn", "Khoảnh Khắc"],
            "qcSongTitles": ["Đi Giữa Trời Rực Rỡ", "Để Mị Nói Cho Mà Nghe"],
            "primaryInstrument": "drums",
            "secondaryInstruments": ["percussion"],
            "assignedSongCount": 2,
            "assignedSongTitles": ["Hào Khí Việt Nam", "Nối Vòng Tay Lớn"],
            "totalPracticeHours": 10,
            "workloadStatus": "optimal",
            "attendanceRate": 100,
            "joinedAt": "2026-08-22"
        },
        {
            "id": "mem-5",
            "userId": "u-105",
            "fullName": "Khánh Linh",
            "email": "khanhlinh@csac.local",
            "phone": "+84 956 789 012",
            "showRole": "Performer",
            "isDM": false,
            "primaryInstrument": "vocal_harmony",
            "secondaryInstruments": ["keys"],
            "assignedSongCount": 2,
            "assignedSongTitles": ["Hào Khí Việt Nam", "Bài Ca Hy Vọng"],
            "totalPracticeHours": 8,
            "workloadStatus": "optimal",
            "attendanceRate": 95,
            "joinedAt": "2026-08-25"
        },
        {
            "id": "mem-6",
            "userId": "u-106",
            "fullName": "Quốc Bảo",
            "email": "quocbao@csac.local",
            "phone": "+84 967 890 123",
            "showRole": "Performer",
            "isDM": false,
            "primaryInstrument": "keys",
            "secondaryInstruments": ["sound_tech"],
            "assignedSongCount": 3,
            "assignedSongTitles": ["Hào Khí Việt Nam", "Đi Giữa Trời Rực Rỡ", "Bài Ca Hy Vọng"],
            "totalPracticeHours": 14,
            "workloadStatus": "moderate",
            "attendanceRate": 90,
            "joinedAt": "2026-08-27"
        },
        {
            "id": "mem-7",
            "userId": "u-107",
            "fullName": "Trọng Hiếu",
            "email": "tronghieu@csac.local",
            "phone": "+84 978 901 234",
            "showRole": "Performer",
            "isDM": false,
            "primaryInstrument": "sound_tech",
            "secondaryInstruments": [],
            "assignedSongCount": 1,
            "assignedSongTitles": ["Hào Khí Việt Nam (Live Audio)"],
            "totalPracticeHours": 6,
            "workloadStatus": "optimal",
            "attendanceRate": 100,
            "joinedAt": "2026-08-29"
        }
    ]);

    Ok(Json(roster))
}

#[derive(Deserialize)]
struct SaveRosterMemberReq {
    #[serde(rename = "fullName")]
    full_name: String,
    email: String,
    phone: Option<String>,
    #[serde(rename = "showRole")]
    show_role: String,
    #[serde(rename = "primaryInstrument")]
    primary_instrument: String,
    #[serde(rename = "secondaryInstruments")]
    secondary_instruments: Option<Vec<String>>,
    #[serde(rename = "practiceHours")]
    practice_hours: Option<i32>,
}

async fn create_show_roster_handler(
    Path(_show_id): Path<String>,
    Json(payload): Json<SaveRosterMemberReq>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    let new_member = json!({
        "id": format!("mem-{}", Utc::now().timestamp_millis()),
        "userId": format!("u-{}", Utc::now().timestamp_millis()),
        "fullName": payload.full_name,
        "email": payload.email,
        "phone": payload.phone.unwrap_or_default(),
        "showRole": payload.show_role,
        "isDM": payload.show_role == "DM",
        "primaryInstrument": payload.primary_instrument,
        "secondaryInstruments": payload.secondary_instruments.unwrap_or_default(),
        "assignedSongCount": 0,
        "assignedSongTitles": [],
        "totalPracticeHours": payload.practice_hours.unwrap_or(4),
        "workloadStatus": "optimal",
        "attendanceRate": 100,
        "joinedAt": Utc::now().format("%Y-%m-%d").to_string()
    });

    Ok((StatusCode::CREATED, Json(new_member)))
}

async fn update_show_roster_handler(
    Path((_show_id, member_id)): Path<(String, String)>,
    Json(payload): Json<SaveRosterMemberReq>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    Ok(Json(json!({
        "id": member_id,
        "fullName": payload.full_name,
        "email": payload.email,
        "phone": payload.phone.unwrap_or_default(),
        "showRole": payload.show_role,
        "isDM": payload.show_role == "DM",
        "primaryInstrument": payload.primary_instrument,
        "secondaryInstruments": payload.secondary_instruments.unwrap_or_default(),
        "totalPracticeHours": payload.practice_hours.unwrap_or(4),
        "updatedAt": Utc::now()
    })))
}

async fn delete_show_roster_handler(
    Path((_show_id, member_id)): Path<(String, String)>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    Ok(Json(json!({
        "status": "deleted",
        "memberId": member_id
    })))
}

async fn get_active_sprint_handler(
    Path(_show_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let sprint_data = json!({
        "sprint": {
            "id": "sprint-3",
            "name": "Sprint 3 (Stage QC & 15m Rehearsal Optimization)",
            "status": "active"
        },
        "selectedSlots": {
            "0_17:00": true,
            "0_17:15": true,
            "0_17:30": true,
            "0_17:45": true,
            "0_18:00": true,
            "0_18:15": true,
            "0_18:30": true,
            "0_18:45": true,
            "2_19:00": true,
            "2_19:15": true,
            "2_19:30": true,
            "2_19:45": true,
            "2_20:00": true,
            "2_20:15": true,
            "2_20:30": true,
            "2_20:45": true,
            "4_18:00": true,
            "4_18:15": true,
            "4_18:30": true,
            "4_18:45": true,
            "4_19:00": true,
            "4_19:15": true,
            "5_17:30": true,
            "5_17:45": true,
            "5_18:00": true,
            "5_18:15": true,
            "5_18:30": true,
            "5_18:45": true,
            "6_19:00": true,
            "6_19:15": true,
            "6_19:30": true,
            "6_19:45": true,
            "6_20:00": true,
            "6_20:15": true
        },
        "rehearsals": [
            {
                "id": "reh-1a",
                "songTitle": "Hào Khí Việt Nam",
                "sessionIndex": 1,
                "totalTargetRehearsals": 3,
                "dayIdx": 0,
                "dayName": "Monday",
                "startTime": "18:15",
                "endTime": "19:45",
                "durationMinutes": 90,
                "room": "Studio Room A",
                "pmName": "Minh Pháp",
                "performers": ["Minh Pháp (Vocal)", "Hoàng Nam (Guitar)", "Bảo Anh (Bass)", "Thu Hà (Drums)"],
                "status": "stage_ready",
                "color": "#ff6b00"
            },
            {
                "id": "reh-1b",
                "songTitle": "Hào Khí Việt Nam",
                "sessionIndex": 2,
                "totalTargetRehearsals": 3,
                "dayIdx": 2,
                "dayName": "Wednesday",
                "startTime": "19:30",
                "endTime": "21:00",
                "durationMinutes": 90,
                "room": "Studio Room A",
                "pmName": "Minh Pháp",
                "performers": ["Minh Pháp (Vocal)", "Hoàng Nam (Guitar)", "Bảo Anh (Bass)", "Thu Hà (Drums)"],
                "status": "stage_ready",
                "color": "#ff6b00"
            },
            {
                "id": "reh-1c",
                "songTitle": "Hào Khí Việt Nam",
                "sessionIndex": 3,
                "totalTargetRehearsals": 3,
                "dayIdx": 5,
                "dayName": "Saturday",
                "startTime": "17:00",
                "endTime": "18:30",
                "durationMinutes": 90,
                "room": "Studio Room A",
                "pmName": "Minh Pháp",
                "performers": ["Minh Pháp (Vocal)", "Hoàng Nam (Guitar)", "Bảo Anh (Bass)", "Thu Hà (Drums)"],
                "status": "stage_ready",
                "color": "#ff6b00"
            },
            {
                "id": "reh-2a",
                "songTitle": "Đi Giữa Trời Rực Rỡ",
                "sessionIndex": 1,
                "totalTargetRehearsals": 2,
                "dayIdx": 1,
                "dayName": "Tuesday",
                "startTime": "18:30",
                "endTime": "20:00",
                "durationMinutes": 90,
                "room": "Studio Room B",
                "pmName": "Hoàng Nam",
                "performers": ["Gia Huy (Vocal)", "Hoàng Nam (Guitar)", "Bảo Anh (Bass)", "Thu Hà (Drums)"],
                "status": "qc_approved",
                "color": "#2563eb"
            },
            {
                "id": "reh-2b",
                "songTitle": "Đi Giữa Trời Rực Rỡ",
                "sessionIndex": 2,
                "totalTargetRehearsals": 2,
                "dayIdx": 4,
                "dayName": "Friday",
                "startTime": "19:00",
                "endTime": "20:30",
                "durationMinutes": 90,
                "room": "Studio Room A",
                "pmName": "Hoàng Nam",
                "performers": ["Gia Huy (Vocal)", "Hoàng Nam (Guitar)", "Bảo Anh (Bass)", "Thu Hà (Drums)"],
                "status": "qc_approved",
                "color": "#2563eb"
            },
            {
                "id": "reh-3a",
                "songTitle": "Giọt Sương Trên Mí Mắt",
                "sessionIndex": 1,
                "totalTargetRehearsals": 2,
                "dayIdx": 0,
                "dayName": "Monday",
                "startTime": "20:00",
                "endTime": "21:30",
                "durationMinutes": 90,
                "room": "Studio Room B",
                "pmName": "Bảo Anh",
                "performers": ["Minh Pháp (Vocal)", "Tùng Dương (Guitar)", "Bảo Anh (Bass)"],
                "status": "ready_for_qc",
                "color": "#16a34a"
            },
            {
                "id": "reh-3b",
                "songTitle": "Giọt Sương Trên Mí Mắt",
                "sessionIndex": 2,
                "totalTargetRehearsals": 2,
                "dayIdx": 3,
                "dayName": "Thursday",
                "startTime": "18:15",
                "endTime": "19:45",
                "durationMinutes": 90,
                "room": "Studio Room A",
                "pmName": "Bảo Anh",
                "performers": ["Minh Pháp (Vocal)", "Tùng Dương (Guitar)", "Bảo Anh (Bass)"],
                "status": "ready_for_qc",
                "color": "#16a34a"
            },
            {
                "id": "reh-4a",
                "songTitle": "Nối Vòng Tay Lớn",
                "sessionIndex": 1,
                "totalTargetRehearsals": 2,
                "dayIdx": 1,
                "dayName": "Tuesday",
                "startTime": "17:00",
                "endTime": "18:30",
                "durationMinutes": 90,
                "room": "Studio Room A",
                "pmName": "Thu Hà",
                "performers": ["Anh Pha (Vocal)", "Hoàng Nam (Guitar)", "Bảo Anh (Bass)", "Thu Hà (Drums)"],
                "status": "in_practice",
                "color": "#9333ea"
            },
            {
                "id": "reh-4b",
                "songTitle": "Nối Vòng Tay Lớn",
                "sessionIndex": 2,
                "totalTargetRehearsals": 2,
                "dayIdx": 4,
                "dayName": "Friday",
                "startTime": "17:30",
                "endTime": "19:00",
                "durationMinutes": 90,
                "room": "Studio Room B",
                "pmName": "Thu Hà",
                "performers": ["Anh Pha (Vocal)", "Hoàng Nam (Guitar)", "Bảo Anh (Bass)", "Thu Hà (Drums)"],
                "status": "in_practice",
                "color": "#9333ea"
            },
            {
                "id": "reh-5a",
                "songTitle": "Túy Âm",
                "sessionIndex": 1,
                "totalTargetRehearsals": 2,
                "dayIdx": 2,
                "dayName": "Wednesday",
                "startTime": "18:00",
                "endTime": "19:30",
                "durationMinutes": 90,
                "room": "Studio Room B",
                "pmName": "Gia Huy",
                "performers": ["Gia Huy (Vocal)", "Bảo Anh (Bass)", "Phương Nhi (Keys)", "Thu Hà (Drums)"],
                "status": "stage_ready",
                "color": "#ea580c"
            },
            {
                "id": "reh-5b",
                "songTitle": "Túy Âm",
                "sessionIndex": 2,
                "totalTargetRehearsals": 2,
                "dayIdx": 6,
                "dayName": "Sunday",
                "startTime": "19:00",
                "endTime": "20:30",
                "durationMinutes": 90,
                "room": "Studio Room A",
                "pmName": "Gia Huy",
                "performers": ["Gia Huy (Vocal)", "Bảo Anh (Bass)", "Phương Nhi (Keys)", "Thu Hà (Drums)"],
                "status": "stage_ready",
                "color": "#ea580c"
            },
            {
                "id": "reh-6a",
                "songTitle": "Đất Nước Trọn Niềm Vui",
                "sessionIndex": 1,
                "totalTargetRehearsals": 2,
                "dayIdx": 3,
                "dayName": "Thursday",
                "startTime": "19:45",
                "endTime": "21:15",
                "durationMinutes": 90,
                "room": "Studio Room B",
                "pmName": "Minh Pháp",
                "performers": ["Minh Pháp (Vocal)", "Phương Nhi (Keys)", "Thu Hà (Drums)"],
                "status": "ready_for_qc",
                "color": "#0891b2"
            },
            {
                "id": "reh-6b",
                "songTitle": "Đất Nước Trọn Niềm Vui",
                "sessionIndex": 2,
                "totalTargetRehearsals": 2,
                "dayIdx": 5,
                "dayName": "Saturday",
                "startTime": "18:45",
                "endTime": "20:15",
                "durationMinutes": 90,
                "room": "Studio Room B",
                "pmName": "Minh Pháp",
                "performers": ["Minh Pháp (Vocal)", "Phương Nhi (Keys)", "Thu Hà (Drums)"],
                "status": "ready_for_qc",
                "color": "#0891b2"
            },
            {
                "id": "reh-7a",
                "songTitle": "Khoảnh Khắc",
                "sessionIndex": 1,
                "totalTargetRehearsals": 1,
                "dayIdx": 6,
                "dayName": "Sunday",
                "startTime": "17:30",
                "endTime": "19:00",
                "durationMinutes": 90,
                "room": "Studio Room B",
                "pmName": "Thu Hà",
                "performers": ["Thu Hà (Vocal)", "Tùng Dương (Guitar)", "Bảo Anh (Bass)"],
                "status": "in_practice",
                "color": "#4f46e5"
            }
        ]
    });

    Ok(Json(sprint_data))
}

async fn submit_show_sprint_availability_handler(
    State(state): State<Arc<AppState>>,
    Path((show_id, sprint_id)): Path<(String, String)>,
    headers: HeaderMap,
    Json(payload): Json<SprintAvailabilityRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let claims = extract_claims(&headers, &state.jwt_secret).await;
    let user_id = claims.map(|c| c.sub).unwrap_or_else(Uuid::nil);

    if let Ok(parsed_sprint_uuid) = Uuid::parse_str(&sprint_id) {
        if user_id != Uuid::nil() {
            for slot in &payload.slots {
                let _ = sqlx::query(
                    "INSERT INTO member_sprint_availabilities (sprint_id, user_id, day_of_week, slot_label, is_available)
                     VALUES ($1, $2, $3, $4, $5)
                     ON CONFLICT (sprint_id, user_id, day_of_week, slot_label)
                     DO UPDATE SET is_available = EXCLUDED.is_available, updated_at = NOW()"
                )
                .bind(parsed_sprint_uuid)
                .bind(user_id)
                .bind(&slot.day_of_week)
                .bind(&slot.slot_label)
                .bind(slot.is_available)
                .execute(&state.db)
                .await;
            }
        }
    }

    let active_slots_count = payload.slots.iter().filter(|s| s.is_available).count();
    let total_hours = (active_slots_count as f64) * 0.25;

    Ok(Json(json!({
        "status": "saved",
        "message": "Sprint availability recorded",
        "show_id": show_id,
        "sprint_id": sprint_id,
        "total_slots": payload.slots.len(),
        "active_slots": active_slots_count,
        "total_hours": total_hours
    })))
}

async fn get_show_sprint_history_handler(
    Path((_show_id, _sprint_id)): Path<(String, String)>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let history = json!({
        "compute_history": [
            {
                "id": "run-101",
                "sprintId": "sprint-3",
                "triggeredBy": "user-001",
                "triggeredByName": "Minh Pháp (DM)",
                "status": "completed",
                "durationMs": 420,
                "score": 98.5,
                "conflictCount": 0,
                "createdAt": "2026-09-12 09:30:15",
                "completedAt": "2026-09-12 09:30:16"
            },
            {
                "id": "run-100",
                "sprintId": "sprint-3",
                "triggeredBy": "user-002",
                "triggeredByName": "Hoàng Nam (Admin)",
                "status": "completed",
                "durationMs": 650,
                "score": 92.0,
                "conflictCount": 1,
                "createdAt": "2026-09-11 14:15:00",
                "completedAt": "2026-09-11 14:15:01"
            }
        ],
        "registration_history": [
            {
                "id": "reg-501",
                "sprintId": "sprint-3",
                "userId": "user-003",
                "userName": "Thu Hà (Member)",
                "actorId": "user-003",
                "actorName": "Thu Hà (Self)",
                "action": "ADD",
                "dayOfWeek": "Monday",
                "slotLabel": "18:15",
                "isAvailable": true,
                "createdAt": "2026-09-12 10:12:00"
            },
            {
                "id": "reg-502",
                "sprintId": "sprint-3",
                "userId": "user-004",
                "userName": "Tuấn Kiệt (PM)",
                "actorId": "user-001",
                "actorName": "Minh Pháp (DM)",
                "action": "UPDATE",
                "dayOfWeek": "Friday",
                "slotLabel": "19:00",
                "isAvailable": true,
                "createdAt": "2026-09-12 08:45:10"
            },
            {
                "id": "reg-503",
                "sprintId": "sprint-3",
                "userId": "user-005",
                "userName": "Bảo Anh (Member)",
                "actorId": "user-005",
                "actorName": "Bảo Anh (Self)",
                "action": "DELETE",
                "dayOfWeek": "Wednesday",
                "slotLabel": "21:00",
                "isAvailable": false,
                "createdAt": "2026-09-11 19:30:22"
            }
        ]
    });

    Ok(Json(history))
}

// ==========================================
// Main Server Entrypoint
// ==========================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://csac_admin:csac_password@localhost:5432/csac_timetable".to_string());
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
    let kafka_brokers = std::env::var("KAFKA_BROKERS").unwrap_or_else(|_| "localhost:9092".to_string());
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "default_jwt_secret_for_local_testing".to_string());

    tracing::info!("Connecting to PostgreSQL: {}", db_url);
    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&db_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    tracing::info!("Connecting to Redis: {}", redis_url);
    let redis_client = redis::Client::open(redis_url).expect("Invalid Redis URL");
    let redis_conn = redis::aio::ConnectionManager::new(redis_client)
        .await
        .expect("Failed to create Redis connection manager");

    tracing::info!("Initializing Kafka Producer: {}", kafka_brokers);
    let kafka_producer: Option<FutureProducer> = ClientConfig::new()
        .set("bootstrap.servers", &kafka_brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .ok();

    let oo_client = OpenObserveClient::new("gateway");

    let app_state = Arc::new(AppState {
        db,
        redis: redis_conn,
        kafka_producer,
        jwt_secret,
        oo_client,
    });

    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/api/v1/auth/login", post(login_handler))
        .route("/api/v1/auth/me", get(me_handler))
        .route("/api/v1/admin/users", get(list_users_handler).post(create_user_handler))
        .route("/api/v1/admin/users/:id/role", put(update_user_role_handler))
        .route("/api/v1/admin/users/:id/downgrade-proposal", post(create_downgrade_proposal_handler))
        .route("/api/v1/admin/approve/proposals", get(list_proposals_handler))
        .route("/api/v1/admin/approve/:id/request-otp", post(request_proposal_otp_handler))
        .route("/api/v1/admin/approve/:id/vote", post(vote_proposal_handler))
        .route("/api/v1/events", get(list_events_handler).post(create_event_handler))
        .route("/api/v1/events/:id", get(get_event_details_handler))
        .route("/api/v1/events/:id/close", put(close_event_handler))
        .route("/api/v1/events/:id/vote", post(submit_vote_handler))
        // Music Numbers & Fleet
        .route("/api/v1/music/numbers", get(list_music_numbers_handler).post(create_music_number_handler))
        .route("/api/v1/music/instruments", get(list_instruments_handler).post(register_instrument_handler))
        .route("/api/v1/music/instruments/:id/status", put(update_instrument_status_handler))
        .route("/api/v1/music/instruments/reserve", post(reserve_instrument_handler))
        // Agile Practice Sprints
        .route("/api/v1/sprints", get(list_sprints_handler))
        .route("/api/v1/sprints/:id/tasks", get(list_sprint_tasks_handler).post(create_sprint_task_handler))
        .route("/api/v1/sprints/:id/tasks/:task_id/review", put(review_task_handler))
        .route("/api/v1/sprints/:id/availability", post(submit_sprint_availability_handler))
        .route("/api/v1/sprints/:id/schedule", post(sprint_schedule_handler))
        .route("/api/v1/sprints/:id/schedule/stream", get(sprint_schedule_sse_handler))
        // Solver proxy
        .route("/api/v1/schedule/solve", post(solve_proxy))
        // Show Studio Routes (/api/v1/shows/*)
        .route("/api/v1/shows/:id/overview", get(get_show_overview_handler))
        .route("/api/v1/shows/:id/numbers", get(list_show_numbers_handler).post(create_show_number_handler))
        .route("/api/v1/shows/:id/numbers/:number_id/stage", put(update_show_number_stage_handler))
        .route("/api/v1/shows/:id/numbers/:number_id/lineup", put(update_show_number_lineup_handler))
        .route("/api/v1/shows/:id/numbers/:number_id/qc", post(submit_show_number_qc_handler))
        .route("/api/v1/shows/:id/roster", get(list_show_roster_handler).post(create_show_roster_handler))
        .route("/api/v1/shows/:id/roster/:member_id", put(update_show_roster_handler).delete(delete_show_roster_handler))
        .route("/api/v1/shows/:id/sprints/active", get(get_active_sprint_handler))
        .route("/api/v1/shows/:id/sprints/:sprint_id/availability", post(submit_show_sprint_availability_handler))
        .route("/api/v1/shows/:id/sprints/:sprint_id/history", get(get_show_sprint_history_handler))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            telemetry_middleware,
        ))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("CSAC Gateway reverse proxy listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

