use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use csac_common::{MusicNumber, MusicNumberMember};
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;
use uuid::Uuid;

use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateMusicNumberRequest {
    pub event_id: Option<Uuid>,
    pub title: String,
    pub genre: Option<String>,
    pub pm_user_id: Option<Uuid>,
    pub target_sessions_per_week: Option<i32>,
    pub description: Option<String>,
}

pub async fn list_music_numbers_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let numbers = sqlx::query_as::<_, MusicNumber>(
        "SELECT * FROM music_numbers ORDER BY created_at DESC"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    let mut result = Vec::new();
    for num in numbers {
        let members = sqlx::query_as::<_, MusicNumberMember>(
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

pub async fn create_music_number_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateMusicNumberRequest>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    let num = sqlx::query_as::<_, MusicNumber>(
        "INSERT INTO music_numbers (event_id, title, genre, pm_user_id, target_sessions_per_week, description)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING *"
    )
    .bind(payload.event_id)
    .bind(&payload.title)
    .bind(&payload.genre)
    .bind(payload.pm_user_id)
    .bind(payload.target_sessions_per_week.unwrap_or(2))
    .bind(&payload.description)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({"error": e.to_string()}))))?;

    Ok((StatusCode::CREATED, Json(json!(num))))
}
