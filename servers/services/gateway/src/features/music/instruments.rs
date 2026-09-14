use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use csac_common::{
    Instrument, InstrumentAvailability, InstrumentOwnership, InstrumentReservation,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;
use uuid::Uuid;

use crate::middleware::extract_claims;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct RegisterInstrumentRequest {
    pub name: String,
    pub code: String,
    pub category: String,
    pub ownership_type: InstrumentOwnership,
    pub owner_user_id: Option<Uuid>,
    pub custody_user_id: Option<Uuid>,
    pub custody_location: Option<String>,
    pub availability_status: Option<InstrumentAvailability>,
    pub notes: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateInstrumentStatusRequest {
    pub availability_status: Option<InstrumentAvailability>,
    pub custody_user_id: Option<Uuid>,
    pub custody_location: Option<String>,
    pub notes: Option<String>,
}

#[derive(Deserialize)]
pub struct ReserveInstrumentRequest {
    pub instrument_id: Uuid,
    pub music_number_id: Uuid,
    pub day_of_week: String,
    pub slot_label: String,
    pub notes: Option<String>,
}

pub async fn list_instruments_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let instruments = sqlx::query_as::<_, Instrument>(
        "SELECT * FROM instruments ORDER BY category ASC, name ASC"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    let reservations = sqlx::query_as::<_, InstrumentReservation>(
        "SELECT * FROM instrument_reservations ORDER BY day_of_week ASC, slot_label ASC"
    )
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "instruments": instruments,
        "reservations": reservations,
    })))
}

pub async fn register_instrument_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterInstrumentRequest>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    let inst = sqlx::query_as::<_, Instrument>(
        "INSERT INTO instruments (name, code, category, ownership_type, owner_user_id, custody_user_id, custody_location, availability_status, notes)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
         RETURNING *"
    )
    .bind(&payload.name)
    .bind(&payload.code)
    .bind(&payload.category)
    .bind(payload.ownership_type)
    .bind(payload.owner_user_id)
    .bind(payload.custody_user_id)
    .bind(payload.custody_location.as_deref().unwrap_or("Club Studio Locker"))
    .bind(payload.availability_status.unwrap_or(InstrumentAvailability::FreeToBorrow))
    .bind(&payload.notes)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({"error": e.to_string()}))))?;

    Ok((StatusCode::CREATED, Json(json!(inst))))
}

pub async fn update_instrument_status_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateInstrumentStatusRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let inst = sqlx::query_as::<_, Instrument>(
        "UPDATE instruments
         SET availability_status = COALESCE($1, availability_status),
             custody_user_id = COALESCE($2, custody_user_id),
             custody_location = COALESCE($3, custody_location),
             notes = COALESCE($4, notes),
             updated_at = NOW()
         WHERE id = $5
         RETURNING *"
    )
    .bind(payload.availability_status)
    .bind(payload.custody_user_id)
    .bind(payload.custody_location)
    .bind(payload.notes)
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Instrument not found"}))))?;

    Ok(Json(json!(inst)))
}

pub async fn reserve_instrument_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<ReserveInstrumentRequest>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    let claims = extract_claims(&headers, &state.jwt_secret).await
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(json!({"error": "Unauthorized"}))))?;

    let conflict = sqlx::query(
        "SELECT id FROM instrument_reservations WHERE instrument_id = $1 AND day_of_week = $2 AND slot_label = $3"
    )
    .bind(payload.instrument_id)
    .bind(&payload.day_of_week)
    .bind(&payload.slot_label)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    if conflict.is_some() {
        return Err((
            StatusCode::CONFLICT,
            Json(json!({
                "error": "Instrument is already reserved for this day and time slot by another music number",
                "instrument_id": payload.instrument_id,
                "day_of_week": payload.day_of_week,
                "slot_label": payload.slot_label,
            })),
        ));
    }

    let res = sqlx::query_as::<_, InstrumentReservation>(
        "INSERT INTO instrument_reservations (instrument_id, music_number_id, reserved_by, day_of_week, slot_label, notes)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING *"
    )
    .bind(payload.instrument_id)
    .bind(payload.music_number_id)
    .bind(claims.sub)
    .bind(payload.day_of_week)
    .bind(payload.slot_label)
    .bind(payload.notes)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok((StatusCode::CREATED, Json(json!(res))))
}
