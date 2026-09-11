use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const TOPIC_USER_CREATED: &str = "csac.user.created";
pub const TOPIC_ADMIN_OTP_GENERATED: &str = "csac.admin.otp_generated";
pub const TOPIC_EVENT_STATUS_CHANGED: &str = "csac.event.status_changed";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCreatedEvent {
    pub user_id: Uuid,
    pub email: String,
    pub full_name: String,
    pub initial_password: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminOtpGeneratedEvent {
    pub admin_id: Uuid,
    pub email: String,
    pub proposal_id: Uuid,
    pub target_admin_name: String,
    pub target_role: String,
    pub otp_code: String,
    pub ttl_seconds: u64,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventStatusChangedEvent {
    pub event_id: Uuid,
    pub title: String,
    pub old_status: String,
    pub new_status: String,
    pub changed_by: Option<Uuid>,
    pub changed_at: DateTime<Utc>,
}
