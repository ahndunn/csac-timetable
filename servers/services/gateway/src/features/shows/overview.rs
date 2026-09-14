use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use csac_common::{Event, MusicNumber, MusicNumberStatus, PracticeSprint, User};
use redis::AsyncCommands;
use serde_json::{json, Value};
use std::sync::Arc;
use uuid::Uuid;

use crate::state::AppState;

pub async fn get_show_overview_handler(
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

    let numbers = sqlx::query_as::<_, MusicNumber>(
        "SELECT * FROM music_numbers WHERE event_id = $1 ORDER BY created_at ASC"
    )
    .bind(event_uuid)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    let total_numbers = numbers.len();
    let qc_approved_count = numbers
        .iter()
        .filter(|n| n.status == MusicNumberStatus::QcApproved || n.status == MusicNumberStatus::StageReady)
        .count();

    let readiness_percent = if total_numbers > 0 {
        ((qc_approved_count as f64 / total_numbers as f64) * 100.0).round() as i32
    } else {
        75
    };

    let mut highlights = Vec::new();
    for num in &numbers {
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
        let genre_str = num.genre.clone().unwrap_or_else(|| "Band Rehearsal".to_string());
        let stage_str = match num.status {
            MusicNumberStatus::Draft => "draft",
            MusicNumberStatus::InPractice => "in_practice",
            MusicNumberStatus::ReadyForQc => "ready_for_qc",
            MusicNumberStatus::QcApproved => "qc_approved",
            MusicNumberStatus::StageReady => "stage_ready",
        };
        let badge_str = match num.status {
            MusicNumberStatus::StageReady => "Stage Ready",
            MusicNumberStatus::QcApproved => "QC Approved",
            MusicNumberStatus::ReadyForQc => "Ready for QC",
            _ => "In Practice",
        };

        highlights.push(json!({
            "title": format!("\"{}\" ({})", num.title, genre_str),
            "meta": format!("Leader (PM): {} • Sessions/wk: {}", pm_name, num.target_sessions_per_week),
            "stage": stage_str,
            "badge": badge_str,
        }));
    }

    let sprints = sqlx::query_as::<_, PracticeSprint>(
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

    if let Ok(ser) = serde_json::to_string(&overview) {
        let _: Result<(), _> = redis_conn.set_ex(&cache_key, ser, 60).await;
    }

    Ok(Json(overview))
}
