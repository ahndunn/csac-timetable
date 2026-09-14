use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use csac_common::{MusicNumber, MusicNumberMember, MusicNumberStatus, PracticeSprint, User};
use serde_json::{json, Value};
use std::sync::Arc;
use uuid::Uuid;

use crate::middleware::extract_claims;
use crate::state::AppState;
use crate::features::sprints::availability::SprintAvailabilityRequest;

pub async fn get_active_sprint_handler(
    State(state): State<Arc<AppState>>,
    Path(show_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let event_uuid = Uuid::parse_str(&show_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(json!({"error": {"code": "INVALID_ID", "message": format!("Invalid show ID '{}'", show_id), "status": 400}}))))?;

    let sprint = sqlx::query_as::<_, PracticeSprint>(
        "SELECT * FROM practice_sprints WHERE event_id = $1 AND is_active = true ORDER BY start_date ASC LIMIT 1"
    )
    .bind(event_uuid)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": {"code": "DB_ERROR", "message": e.to_string(), "status": 500}}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": {"code": "SPRINT_NOT_FOUND", "message": format!("No active practice sprint found for show '{}'", show_id), "status": 404}}))))?;

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

    let numbers = sqlx::query_as::<_, MusicNumber>(
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
        let members = sqlx::query_as::<_, MusicNumberMember>(
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
                    MusicNumberStatus::Draft => "draft",
                    MusicNumberStatus::InPractice => "in_practice",
                    MusicNumberStatus::ReadyForQc => "ready_for_qc",
                    MusicNumberStatus::QcApproved => "qc_approved",
                    MusicNumberStatus::StageReady => "stage_ready",
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

pub async fn submit_show_sprint_availability_handler(
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
