use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteUploadRequestedEvent {
    pub event_id: String,
    pub user_id: Option<String>,
    pub file_name: String,
    pub payload_s3_key: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleGeneratedEvent {
    pub event_id: String,
    pub schedule_id: String,
    pub total_sessions: usize,
    pub timestamp: DateTime<Utc>,
}
