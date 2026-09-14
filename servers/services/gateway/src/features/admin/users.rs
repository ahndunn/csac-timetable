use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use chrono::Utc;
use csac_common::{
    calculate_required_approvals, generate_random_password, hash_password, AdminDowngradeProposal,
    User, UserRole,
};
use csac_kafka_events::{UserCreatedEvent, TOPIC_USER_CREATED};
use rdkafka::producer::FutureRecord;
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

use crate::middleware::extract_claims;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub full_name: String,
    pub role: UserRole,
}

#[derive(Deserialize)]
pub struct UpdateUserRoleRequest {
    pub role: UserRole,
}

#[derive(Deserialize)]
pub struct CreateProposalRequest {
    pub target_role: UserRole,
    pub reason: String,
}

pub async fn list_users_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<User>>, (StatusCode, Json<Value>)> {
    let users = sqlx::query_as::<_, User>(
        "SELECT id, email, full_name, password_hash, role, status, created_at, updated_at FROM users ORDER BY created_at ASC"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(users))
}

pub async fn create_user_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    let initial_pass = generate_random_password(12);
    let hash = hash_password(&initial_pass)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (email, full_name, password_hash, role, status) VALUES ($1, $2, $3, $4, 'active') RETURNING *"
    )
    .bind(&payload.email)
    .bind(&payload.full_name)
    .bind(&hash)
    .bind(payload.role)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({"error": e.to_string()}))))?;

    if let Some(producer) = &state.kafka_producer {
        let event = UserCreatedEvent {
            user_id: user.id,
            email: user.email.clone(),
            full_name: user.full_name.clone(),
            initial_password: initial_pass.clone(),
            role: user.role.as_str().to_string(),
            created_at: Utc::now(),
        };

        if let Ok(event_json) = serde_json::to_string(&event) {
            let key_str = user.id.to_string();
            let record = FutureRecord::to(TOPIC_USER_CREATED)
                .payload(&event_json)
                .key(&key_str);
            let _ = producer.send(record, Duration::from_secs(3)).await;
        }
    }

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "user": user,
            "initial_password": initial_pass
        })),
    ))
}

pub async fn update_user_role_handler(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<UpdateUserRoleRequest>,
) -> Result<Json<User>, (StatusCode, Json<Value>)> {
    let user = sqlx::query_as::<_, User>(
        "UPDATE users SET role = $1, updated_at = NOW() WHERE id = $2 RETURNING *"
    )
    .bind(payload.role)
    .bind(user_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "User not found"}))))?;

    Ok(Json(user))
}

pub async fn create_downgrade_proposal_handler(
    State(state): State<Arc<AppState>>,
    Path(target_admin_id): Path<Uuid>,
    headers: HeaderMap,
    Json(payload): Json<CreateProposalRequest>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    let claims = extract_claims(&headers, &state.jwt_secret).await
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(json!({"error": "Unauthorized"}))))?;

    let total_admins: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM users WHERE role = 'admin' AND status = 'active'"
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    let req_approvals = calculate_required_approvals(total_admins.0 as usize)
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({"error": e.to_string()}))))?;

    let proposal = sqlx::query_as::<_, AdminDowngradeProposal>(
        "INSERT INTO admin_downgrade_proposals (target_admin_id, target_role, initiated_by, reason, total_admins_at_proposal, required_approvals)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING *"
    )
    .bind(target_admin_id)
    .bind(payload.target_role)
    .bind(claims.sub)
    .bind(payload.reason)
    .bind(total_admins.0 as i32)
    .bind(req_approvals as i32)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok((StatusCode::CREATED, Json(json!(proposal))))
}
