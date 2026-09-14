use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "instrument_ownership", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum InstrumentOwnership {
    #[sqlx(rename = "club_property")]
    #[serde(rename = "club_property")]
    ClubProperty,
    #[sqlx(rename = "member_owned")]
    #[serde(rename = "member_owned")]
    MemberOwned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "instrument_availability", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum InstrumentAvailability {
    #[sqlx(rename = "free_to_borrow")]
    #[serde(rename = "free_to_borrow")]
    FreeToBorrow,
    #[sqlx(rename = "in_use")]
    #[serde(rename = "in_use")]
    InUse,
    #[sqlx(rename = "unavailable")]
    #[serde(rename = "unavailable")]
    Unavailable,
    #[sqlx(rename = "in_maintenance")]
    #[serde(rename = "in_maintenance")]
    InMaintenance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "music_number_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum MusicNumberStatus {
    #[sqlx(rename = "draft")]
    #[serde(rename = "draft")]
    Draft,
    #[sqlx(rename = "in_practice")]
    #[serde(rename = "in_practice")]
    InPractice,
    #[sqlx(rename = "ready_for_qc")]
    #[serde(rename = "ready_for_qc")]
    ReadyForQc,
    #[sqlx(rename = "qc_approved")]
    #[serde(rename = "qc_approved")]
    QcApproved,
    #[sqlx(rename = "stage_ready")]
    #[serde(rename = "stage_ready")]
    StageReady,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "task_type", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum TaskType {
    #[sqlx(rename = "study")]
    #[serde(rename = "study")]
    Study,
    #[sqlx(rename = "create")]
    #[serde(rename = "create")]
    Create,
    #[sqlx(rename = "review_qc")]
    #[serde(rename = "review_qc")]
    ReviewQc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "task_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    #[sqlx(rename = "todo")]
    #[serde(rename = "todo")]
    Todo,
    #[sqlx(rename = "in_progress")]
    #[serde(rename = "in_progress")]
    InProgress,
    #[sqlx(rename = "under_review")]
    #[serde(rename = "under_review")]
    UnderReview,
    #[sqlx(rename = "passed")]
    #[serde(rename = "passed")]
    Passed,
    #[sqlx(rename = "blocked")]
    #[serde(rename = "blocked")]
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Instrument {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub category: String,
    pub ownership_type: InstrumentOwnership,
    pub owner_user_id: Option<Uuid>,
    pub custody_user_id: Option<Uuid>,
    pub custody_location: Option<String>,
    pub availability_status: InstrumentAvailability,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MusicNumber {
    pub id: Uuid,
    pub event_id: Option<Uuid>,
    pub title: String,
    pub genre: Option<String>,
    pub pm_user_id: Option<Uuid>,
    pub target_sessions_per_week: i32,
    pub status: MusicNumberStatus,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MusicNumberMember {
    pub id: Uuid,
    pub music_number_id: Uuid,
    pub user_id: Uuid,
    pub instrument_role: String,
    pub is_lead: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PracticeSprint {
    pub id: Uuid,
    pub event_id: Option<Uuid>,
    pub name: String,
    pub sprint_goal: Option<String>,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PracticeTask {
    pub id: Uuid,
    pub sprint_id: Uuid,
    pub music_number_id: Uuid,
    pub task_type: TaskType,
    pub title: String,
    pub description: Option<String>,
    pub assigned_to: Option<Uuid>,
    pub qc_reviewer_id: Option<Uuid>,
    pub status: TaskStatus,
    pub qc_feedback: Option<String>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct InstrumentReservation {
    pub id: Uuid,
    pub instrument_id: Uuid,
    pub music_number_id: Uuid,
    pub reserved_by: Uuid,
    pub day_of_week: String,
    pub slot_label: String,
    pub session_date: Option<chrono::NaiveDate>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MemberSprintAvailability {
    pub id: Uuid,
    pub sprint_id: Uuid,
    pub user_id: Uuid,
    pub day_of_week: String,
    pub slot_label: String,
    pub is_available: bool,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SprintScheduleRun {
    pub id: Uuid,
    pub sprint_id: Uuid,
    pub triggered_by: Uuid,
    pub status: String,
    pub duration_ms: Option<i32>,
    pub score: Option<f64>,
    pub conflict_count: Option<i32>,
    pub assignments: Option<serde_json::Value>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}
