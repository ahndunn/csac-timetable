use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{DateTime, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rand::Rng;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Authentication failed: {0}")]
    Unauthorized(String),
    #[error("Forbidden: {0}")]
    Forbidden(String),
    #[error("Conflict: {0}")]
    Conflict(String),
    #[error("Rate limit exceeded")]
    RateLimited,
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),
    #[error("Internal error: {0}")]
    Internal(String),
}

pub type AppResult<T> = Result<T, AppError>;

// ==========================================
// Domain Enums & Models
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Moderator,
    Member,
}

impl UserRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserRole::Admin => "admin",
            UserRole::Moderator => "moderator",
            UserRole::Member => "member",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum UserStatus {
    Active,
    Suspended,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "event_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum EventStatus {
    Draft,
    Open,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "proposal_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ProposalStatus {
    Pending,
    Approved,
    Rejected,
    Expired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "vote_decision", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum VoteDecision {
    Approve,
    Reject,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub full_name: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: UserRole,
    pub status: UserStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Event {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
    pub status: EventStatus,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct EventTimeSlot {
    pub id: Uuid,
    pub event_id: Uuid,
    pub day_of_week: String,
    pub slot_label: String,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AdminDowngradeProposal {
    pub id: Uuid,
    pub target_admin_id: Uuid,
    pub target_role: UserRole,
    pub initiated_by: Uuid,
    pub reason: String,
    pub total_admins_at_proposal: i32,
    pub required_approvals: i32,
    pub current_approvals: i32,
    pub status: ProposalStatus,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

// ==========================================
// Argon2id Password Cryptography
// ==========================================

pub fn hash_password(password: &str) -> AppResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| AppError::Internal(format!("Password hashing error: {}", e)))
}

pub fn verify_password(password: &str, password_hash: &str) -> AppResult<bool> {
    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|e| AppError::Internal(format!("Invalid password hash: {}", e)))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

pub fn generate_random_password(len: usize) -> String {
    const CHARSET: &[u8] = b"abcdefghjkmnpqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ23456789!@#$%";
    let mut rng = rand::thread_rng();
    (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

pub fn generate_otp_code() -> String {
    let mut rng = rand::thread_rng();
    let num: u32 = rng.gen_range(100_000..=999_999);
    format!("{:06}", num)
}

// ==========================================
// JWT Claims & Verification
// ==========================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,
    pub email: String,
    pub name: String,
    pub role: UserRole,
    pub exp: usize,
    pub iat: usize,
}

pub fn generate_jwt(user: &User, secret: &str, duration_secs: i64) -> AppResult<String> {
    let now = Utc::now();
    let exp = now + chrono::Duration::seconds(duration_secs);
    let claims = Claims {
        sub: user.id,
        email: user.email.clone(),
        name: user.full_name.clone(),
        role: user.role,
        exp: exp.timestamp() as usize,
        iat: now.timestamp() as usize,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(format!("JWT encode error: {}", e)))
}

pub fn verify_jwt(token: &str, secret: &str) -> AppResult<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| AppError::Unauthorized(format!("Invalid or expired token: {}", e)))
}

// ==========================================
// Admin Quorum Calculation Formula
// ==========================================

/// Calculate required admin approvals: M = min(ceil(N / 2), 3)
/// Returns error if N <= 1 (Sole admin protection)
pub fn calculate_required_approvals(total_admins: usize) -> AppResult<usize> {
    if total_admins <= 1 {
        return Err(AppError::Validation(
            "Cannot downgrade the sole active administrator (system requires at least 1 admin)"
                .to_string(),
        ));
    }
    let half_ceil = (total_admins + 1) / 2;
    Ok(std::cmp::min(half_ceil, 3))
}

// ==========================================
// OpenObserve Structured Telemetry Logger
// ==========================================

#[derive(Debug, Clone)]
pub struct OpenObserveClient {
    client: reqwest::Client,
    url: Option<String>,
    auth_header: Option<String>,
    service_name: String,
}

impl OpenObserveClient {
    pub fn new(service_name: &str) -> Self {
        let url = std::env::var("OPENOBSERVE_URL").ok();
        let auth_header = std::env::var("OPENOBSERVE_AUTH").ok();
        Self {
            client: reqwest::Client::new(),
            url,
            auth_header,
            service_name: service_name.to_string(),
        }
    }

    pub async fn emit_log(&self, level: &str, message: &str, metadata: serde_json::Value) {
        if let (Some(url), Some(auth)) = (&self.url, &self.auth_header) {
            let payload = serde_json::json!([{
                "timestamp": Utc::now().to_rfc3339(),
                "service": self.service_name,
                "level": level,
                "message": message,
                "data": metadata,
            }]);

            let _ = self
                .client
                .post(url)
                .header("Authorization", auth)
                .header("Content-Type", "application/json")
                .json(&payload)
                .send()
                .await;
        }
    }
}
