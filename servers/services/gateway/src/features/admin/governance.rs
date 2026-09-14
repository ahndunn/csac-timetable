use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use chrono::Utc;
use csac_common::{
    generate_otp_code, AdminDowngradeProposal, ProposalStatus, User,
    VoteDecision,
};
use csac_kafka_events::{AdminOtpGeneratedEvent, TOPIC_ADMIN_OTP_GENERATED};
use rdkafka::producer::FutureRecord;
use redis::AsyncCommands;
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

use crate::middleware::extract_claims;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct VoteProposalRequest {
    pub decision: VoteDecision,
    pub otp: String,
}

pub async fn list_proposals_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Value>>, (StatusCode, Json<Value>)> {
    let proposals = sqlx::query_as::<_, AdminDowngradeProposal>(
        "SELECT * FROM admin_downgrade_proposals WHERE status = 'pending' ORDER BY created_at DESC"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    let mut result = Vec::new();
    for p in proposals {
        let target_user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(p.target_admin_id)
            .fetch_optional(&state.db)
            .await
            .unwrap_or(None);

        let initiator = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(p.initiated_by)
            .fetch_optional(&state.db)
            .await
            .unwrap_or(None);

        result.push(json!({
            "proposal": p,
            "target_user": target_user,
            "initiator": initiator,
        }));
    }

    Ok(Json(result))
}

pub async fn request_proposal_otp_handler(
    State(state): State<Arc<AppState>>,
    Path(proposal_id): Path<Uuid>,
    headers: HeaderMap,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let claims = extract_claims(&headers, &state.jwt_secret).await
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(json!({"error": "Unauthorized"}))))?;

    let proposal = sqlx::query_as::<_, AdminDowngradeProposal>(
        "SELECT * FROM admin_downgrade_proposals WHERE id = $1 AND status = 'pending'"
    )
    .bind(proposal_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Proposal not found or inactive"}))))?;

    let target_user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(proposal.target_admin_id)
        .fetch_one(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    let otp_code = generate_otp_code();
    let redis_key = format!("otp:downgrade:{}:{}", proposal_id, claims.sub);

    let mut redis_conn = state.redis.clone();
    let _: () = redis_conn
        .set_ex(&redis_key, &otp_code, 600)
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
            ttl_seconds: 600,
            generated_at: Utc::now(),
        };

        if let Ok(event_json) = serde_json::to_string(&event) {
            let key_str = claims.sub.to_string();
            let record = FutureRecord::to(TOPIC_ADMIN_OTP_GENERATED)
                .payload(&event_json)
                .key(&key_str);
            let _ = producer.send(record, Duration::from_secs(3)).await;
        }
    }

    Ok(Json(json!({
        "message": "OTP generated and dispatched to administrator email",
        "ttl_seconds": 600,
        "otp_debug": otp_code
    })))
}

pub async fn vote_proposal_handler(
    State(state): State<Arc<AppState>>,
    Path(proposal_id): Path<Uuid>,
    headers: HeaderMap,
    Json(payload): Json<VoteProposalRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let claims = extract_claims(&headers, &state.jwt_secret).await
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(json!({"error": "Unauthorized"}))))?;

    let redis_key = format!("otp:downgrade:{}:{}", proposal_id, claims.sub);
    let mut redis_conn = state.redis.clone();

    let stored_otp: Option<String> = redis_conn
        .get(&redis_key)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    if stored_otp.is_none() || stored_otp.unwrap() != payload.otp {
        return Err((StatusCode::BAD_REQUEST, Json(json!({"error": "Invalid or expired OTP"}))));
    }

    let _: () = redis_conn.del(&redis_key).await.unwrap_or(());

    let proposal = sqlx::query_as::<_, AdminDowngradeProposal>(
        "SELECT * FROM admin_downgrade_proposals WHERE id = $1 AND status = 'pending'"
    )
    .bind(proposal_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Proposal not found or inactive"}))))?;

    let vote_decision_str = match payload.decision {
        VoteDecision::Approve => "approve",
        VoteDecision::Reject => "reject",
    };

    sqlx::query(
        "INSERT INTO admin_downgrade_votes (proposal_id, admin_id, decision)
         VALUES ($1, $2, $3::vote_decision)
         ON CONFLICT (proposal_id, admin_id) DO UPDATE SET decision = EXCLUDED.decision"
    )
    .bind(proposal_id)
    .bind(claims.sub)
    .bind(vote_decision_str)
    .execute(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    let approve_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM admin_downgrade_votes WHERE proposal_id = $1 AND decision = 'approve'"
    )
    .bind(proposal_id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    let mut new_status = proposal.status;

    if approve_count.0 >= proposal.required_approvals as i64 {
        new_status = ProposalStatus::Approved;
        sqlx::query("UPDATE users SET role = $1 WHERE id = $2")
            .bind(proposal.target_role)
            .bind(proposal.target_admin_id)
            .execute(&state.db)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;
    }

    sqlx::query("UPDATE admin_downgrade_proposals SET current_approvals = $1, status = $2 WHERE id = $3")
        .bind(approve_count.0 as i32)
        .bind(new_status)
        .bind(proposal_id)
        .execute(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({
        "status": "VOTE_RECORDED",
        "current_approvals": approve_count.0,
        "required_approvals": proposal.required_approvals,
        "proposal_status": new_status,
    })))
}
