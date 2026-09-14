use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use csac_common::{PracticeTask, TaskStatus, TaskType};
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;
use uuid::Uuid;

use crate::middleware::extract_claims;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreatePracticeTaskRequest {
    pub music_number_id: Uuid,
    pub task_type: TaskType,
    pub title: String,
    pub description: Option<String>,
    pub assigned_to: Option<Uuid>,
    pub qc_reviewer_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct ReviewTaskRequest {
    pub status: TaskStatus,
    pub qc_feedback: Option<String>,
}

pub async fn list_sprint_tasks_handler(
    State(state): State<Arc<AppState>>,
    Path(sprint_id): Path<Uuid>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let tasks = sqlx::query_as::<_, PracticeTask>(
        "SELECT * FROM practice_tasks WHERE sprint_id = $1 ORDER BY created_at ASC"
    )
    .bind(sprint_id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!(tasks)))
}

pub async fn create_sprint_task_handler(
    State(state): State<Arc<AppState>>,
    Path(sprint_id): Path<Uuid>,
    headers: HeaderMap,
    Json(payload): Json<CreatePracticeTaskRequest>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    let _claims = extract_claims(&headers, &state.jwt_secret).await
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(json!({"error": "Unauthorized"}))))?;

    let task = sqlx::query_as::<_, PracticeTask>(
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

pub async fn review_task_handler(
    State(state): State<Arc<AppState>>,
    Path((_sprint_id, task_id)): Path<(Uuid, Uuid)>,
    headers: HeaderMap,
    Json(payload): Json<ReviewTaskRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let _claims = extract_claims(&headers, &state.jwt_secret).await
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(json!({"error": "Unauthorized"}))))?;

    let task = sqlx::query_as::<_, PracticeTask>(
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
