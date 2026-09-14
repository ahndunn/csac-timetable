use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use csac_common::{generate_random_password, hash_password, MusicNumber, MusicNumberMember, User, UserRole};
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;
use uuid::Uuid;

use crate::state::AppState;

#[derive(Deserialize)]
pub struct SaveRosterMemberReq {
    #[serde(rename = "fullName")]
    pub full_name: String,
    pub email: String,
    pub phone: Option<String>,
    #[serde(rename = "showRole")]
    pub show_role: String,
    #[serde(rename = "primaryInstrument")]
    pub primary_instrument: String,
    #[serde(rename = "secondaryInstruments")]
    pub secondary_instruments: Option<Vec<String>>,
    #[serde(rename = "practiceHours")]
    pub practice_hours: Option<i32>,
}

pub async fn list_show_roster_handler(
    State(state): State<Arc<AppState>>,
    Path(_show_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let users = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE status = 'active' ORDER BY full_name ASC"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": {"code": "DB_ERROR", "message": e.to_string(), "status": 500}}))))?;

    let mut roster = Vec::new();
    for (idx, u) in users.into_iter().enumerate() {
        let role_str = match u.role {
            UserRole::Admin => "DM",
            UserRole::Moderator => "PM",
            UserRole::Member => if idx % 2 == 0 { "QC" } else { "Performer" },
        };
        let primary_inst = match idx % 5 {
            0 => "vocal_lead",
            1 => "guitar_lead",
            2 => "bass",
            3 => "drums",
            _ => "keys",
        };

        let assigned_memberships = sqlx::query_as::<_, MusicNumberMember>(
            "SELECT * FROM music_number_members WHERE user_id = $1"
        )
        .bind(u.id)
        .fetch_all(&state.db)
        .await
        .unwrap_or_default();

        let mut assigned_titles = Vec::new();
        for mem in &assigned_memberships {
            let num_row = sqlx::query_as::<_, MusicNumber>(
                "SELECT * FROM music_numbers WHERE id = $1"
            )
            .bind(mem.music_number_id)
            .fetch_optional(&state.db)
            .await
            .unwrap_or(None);
            if let Some(nr) = num_row {
                assigned_titles.push(nr.title);
            }
        }

        let assigned_count = assigned_titles.len();
        let total_practice_hours = if assigned_count > 0 { (assigned_count * 4) as i32 } else { 2 };
        let workload_status = if total_practice_hours > 16 { "fatigued" } else if total_practice_hours > 8 { "moderate" } else { "optimal" };

        roster.push(json!({
            "id": format!("mem-{}", u.id),
            "userId": u.id.to_string(),
            "fullName": u.full_name,
            "email": u.email,
            "phone": "+84 901 234 567",
            "showRole": role_str,
            "isDM": u.role == UserRole::Admin,
            "pmSongTitles": if u.role == UserRole::Moderator { vec!["PHONECERT"] } else { vec![] },
            "qcSongTitles": if role_str == "QC" { vec!["NÀNG THƠ"] } else { vec![] },
            "primaryInstrument": primary_inst,
            "secondaryInstruments": ["guitar_rhythm"],
            "assignedSongCount": assigned_count,
            "assignedSongTitles": assigned_titles,
            "totalPracticeHours": total_practice_hours,
            "workloadStatus": workload_status,
            "attendanceRate": 100,
            "joinedAt": u.created_at.format("%Y-%m-%d").to_string()
        }));
    }

    Ok(Json(json!(roster)))
}

pub async fn create_show_roster_handler(
    State(state): State<Arc<AppState>>,
    Path(_show_id): Path<String>,
    Json(payload): Json<SaveRosterMemberReq>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    let target_role = match payload.show_role.as_str() {
        "DM" => UserRole::Admin,
        "PM" => UserRole::Moderator,
        _ => UserRole::Member,
    };

    let initial_pass = generate_random_password(10);
    let hash = hash_password(&initial_pass)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": {"code": "HASH_ERROR", "message": e.to_string(), "status": 500}}))))?;

    let created_user = sqlx::query_as::<_, User>(
        "INSERT INTO users (email, full_name, password_hash, role, status)
         VALUES ($1, $2, $3, $4, 'active')
         ON CONFLICT (email) DO UPDATE SET full_name = EXCLUDED.full_name, role = EXCLUDED.role
         RETURNING *"
    )
    .bind(&payload.email)
    .bind(&payload.full_name)
    .bind(&hash)
    .bind(target_role)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({"error": {"code": "USER_CREATION_FAILED", "message": e.to_string(), "status": 400}}))))?;

    let new_member = json!({
        "id": format!("mem-{}", created_user.id),
        "userId": created_user.id.to_string(),
        "fullName": created_user.full_name,
        "email": created_user.email,
        "phone": payload.phone.unwrap_or_default(),
        "showRole": payload.show_role,
        "isDM": payload.show_role == "DM",
        "primaryInstrument": payload.primary_instrument,
        "secondaryInstruments": payload.secondary_instruments.unwrap_or_default(),
        "assignedSongCount": 0,
        "assignedSongTitles": [],
        "totalPracticeHours": payload.practice_hours.unwrap_or(4),
        "workloadStatus": "optimal",
        "attendanceRate": 100,
        "joinedAt": created_user.created_at.format("%Y-%m-%d").to_string()
    });

    Ok((StatusCode::CREATED, Json(new_member)))
}

pub async fn update_show_roster_handler(
    State(state): State<Arc<AppState>>,
    Path((_show_id, member_id)): Path<(String, String)>,
    Json(payload): Json<SaveRosterMemberReq>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let clean_id = member_id.trim_start_matches("mem-");
    let user_uuid = Uuid::parse_str(clean_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(json!({"error": {"code": "INVALID_ID", "message": format!("Invalid member ID '{}'", member_id), "status": 400}}))))?;

    let target_role = match payload.show_role.as_str() {
        "DM" => UserRole::Admin,
        "PM" => UserRole::Moderator,
        _ => UserRole::Member,
    };

    let updated = sqlx::query_as::<_, User>(
        "UPDATE users SET full_name = $1, email = $2, role = $3, updated_at = NOW() WHERE id = $4 RETURNING *"
    )
    .bind(&payload.full_name)
    .bind(&payload.email)
    .bind(target_role)
    .bind(user_uuid)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": {"code": "DB_ERROR", "message": e.to_string(), "status": 500}}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": {"code": "MEMBER_NOT_FOUND", "message": format!("Member with ID '{}' not found", member_id), "status": 404}}))))?;

    Ok(Json(json!({
        "id": member_id,
        "userId": updated.id.to_string(),
        "fullName": updated.full_name,
        "email": updated.email,
        "phone": payload.phone.unwrap_or_default(),
        "showRole": payload.show_role,
        "isDM": payload.show_role == "DM",
        "primaryInstrument": payload.primary_instrument,
        "secondaryInstruments": payload.secondary_instruments.unwrap_or_default(),
        "totalPracticeHours": payload.practice_hours.unwrap_or(4),
        "updatedAt": Utc::now()
    })))
}

pub async fn delete_show_roster_handler(
    State(state): State<Arc<AppState>>,
    Path((_show_id, member_id)): Path<(String, String)>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let clean_id = member_id.trim_start_matches("mem-");
    let user_uuid = Uuid::parse_str(clean_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(json!({"error": {"code": "INVALID_ID", "message": format!("Invalid member ID '{}'", member_id), "status": 400}}))))?;

    let res = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(user_uuid)
        .execute(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": {"code": "DB_ERROR", "message": e.to_string(), "status": 500}}))))?;

    if res.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, Json(json!({"error": {"code": "MEMBER_NOT_FOUND", "message": format!("Member with ID '{}' not found", member_id), "status": 404}}))));
    }

    Ok(Json(json!({
        "status": "deleted",
        "memberId": member_id
    })))
}
