use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use csac_common::{MusicNumber, MusicNumberStatus};
use redis::AsyncCommands;
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;
use uuid::Uuid;

use crate::state::AppState;

#[derive(Deserialize)]
pub struct UpdateStageReq {
    pub stage: String,
}

#[derive(Deserialize)]
pub struct UpdateLineupReq {
    #[serde(rename = "vocalLead")]
    pub vocal_lead: Option<String>,
    #[serde(rename = "guitarLead")]
    pub guitar_lead: Option<String>,
    pub bass: Option<String>,
    pub drums: Option<String>,
    pub keys: Option<String>,
}

#[derive(Deserialize)]
pub struct SubmitQcReq {
    pub verdict: String,
    pub notes: Option<String>,
}

pub async fn update_show_number_stage_handler(
    State(state): State<Arc<AppState>>,
    Path((show_id, number_id)): Path<(String, String)>,
    Json(payload): Json<UpdateStageReq>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let number_uuid = Uuid::parse_str(&number_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(json!({"error": {"code": "INVALID_ID", "message": format!("Invalid number ID '{}'", number_id), "status": 400}}))))?;

    let target_status = match payload.stage.as_str() {
        "draft" => MusicNumberStatus::Draft,
        "in_practice" => MusicNumberStatus::InPractice,
        "ready_for_qc" => MusicNumberStatus::ReadyForQc,
        "qc_approved" => MusicNumberStatus::QcApproved,
        "stage_ready" => MusicNumberStatus::StageReady,
        _ => return Err((StatusCode::BAD_REQUEST, Json(json!({"error": {"code": "INVALID_STAGE", "message": format!("Unknown stage '{}'", payload.stage), "status": 400}})))),
    };

    let updated = sqlx::query_as::<_, MusicNumber>(
        "UPDATE music_numbers SET status = $1, updated_at = NOW() WHERE id = $2 RETURNING *"
    )
    .bind(target_status)
    .bind(number_uuid)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": {"code": "DB_ERROR", "message": e.to_string(), "status": 500}}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": {"code": "NUMBER_NOT_FOUND", "message": format!("Number with ID '{}' not found", number_id), "status": 404}}))))?;

    let mut redis_conn = state.redis.clone();
    let _: Result<(), _> = redis_conn.del(format!("cache:shows:overview:{}", show_id)).await;

    Ok(Json(json!({
        "id": updated.id.to_string(),
        "stage": payload.stage,
        "updated_at": updated.updated_at
    })))
}

pub async fn update_show_number_lineup_handler(
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

pub async fn submit_show_number_qc_handler(
    State(state): State<Arc<AppState>>,
    Path((show_id, number_id)): Path<(String, String)>,
    Json(payload): Json<SubmitQcReq>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let number_uuid = Uuid::parse_str(&number_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(json!({"error": {"code": "INVALID_ID", "message": format!("Invalid number ID '{}'", number_id), "status": 400}}))))?;

    let next_status = if payload.verdict == "pass" {
        MusicNumberStatus::QcApproved
    } else {
        MusicNumberStatus::InPractice
    };

    let updated = sqlx::query_as::<_, MusicNumber>(
        "UPDATE music_numbers SET status = $1, updated_at = NOW() WHERE id = $2 RETURNING *"
    )
    .bind(next_status)
    .bind(number_uuid)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": {"code": "DB_ERROR", "message": e.to_string(), "status": 500}}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": {"code": "NUMBER_NOT_FOUND", "message": format!("Number with ID '{}' not found", number_id), "status": 404}}))))?;

    let _ = sqlx::query(
        "INSERT INTO audit_logs (action, resource_type, resource_id, metadata) VALUES ($1, $2, $3, $4)"
    )
    .bind("QC_AUDIT_SUBMIT")
    .bind("music_numbers")
    .bind(number_uuid)
    .bind(json!({"verdict": payload.verdict, "notes": payload.notes}))
    .execute(&state.db)
    .await;

    let mut redis_conn = state.redis.clone();
    let _: Result<(), _> = redis_conn.del(format!("cache:shows:overview:{}", show_id)).await;

    Ok(Json(json!({
        "id": updated.id.to_string(),
        "stage": match updated.status {
            MusicNumberStatus::QcApproved => "qc_approved",
            _ => "in_practice",
        },
        "qcNotes": payload.notes,
        "reviewed_at": Utc::now()
    })))
}
