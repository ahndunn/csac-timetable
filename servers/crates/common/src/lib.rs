pub mod auth;
pub mod errors;
pub mod models;
pub mod telemetry;

pub use auth::*;
pub use errors::{AppError, AppResult};
pub use models::*;
pub use telemetry::*;

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

pub fn sprint_schedule_channel(sprint_id: &str) -> String {
    format!("sprint:schedule:{}", sprint_id)
}
