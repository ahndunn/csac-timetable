use axum::{
    middleware,
    routing::{get, post, put},
    Json, Router,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::features::{
    admin::{
        create_downgrade_proposal_handler, create_user_handler, list_proposals_handler,
        list_users_handler, request_proposal_otp_handler, update_user_role_handler,
        vote_proposal_handler,
    },
    auth::{login_handler, me_handler},
    events::{
        close_event_handler, create_event_handler, get_event_details_handler, list_events_handler,
        submit_vote_handler,
    },
    music::{
        create_music_number_handler, list_instruments_handler, list_music_numbers_handler,
        register_instrument_handler, reserve_instrument_handler, update_instrument_status_handler,
    },
    shows::{
        create_show_number_handler, create_show_roster_handler, delete_show_roster_handler,
        get_active_sprint_handler, get_show_overview_handler, get_show_sprint_history_handler,
        list_show_numbers_handler, list_show_roster_handler, submit_show_number_qc_handler,
        submit_show_sprint_availability_handler, update_show_number_lineup_handler,
        update_show_number_stage_handler, update_show_roster_handler,
    },
    sprints::{
        create_sprint_task_handler, list_sprint_tasks_handler, list_sprints_handler,
        review_task_handler, sprint_schedule_handler, sprint_schedule_sse_handler,
        submit_sprint_availability_handler,
    },
};
use crate::middleware::telemetry_middleware;
use crate::state::AppState;

async fn healthz() -> &'static str {
    "OK"
}

async fn solve_proxy(Json(payload): Json<Value>) -> Json<Value> {
    Json(json!({
        "status": "FORWARDED_TO_SCHEDULER",
        "mock_result": {
            "schedule": [],
            "unresolved": [],
            "stats": { "totalRequested": 0, "totalScheduled": 0 }
        },
        "payload": payload
    }))
}

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/api/v1/auth/login", post(login_handler))
        .route("/api/v1/auth/me", get(me_handler))
        .route("/api/v1/admin/users", get(list_users_handler).post(create_user_handler))
        .route("/api/v1/admin/users/:id/role", put(update_user_role_handler))
        .route("/api/v1/admin/users/:id/downgrade-proposal", post(create_downgrade_proposal_handler))
        .route("/api/v1/admin/approve/proposals", get(list_proposals_handler))
        .route("/api/v1/admin/approve/:id/request-otp", post(request_proposal_otp_handler))
        .route("/api/v1/admin/approve/:id/vote", post(vote_proposal_handler))
        .route("/api/v1/events", get(list_events_handler).post(create_event_handler))
        .route("/api/v1/events/:id", get(get_event_details_handler))
        .route("/api/v1/events/:id/close", put(close_event_handler))
        .route("/api/v1/events/:id/vote", post(submit_vote_handler))
        // Music Numbers & Fleet
        .route("/api/v1/music/numbers", get(list_music_numbers_handler).post(create_music_number_handler))
        .route("/api/v1/music/instruments", get(list_instruments_handler).post(register_instrument_handler))
        .route("/api/v1/music/instruments/:id/status", put(update_instrument_status_handler))
        .route("/api/v1/music/instruments/reserve", post(reserve_instrument_handler))
        // Agile Practice Sprints
        .route("/api/v1/sprints", get(list_sprints_handler))
        .route("/api/v1/sprints/:id/tasks", get(list_sprint_tasks_handler).post(create_sprint_task_handler))
        .route("/api/v1/sprints/:id/tasks/:task_id/review", put(review_task_handler))
        .route("/api/v1/sprints/:id/availability", post(submit_sprint_availability_handler))
        .route("/api/v1/sprints/:id/schedule", post(sprint_schedule_handler))
        .route("/api/v1/sprints/:id/schedule/stream", get(sprint_schedule_sse_handler))
        // Solver proxy
        .route("/api/v1/schedule/solve", post(solve_proxy))
        // Show Studio Routes (/api/v1/shows/*)
        .route("/api/v1/shows/:id/overview", get(get_show_overview_handler))
        .route("/api/v1/shows/:id/numbers", get(list_show_numbers_handler).post(create_show_number_handler))
        .route("/api/v1/shows/:id/numbers/:number_id/stage", put(update_show_number_stage_handler))
        .route("/api/v1/shows/:id/numbers/:number_id/lineup", put(update_show_number_lineup_handler))
        .route("/api/v1/shows/:id/numbers/:number_id/qc", post(submit_show_number_qc_handler))
        .route("/api/v1/shows/:id/roster", get(list_show_roster_handler).post(create_show_roster_handler))
        .route("/api/v1/shows/:id/roster/:member_id", put(update_show_roster_handler).delete(delete_show_roster_handler))
        .route("/api/v1/shows/:id/sprints/active", get(get_active_sprint_handler))
        .route("/api/v1/shows/:id/sprints/:sprint_id/availability", post(submit_show_sprint_availability_handler))
        .route("/api/v1/shows/:id/sprints/:sprint_id/history", get(get_show_sprint_history_handler))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            telemetry_middleware,
        ))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(app_state)
}
