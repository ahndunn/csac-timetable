use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use csac_common::{MusicNumber, MusicNumberMember, MusicNumberStatus, PracticeTask, User};
use redis::AsyncCommands;
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;
use uuid::Uuid;

use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateShowNumberReq {
    pub title: String,
    pub genre: Option<String>,
    #[serde(rename = "pm_name")]
    pub pm_name: Option<String>,
    #[serde(rename = "qc_reviewer")]
    pub qc_reviewer: Option<String>,
}

pub async fn list_show_numbers_handler(
    State(state): State<Arc<AppState>>,
    Path(show_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let event_uuid = Uuid::parse_str(&show_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(json!({"error": {"code": "INVALID_ID", "message": format!("Invalid show ID '{}'", show_id), "status": 400}}))))?;

    let numbers = sqlx::query_as::<_, MusicNumber>(
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

        let members = sqlx::query_as::<_, MusicNumberMember>(
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
            if let Some(u) = u_row {
                let key = match m.instrument_role.to_lowercase().as_str() {
                    r if r.contains("vocal") => "vocalLead",
                    r if r.contains("guitar") => "guitarLead",
                    r if r.contains("bass") => "bass",
                    r if r.contains("drum") || r.contains("cajon") => "drums",
                    _ => "keys",
                };
                lineup.insert(key.to_string(), json!(u.full_name));
            }
        }

        let qc_task = sqlx::query_as::<_, PracticeTask>(
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
                MusicNumberStatus::Draft => "draft",
                MusicNumberStatus::InPractice => "in_practice",
                MusicNumberStatus::ReadyForQc => "ready_for_qc",
                MusicNumberStatus::QcApproved => "qc_approved",
                MusicNumberStatus::StageReady => "stage_ready",
            },
            "qcReviewer": qc_reviewer_name,
            "qcNotes": qc_task.and_then(|t| t.qc_feedback),
            "lineup": Value::Object(lineup)
        }));
    }

    Ok(Json(json!(result)))
}

pub async fn create_show_number_handler(
    State(state): State<Arc<AppState>>,
    Path(show_id): Path<String>,
    Json(payload): Json<CreateShowNumberReq>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    let event_uuid = Uuid::parse_str(&show_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(json!({"error": {"code": "INVALID_ID", "message": format!("Invalid show ID '{}'", show_id), "status": 400}}))))?;

    let mut redis_conn = state.redis.clone();
    let _: Result<(), _> = redis_conn.del(format!("cache:shows:overview:{}", show_id)).await;

    let new_num = sqlx::query_as::<_, MusicNumber>(
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
