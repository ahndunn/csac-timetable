use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use csac_common::PracticeSprint;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use uuid::Uuid;

use crate::middleware::extract_claims;
use crate::state::AppState;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SprintAvailabilitySlot {
    pub day_of_week: String,
    pub slot_label: String,
    pub is_available: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SprintAvailabilityRequest {
    pub slots: Vec<SprintAvailabilitySlot>,
}

pub async fn list_sprints_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let sprints = sqlx::query_as::<_, PracticeSprint>(
        "SELECT * FROM practice_sprints ORDER BY start_date ASC"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!(sprints)))
}

pub async fn submit_sprint_availability_handler(
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
