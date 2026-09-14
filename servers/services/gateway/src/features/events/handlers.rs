use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use chrono::Utc;
use csac_common::{Event, EventStatus, EventTimeSlot};
use csac_kafka_events::{EventStatusChangedEvent, TOPIC_EVENT_STATUS_CHANGED};
use rdkafka::producer::FutureRecord;
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

use crate::middleware::extract_claims;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateEventSlotPayload {
    pub day_of_week: String,
    pub slot_label: String,
    pub sort_order: Option<i32>,
}

#[derive(Deserialize)]
pub struct CreateEventRequest {
    pub title: String,
    pub description: Option<String>,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
    pub time_slots: Vec<CreateEventSlotPayload>,
}

#[derive(Deserialize)]
pub struct SubmitVoteRequest {
    pub slot_id: Uuid,
    pub is_available: bool,
    pub note: Option<String>,
}

pub async fn list_events_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Event>>, (StatusCode, Json<Value>)> {
    let events = sqlx::query_as::<_, Event>(
        "SELECT * FROM events ORDER BY created_at DESC"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(events))
}

pub async fn create_event_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<CreateEventRequest>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    let claims = extract_claims(&headers, &state.jwt_secret).await
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(json!({"error": "Unauthorized"}))))?;

    let mut tx = state.db.begin().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    let event = sqlx::query_as::<_, Event>(
        "INSERT INTO events (title, description, start_date, end_date, created_by)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING *"
    )
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(payload.start_date)
    .bind(payload.end_date)
    .bind(claims.sub)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({"error": e.to_string()}))))?;

    for (idx, slot) in payload.time_slots.iter().enumerate() {
        let order = slot.sort_order.unwrap_or(idx as i32);
        sqlx::query(
            "INSERT INTO event_time_slots (event_id, day_of_week, slot_label, sort_order)
             VALUES ($1, $2, $3, $4)"
        )
        .bind(event.id)
        .bind(&slot.day_of_week)
        .bind(&slot.slot_label)
        .bind(order)
        .execute(&mut *tx)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;
    }

    tx.commit().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok((StatusCode::CREATED, Json(json!(event))))
}

pub async fn close_event_handler(
    State(state): State<Arc<AppState>>,
    Path(event_id): Path<Uuid>,
    headers: HeaderMap,
) -> Result<Json<Event>, (StatusCode, Json<Value>)> {
    let claims = extract_claims(&headers, &state.jwt_secret).await;
    let changer_id = claims.as_ref().map(|c| c.sub);

    let event = sqlx::query_as::<_, Event>(
        "UPDATE events SET status = 'closed', closed_at = NOW() WHERE id = $1 RETURNING *"
    )
    .bind(event_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Event not found"}))))?;

    if let Some(producer) = &state.kafka_producer {
        let event_payload = EventStatusChangedEvent {
            event_id: event.id,
            title: event.title.clone(),
            old_status: "open".to_string(),
            new_status: "closed".to_string(),
            changed_by: changer_id,
            changed_at: Utc::now(),
        };

        if let Ok(event_json) = serde_json::to_string(&event_payload) {
            let key_str = event.id.to_string();
            let record = FutureRecord::to(TOPIC_EVENT_STATUS_CHANGED)
                .payload(&event_json)
                .key(&key_str);
            let _ = producer.send(record, Duration::from_secs(3)).await;
        }
    }

    Ok(Json(event))
}

pub async fn submit_vote_handler(
    State(state): State<Arc<AppState>>,
    Path(event_id): Path<Uuid>,
    headers: HeaderMap,
    Json(payload): Json<SubmitVoteRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let claims = extract_claims(&headers, &state.jwt_secret).await
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(json!({"error": "Unauthorized"}))))?;

    let event = sqlx::query_as::<_, Event>("SELECT * FROM events WHERE id = $1")
        .bind(event_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Event not found"}))))?;

    if event.status != EventStatus::Open {
        return Err((StatusCode::FORBIDDEN, Json(json!({"error": "Voting is closed for this event"}))));
    }

    sqlx::query(
        "INSERT INTO votes (event_id, user_id, slot_id, is_available, note, updated_at)
         VALUES ($1, $2, $3, $4, $5, NOW())
         ON CONFLICT (event_id, user_id, slot_id)
         DO UPDATE SET is_available = EXCLUDED.is_available, note = EXCLUDED.note, updated_at = NOW()"
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

pub async fn get_event_details_handler(
    State(state): State<Arc<AppState>>,
    Path(event_id): Path<Uuid>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let event = sqlx::query_as::<_, Event>("SELECT * FROM events WHERE id = $1")
        .bind(event_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Event not found"}))))?;

    let slots = sqlx::query_as::<_, EventTimeSlot>(
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
