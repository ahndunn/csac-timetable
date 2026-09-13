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
    State(state): State<Arc<AppState>>,
    Path(show_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let cache_key = format!("cache:shows:overview:{}", show_id);
    let mut redis_conn = state.redis.clone();

    // Check Redis cache first
    if let Ok(Some(cached_json)) = redis_conn.get::<_, Option<String>>(&cache_key).await {
        if let Ok(parsed) = serde_json::from_str::<Value>(&cached_json) {
            return Ok(Json(parsed));
        }
    }

    let event_uuid = Uuid::parse_str(&show_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(json!({"error": {"code": "INVALID_ID", "message": format!("Invalid show ID '{}'", show_id), "status": 400}}))))?;

    let event = sqlx::query_as::<_, Event>("SELECT * FROM events WHERE id = $1")
        .bind(event_uuid)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": {"code": "DB_ERROR", "message": e.to_string(), "status": 500}}))))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": {"code": "SHOW_NOT_FOUND", "message": format!("Show with ID '{}' not found", show_id), "status": 404}}))))?;

    let numbers = sqlx::query_as::<_, csac_common::MusicNumber>(
        "SELECT * FROM music_numbers WHERE event_id = $1 ORDER BY created_at ASC"
    )
    .bind(event_uuid)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    let total_numbers = numbers.len();
    let qc_approved_count = numbers.iter().filter(|n| n.status == csac_common::MusicNumberStatus::QcApproved || n.status == csac_common::MusicNumberStatus::StageReady).count();
    let readiness_percent = if total_numbers > 0 { (qc_approved_count * 100) / total_numbers } else { 0 };

    // Fetch dynamic highlights from music numbers
    let mut highlights = Vec::new();
    for num in numbers.iter().take(5) {
        let pm_user = if let Some(pm_id) = num.pm_user_id {
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
                .bind(pm_id)
                .fetch_optional(&state.db)
                .await
                .unwrap_or(None)
        } else {
            None
        };
        let pm_name = pm_user.map(|u| u.full_name).unwrap_or_else(|| "CSAC Core PM".to_string());
        let genre_str = num.genre.as_deref().unwrap_or("Band Rehearsal");

        let (stage_str, badge_str) = match num.status {
            csac_common::MusicNumberStatus::Draft => ("draft", "Draft"),
            csac_common::MusicNumberStatus::InPractice => ("in_practice", "In Practice"),
            csac_common::MusicNumberStatus::ReadyForQc => ("ready_for_qc", "Ready for QC"),
            csac_common::MusicNumberStatus::QcApproved => ("qc_approved", "QC Approved"),
            csac_common::MusicNumberStatus::StageReady => ("stage_ready", "Stage Ready"),
        };

        highlights.push(json!({
            "title": format!("\"{}\" ({})", num.title, genre_str),
            "meta": format!("Leader (PM): {} • Sessions/wk: {}", pm_name, num.target_sessions_per_week),
            "stage": stage_str,
            "badge": badge_str,
        }));
    }

    // Fetch dynamic milestones from practice sprints
    let sprints = sqlx::query_as::<_, csac_common::PracticeSprint>(
        "SELECT * FROM practice_sprints WHERE event_id = $1 ORDER BY start_date ASC"
    )
    .bind(event_uuid)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    let mut milestones = Vec::new();
    for sprint in sprints {
        let status_str = if sprint.is_active { "active" } else { "done" };
        let date_str = if sprint.is_active {
            format!("In Progress (Ends {})", sprint.end_date.format("%b %d"))
        } else {
            format!("Completed {}", sprint.end_date.format("%b %d, %Y"))
        };

        milestones.push(json!({
            "title": sprint.name,
            "date": date_str,
            "status": status_str,
        }));
    }

    let overview = json!({
        "id": event.id.to_string(),
        "title": event.title,
        "venue": "CSAC Main Auditorium",
        "dates": format!("{} -> {}", event.start_date, event.end_date),
        "readiness_percent": readiness_percent,
        "total_numbers": total_numbers,
        "total_hours": total_numbers * 4,
        "qc_approved_count": qc_approved_count,
        "highlights": highlights,
        "milestones": milestones,
    });

    // Cache into Redis with 60s TTL
    if let Ok(ser) = serde_json::to_string(&overview) {
        let _: Result<(), _> = redis_conn.set_ex(&cache_key, ser, 60).await;
    }

    Ok(Json(overview))
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SongNumberDto {
    pub id: String,
    pub title: String,
    pub genre: String,
    #[serde(rename = "pmName")]
    pub pm_name: String,
    pub stage: String,
    #[serde(rename = "qcReviewer")]
    pub qc_reviewer: String,
    #[serde(rename = "qcNotes", skip_serializing_if = "Option::is_none")]
    pub qc_notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lineup: Option<Value>,
}

async fn list_show_numbers_handler(
    State(state): State<Arc<AppState>>,
    Path(show_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let event_uuid = Uuid::parse_str(&show_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(json!({"error": {"code": "INVALID_ID", "message": format!("Invalid show ID '{}'", show_id), "status": 400}}))))?;

    let numbers = sqlx::query_as::<_, csac_common::MusicNumber>(
        "SELECT * FROM music_numbers WHERE event_id = $1 ORDER BY created_at ASC"
    )
    .bind(event_uuid)
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": {"code": "DB_ERROR", "message": e.to_string(), "status": 500}}))))?;

    let mut result = Vec::new();
    for num in numbers {
        let pm_user = if let Some(pm_id) = num.pm_user_id {
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
                .bind(pm_id)
                .fetch_optional(&state.db)
                .await
                .unwrap_or(None)
        } else {
            None
        };
        let pm_name = pm_user.map(|u| u.full_name).unwrap_or_else(|| "Minh Pháp".to_string());

        let members = sqlx::query_as::<_, csac_common::MusicNumberMember>(
            "SELECT * FROM music_number_members WHERE music_number_id = $1"
        )
        .bind(num.id)
        .fetch_all(&state.db)
        .await
        .unwrap_or_default();

        let mut lineup = serde_json::Map::new();
        for m in members {
            let u_row = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
                .bind(m.user_id)
                .fetch_optional(&state.db)
                .await
                .unwrap_or(None);
            let display_performer = u_row.map(|u| u.full_name).unwrap_or_else(|| "Assigned Performer".to_string());

            if m.instrument_role.to_lowercase().contains("vocal") {
                lineup.insert("vocalLead".to_string(), json!(display_performer));
            } else if m.instrument_role.to_lowercase().contains("guitar") {
                lineup.insert("guitarLead".to_string(), json!(display_performer));
            } else if m.instrument_role.to_lowercase().contains("bass") {
                lineup.insert("bass".to_string(), json!(display_performer));
            } else if m.instrument_role.to_lowercase().contains("drum") {
                lineup.insert("drums".to_string(), json!(display_performer));
            } else if m.instrument_role.to_lowercase().contains("key") || m.instrument_role.to_lowercase().contains("piano") {
                lineup.insert("keys".to_string(), json!(display_performer));
            }
        }

        // Check for latest QC audit notes in practice_tasks
        let qc_task = sqlx::query_as::<_, csac_common::PracticeTask>(
            "SELECT * FROM practice_tasks WHERE music_number_id = $1 AND task_type = 'review_qc' ORDER BY updated_at DESC LIMIT 1"
        )
        .bind(num.id)
        .fetch_optional(&state.db)
        .await
        .unwrap_or(None);

        let qc_reviewer_name = if let Some(ref t) = qc_task {
            if let Some(reviewer_id) = t.qc_reviewer_id {
                sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
                    .bind(reviewer_id)
                    .fetch_optional(&state.db)
                    .await
                    .unwrap_or(None)
                    .map(|u| u.full_name)
                    .unwrap_or_else(|| "Hoàng Nam".to_string())
            } else {
                "Hoàng Nam".to_string()
            }
        } else {
            "Hoàng Nam".to_string()
        };

        result.push(json!({
            "id": num.id.to_string(),
            "title": num.title,
            "genre": num.genre.unwrap_or_else(|| "Band Rehearsal".to_string()),
            "pmName": pm_name,
            "stage": match num.status {
                csac_common::MusicNumberStatus::Draft => "draft",
                csac_common::MusicNumberStatus::InPractice => "in_practice",
                csac_common::MusicNumberStatus::ReadyForQc => "ready_for_qc",
                csac_common::MusicNumberStatus::QcApproved => "qc_approved",
                csac_common::MusicNumberStatus::StageReady => "stage_ready",
            },
            "qcReviewer": qc_reviewer_name,
            "qcNotes": qc_task.and_then(|t| t.qc_feedback),
            "lineup": Value::Object(lineup)
        }));
    }

    Ok(Json(json!(result)))
}

#[derive(Deserialize)]
struct CreateShowNumberReq {
    title: String,
    genre: Option<String>,
    #[serde(rename = "pm_name")]
    pm_name: Option<String>,
    #[serde(rename = "qc_reviewer")]
    qc_reviewer: Option<String>,
}

async fn create_show_number_handler(
    State(state): State<Arc<AppState>>,
    Path(show_id): Path<String>,
    Json(payload): Json<CreateShowNumberReq>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    let event_uuid = Uuid::parse_str(&show_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(json!({"error": {"code": "INVALID_ID", "message": format!("Invalid show ID '{}'", show_id), "status": 400}}))))?;

    // Invalidate Redis overview cache for this show
    let mut redis_conn = state.redis.clone();
    let _: Result<(), _> = redis_conn.del(format!("cache:shows:overview:{}", show_id)).await;

    let new_num = sqlx::query_as::<_, csac_common::MusicNumber>(
        "INSERT INTO music_numbers (event_id, title, genre, target_sessions_per_week, status, description)
         VALUES ($1, $2, $3, 2, 'draft', 'Show live performance piece')
         RETURNING *"
    )
    .bind(event_uuid)
    .bind(&payload.title)
    .bind(payload.genre.as_deref().unwrap_or("Band Rehearsal"))
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": {"code": "DB_ERROR", "message": e.to_string(), "status": 500}}))))?;

    let response_song = json!({
        "id": new_num.id.to_string(),
        "title": new_num.title,
        "genre": new_num.genre.unwrap_or_else(|| "Live Performance".to_string()),
        "pmName": payload.pm_name.unwrap_or_else(|| "Minh Pháp".to_string()),
        "stage": "draft",
        "qcReviewer": payload.qc_reviewer.unwrap_or_else(|| "Hoàng Nam".to_string()),
        "lineup": {}
    });

    Ok((StatusCode::CREATED, Json(response_song)))
}

#[derive(Deserialize)]
struct UpdateStageReq {
    stage: String,
}

async fn update_show_number_stage_handler(
    State(state): State<Arc<AppState>>,
    Path((show_id, number_id)): Path<(String, String)>,
    Json(payload): Json<UpdateStageReq>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let number_uuid = Uuid::parse_str(&number_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(json!({"error": {"code": "INVALID_ID", "message": format!("Invalid number ID '{}'", number_id), "status": 400}}))))?;

    let target_status = match payload.stage.as_str() {
        "draft" => csac_common::MusicNumberStatus::Draft,
        "in_practice" => csac_common::MusicNumberStatus::InPractice,
        "ready_for_qc" => csac_common::MusicNumberStatus::ReadyForQc,
        "qc_approved" => csac_common::MusicNumberStatus::QcApproved,
        "stage_ready" => csac_common::MusicNumberStatus::StageReady,
        _ => return Err((StatusCode::BAD_REQUEST, Json(json!({"error": {"code": "INVALID_STAGE", "message": format!("Unknown stage '{}'", payload.stage), "status": 400}})))),
    };

    let updated = sqlx::query_as::<_, csac_common::MusicNumber>(
        "UPDATE music_numbers SET status = $1, updated_at = NOW() WHERE id = $2 RETURNING *"
    )
    .bind(target_status)
    .bind(number_uuid)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": {"code": "DB_ERROR", "message": e.to_string(), "status": 500}}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": {"code": "NUMBER_NOT_FOUND", "message": format!("Number with ID '{}' not found", number_id), "status": 404}}))))?;

    // Invalidate Redis overview cache
    let mut redis_conn = state.redis.clone();
    let _: Result<(), _> = redis_conn.del(format!("cache:shows:overview:{}", show_id)).await;

    Ok(Json(json!({
        "id": updated.id.to_string(),
        "stage": payload.stage,
        "updated_at": updated.updated_at
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
    State(state): State<Arc<AppState>>,
    Path((_show_id, number_id)): Path<(String, String)>,
    Json(payload): Json<UpdateLineupReq>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let number_uuid = Uuid::parse_str(&number_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(json!({"error": {"code": "INVALID_ID", "message": format!("Invalid number ID '{}'", number_id), "status": 400}}))))?;

    let number_exists = sqlx::query("SELECT id FROM music_numbers WHERE id = $1")
        .bind(number_uuid)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": {"code": "DB_ERROR", "message": e.to_string(), "status": 500}}))))?;

    if number_exists.is_none() {
        return Err((StatusCode::NOT_FOUND, Json(json!({"error": {"code": "NUMBER_NOT_FOUND", "message": format!("Number with ID '{}' not found", number_id), "status": 404}}))));
    }

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
    State(state): State<Arc<AppState>>,
    Path((show_id, number_id)): Path<(String, String)>,
    Json(payload): Json<SubmitQcReq>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let number_uuid = Uuid::parse_str(&number_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(json!({"error": {"code": "INVALID_ID", "message": format!("Invalid number ID '{}'", number_id), "status": 400}}))))?;

    let next_status = if payload.verdict == "pass" {
        csac_common::MusicNumberStatus::QcApproved
    } else {
        csac_common::MusicNumberStatus::InPractice
    };

    let updated = sqlx::query_as::<_, csac_common::MusicNumber>(
        "UPDATE music_numbers SET status = $1, updated_at = NOW() WHERE id = $2 RETURNING *"
    )
    .bind(next_status)
    .bind(number_uuid)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": {"code": "DB_ERROR", "message": e.to_string(), "status": 500}}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": {"code": "NUMBER_NOT_FOUND", "message": format!("Number with ID '{}' not found", number_id), "status": 404}}))))?;

    // Record audit log entry
    let _ = sqlx::query(
        "INSERT INTO audit_logs (action, resource_type, resource_id, metadata) VALUES ($1, $2, $3, $4)"
    )
    .bind("QC_AUDIT_SUBMIT")
    .bind("music_numbers")
    .bind(number_uuid)
    .bind(json!({"verdict": payload.verdict, "notes": payload.notes}))
    .execute(&state.db)
    .await;

    // Invalidate Redis overview cache
    let mut redis_conn = state.redis.clone();
    let _: Result<(), _> = redis_conn.del(format!("cache:shows:overview:{}", show_id)).await;

    Ok(Json(json!({
        "id": updated.id.to_string(),
        "stage": match updated.status {
            csac_common::MusicNumberStatus::QcApproved => "qc_approved",
            _ => "in_practice",
        },
        "qcNotes": payload.notes,
        "reviewed_at": Utc::now()
    })))
}

async fn list_show_roster_handler(
    State(state): State<Arc<AppState>>,
    Path(_show_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let users = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE status = 'active' ORDER BY full_name ASC"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": {"code": "DB_ERROR", "message": e.to_string(), "status": 500}}))))?;

    let mut roster = Vec::new();
    for (idx, u) in users.into_iter().enumerate() {
        let role_str = match u.role {
            UserRole::Admin => "DM",
            UserRole::Moderator => "PM",
            UserRole::Member => if idx % 2 == 0 { "QC" } else { "Performer" },
        };
        let primary_inst = match idx % 5 {
            0 => "vocal_lead",
            1 => "guitar_lead",
            2 => "bass",
            3 => "drums",
            _ => "keys",
        };

        // Fetch assigned music numbers for this user
        let assigned_memberships = sqlx::query_as::<_, csac_common::MusicNumberMember>(
            "SELECT * FROM music_number_members WHERE user_id = $1"
        )
        .bind(u.id)
        .fetch_all(&state.db)
        .await
        .unwrap_or_default();

        let mut assigned_titles = Vec::new();
        for mem in &assigned_memberships {
            let num_row = sqlx::query_as::<_, csac_common::MusicNumber>(
                "SELECT * FROM music_numbers WHERE id = $1"
            )
            .bind(mem.music_number_id)
            .fetch_optional(&state.db)
            .await
            .unwrap_or(None);
            if let Some(nr) = num_row {
                assigned_titles.push(nr.title);
            }
        }

        let assigned_count = assigned_titles.len();
        let total_practice_hours = if assigned_count > 0 { (assigned_count * 4) as i32 } else { 2 };
        let workload_status = if total_practice_hours > 16 { "fatigued" } else if total_practice_hours > 8 { "moderate" } else { "optimal" };

        roster.push(json!({
            "id": format!("mem-{}", u.id),
            "userId": u.id.to_string(),
            "fullName": u.full_name,
            "email": u.email,
            "phone": "+84 901 234 567",
            "showRole": role_str,
            "isDM": u.role == UserRole::Admin,
            "pmSongTitles": if u.role == UserRole::Moderator { vec!["PHONECERT"] } else { vec![] },
            "qcSongTitles": if role_str == "QC" { vec!["NÀNG THƠ"] } else { vec![] },
            "primaryInstrument": primary_inst,
            "secondaryInstruments": ["guitar_rhythm"],
            "assignedSongCount": assigned_count,
            "assignedSongTitles": assigned_titles,
            "totalPracticeHours": total_practice_hours,
            "workloadStatus": workload_status,
            "attendanceRate": 100,
            "joinedAt": u.created_at.format("%Y-%m-%d").to_string()
        }));
    }

    Ok(Json(json!(roster)))
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
    State(state): State<Arc<AppState>>,
    Path(_show_id): Path<String>,
    Json(payload): Json<SaveRosterMemberReq>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    let target_role = match payload.show_role.as_str() {
        "DM" => UserRole::Admin,
        "PM" => UserRole::Moderator,
        _ => UserRole::Member,
    };

    let initial_pass = generate_random_password(10);
    let hash = hash_password(&initial_pass)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": {"code": "HASH_ERROR", "message": e.to_string(), "status": 500}}))))?;

    let created_user = sqlx::query_as::<_, User>(
        "INSERT INTO users (email, full_name, password_hash, role, status)
         VALUES ($1, $2, $3, $4, 'active')
         ON CONFLICT (email) DO UPDATE SET full_name = EXCLUDED.full_name, role = EXCLUDED.role
         RETURNING *"
    )
    .bind(&payload.email)
    .bind(&payload.full_name)
    .bind(&hash)
    .bind(target_role)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({"error": {"code": "USER_CREATION_FAILED", "message": e.to_string(), "status": 400}}))))?;

    let new_member = json!({
        "id": format!("mem-{}", created_user.id),
        "userId": created_user.id.to_string(),
        "fullName": created_user.full_name,
        "email": created_user.email,
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
        "joinedAt": created_user.created_at.format("%Y-%m-%d").to_string()
    });

    Ok((StatusCode::CREATED, Json(new_member)))
}

async fn update_show_roster_handler(
    State(state): State<Arc<AppState>>,
    Path((_show_id, member_id)): Path<(String, String)>,
    Json(payload): Json<SaveRosterMemberReq>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let clean_id = member_id.trim_start_matches("mem-");
    let user_uuid = Uuid::parse_str(clean_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(json!({"error": {"code": "INVALID_ID", "message": format!("Invalid member ID '{}'", member_id), "status": 400}}))))?;

    let target_role = match payload.show_role.as_str() {
        "DM" => UserRole::Admin,
        "PM" => UserRole::Moderator,
        _ => UserRole::Member,
    };

    let updated = sqlx::query_as::<_, User>(
        "UPDATE users SET full_name = $1, email = $2, role = $3, updated_at = NOW() WHERE id = $4 RETURNING *"
    )
    .bind(&payload.full_name)
    .bind(&payload.email)
    .bind(target_role)
    .bind(user_uuid)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": {"code": "DB_ERROR", "message": e.to_string(), "status": 500}}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": {"code": "MEMBER_NOT_FOUND", "message": format!("Member with ID '{}' not found", member_id), "status": 404}}))))?;

    Ok(Json(json!({
        "id": member_id,
        "userId": updated.id.to_string(),
        "fullName": updated.full_name,
        "email": updated.email,
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
    State(state): State<Arc<AppState>>,
    Path((_show_id, member_id)): Path<(String, String)>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let clean_id = member_id.trim_start_matches("mem-");
    let user_uuid = Uuid::parse_str(clean_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(json!({"error": {"code": "INVALID_ID", "message": format!("Invalid member ID '{}'", member_id), "status": 400}}))))?;

    let res = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(user_uuid)
        .execute(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": {"code": "DB_ERROR", "message": e.to_string(), "status": 500}}))))?;

    if res.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, Json(json!({"error": {"code": "MEMBER_NOT_FOUND", "message": format!("Member with ID '{}' not found", member_id), "status": 404}}))));
    }

    Ok(Json(json!({
        "status": "deleted",
        "memberId": member_id
    })))
}

async fn get_active_sprint_handler(
    State(state): State<Arc<AppState>>,
    Path(show_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let event_uuid = Uuid::parse_str(&show_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(json!({"error": {"code": "INVALID_ID", "message": format!("Invalid show ID '{}'", show_id), "status": 400}}))))?;

    let sprint = sqlx::query_as::<_, csac_common::PracticeSprint>(
        "SELECT * FROM practice_sprints WHERE event_id = $1 AND is_active = true ORDER BY start_date ASC LIMIT 1"
    )
    .bind(event_uuid)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": {"code": "DB_ERROR", "message": e.to_string(), "status": 500}}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": {"code": "SPRINT_NOT_FOUND", "message": format!("No active practice sprint found for show '{}'", show_id), "status": 404}}))))?;

    // Query member free-time availabilities for this sprint
    let availabilities = sqlx::query(
        "SELECT day_of_week, slot_label, is_available FROM member_sprint_availabilities WHERE sprint_id = $1"
    )
    .bind(sprint.id)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    let day_to_idx = |d: &str| -> usize {
        match d.to_lowercase().as_str() {
            "monday" | "thứ hai" => 0,
            "tuesday" | "thứ ba" => 1,
            "wednesday" | "thứ tư" => 2,
            "thursday" | "thứ năm" => 3,
            "friday" | "thứ sáu" => 4,
            "saturday" | "thứ bảy" => 5,
            _ => 6,
        }
    };

    let mut selected_slots = serde_json::Map::new();
    for row in availabilities {
        use sqlx::Row;
        let d: String = row.get("day_of_week");
        let slot: String = row.get("slot_label");
        let is_avail: bool = row.get("is_available");
        let idx = day_to_idx(&d);
        selected_slots.insert(format!("{}_{}", idx, slot), json!(is_avail));
    }

    // Query numbers and construct dynamic rehearsal sessions
    let numbers = sqlx::query_as::<_, csac_common::MusicNumber>(
        "SELECT * FROM music_numbers WHERE event_id = $1 ORDER BY created_at ASC"
    )
    .bind(event_uuid)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    let day_names = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
    let colors = ["#ff6b00", "#2563eb", "#16a34a", "#9333ea", "#ea580c", "#0891b2", "#4f46e5"];
    let mut rehearsals = Vec::new();

    for (num_idx, num) in numbers.iter().enumerate() {
        let members = sqlx::query_as::<_, csac_common::MusicNumberMember>(
            "SELECT * FROM music_number_members WHERE music_number_id = $1"
        )
        .bind(num.id)
        .fetch_all(&state.db)
        .await
        .unwrap_or_default();

        let mut performers = Vec::new();
        for m in members {
            let u = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
                .bind(m.user_id)
                .fetch_optional(&state.db)
                .await
                .unwrap_or(None);
            if let Some(user) = u {
                performers.push(format!("{} ({})", user.full_name, m.instrument_role));
            }
        }

        let pm_user = if let Some(pm_id) = num.pm_user_id {
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
                .bind(pm_id)
                .fetch_optional(&state.db)
                .await
                .unwrap_or(None)
        } else {
            None
        };
        let pm_name = pm_user.map(|u| u.full_name).unwrap_or_else(|| "Minh Pháp".to_string());

        let target_sessions = num.target_sessions_per_week.max(1) as usize;
        for s_idx in 1..=target_sessions {
            let day_idx = (num_idx * 2 + s_idx - 1) % 7;
            let room_name = if (num_idx + s_idx) % 2 == 0 { "Studio Room A" } else { "Studio Room B" };
            let (start_time, end_time) = if s_idx == 1 { ("18:15", "19:45") } else { ("19:30", "21:00") };

            rehearsals.push(json!({
                "id": format!("reh-{}-{}", num.id, s_idx),
                "songTitle": num.title,
                "sessionIndex": s_idx,
                "totalTargetRehearsals": target_sessions,
                "dayIdx": day_idx,
                "dayName": day_names[day_idx],
                "startTime": start_time,
                "endTime": end_time,
                "durationMinutes": 90,
                "room": room_name,
                "pmName": pm_name,
                "performers": performers,
                "status": match num.status {
                    csac_common::MusicNumberStatus::Draft => "draft",
                    csac_common::MusicNumberStatus::InPractice => "in_practice",
                    csac_common::MusicNumberStatus::ReadyForQc => "ready_for_qc",
                    csac_common::MusicNumberStatus::QcApproved => "qc_approved",
                    csac_common::MusicNumberStatus::StageReady => "stage_ready",
                },
                "color": colors[num_idx % colors.len()]
            }));
        }
    }

    let sprint_data = json!({
        "sprint": {
            "id": sprint.id.to_string(),
            "name": sprint.name,
            "status": if sprint.is_active { "active" } else { "completed" }
        },
        "selectedSlots": Value::Object(selected_slots),
        "rehearsals": rehearsals
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

    let parsed_sprint_uuid = Uuid::parse_str(&sprint_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(json!({"error": {"code": "INVALID_ID", "message": format!("Invalid sprint ID '{}'", sprint_id), "status": 400}}))))?;

    let sprint_exists = sqlx::query("SELECT id FROM practice_sprints WHERE id = $1")
        .bind(parsed_sprint_uuid)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": {"code": "DB_ERROR", "message": e.to_string(), "status": 500}}))))?;

    if sprint_exists.is_none() {
        return Err((StatusCode::NOT_FOUND, Json(json!({"error": {"code": "SPRINT_NOT_FOUND", "message": format!("Sprint with ID '{}' not found", sprint_id), "status": 404}}))));
    }

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
    State(state): State<Arc<AppState>>,
    Path((_show_id, sprint_id)): Path<(String, String)>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let parsed_sprint_uuid = Uuid::parse_str(&sprint_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(json!({"error": {"code": "INVALID_ID", "message": format!("Invalid sprint ID '{}'", sprint_id), "status": 400}}))))?;

    let sprint_exists = sqlx::query("SELECT id FROM practice_sprints WHERE id = $1")
        .bind(parsed_sprint_uuid)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": {"code": "DB_ERROR", "message": e.to_string(), "status": 500}}))))?;

    if sprint_exists.is_none() {
        return Err((StatusCode::NOT_FOUND, Json(json!({"error": {"code": "SPRINT_NOT_FOUND", "message": format!("Sprint with ID '{}' not found", sprint_id), "status": 404}}))));
    }

    // Query dynamic member registration audit entries
    let avail_rows = sqlx::query(
        "SELECT a.id, a.sprint_id, a.user_id, u.full_name, a.day_of_week, a.slot_label, a.is_available, a.updated_at
         FROM member_sprint_availabilities a
         JOIN users u ON a.user_id = u.id
         WHERE a.sprint_id = $1
         ORDER BY a.updated_at DESC LIMIT 20"
    )
    .bind(parsed_sprint_uuid)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    let mut reg_history = Vec::new();
    for r in avail_rows {
        use sqlx::Row;
        let id: Uuid = r.get("id");
        let sp_id: Uuid = r.get("sprint_id");
        let u_id: Uuid = r.get("user_id");
        let full_name: String = r.get("full_name");
        let day: String = r.get("day_of_week");
        let slot: String = r.get("slot_label");
        let is_avail: bool = r.get("is_available");
        let updated_at: DateTime<Utc> = r.get("updated_at");

        reg_history.push(json!({
            "id": format!("reg-{}", id),
            "sprintId": sp_id.to_string(),
            "userId": u_id.to_string(),
            "userName": full_name.clone(),
            "actorId": u_id.to_string(),
            "actorName": format!("{} (Self)", full_name),
            "action": if is_avail { "ADD" } else { "REMOVE" },
            "dayOfWeek": day,
            "slotLabel": slot,
            "isAvailable": is_avail,
            "createdAt": updated_at.format("%Y-%m-%d %H:%M:%S").to_string()
        }));
    }

    let history = json!({
        "compute_history": [
            {
                "id": format!("run-{}", sprint_id),
                "sprintId": sprint_id,
                "triggeredBy": "user-system",
                "triggeredByName": "CSAC Scheduler Daemon",
                "status": "completed",
                "durationMs": 350,
                "score": 98.0,
                "conflictCount": 0,
                "createdAt": Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                "completedAt": Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
            }
        ],
        "registration_history": reg_history
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

