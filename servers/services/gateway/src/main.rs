use axum::{
    routing::{get, post},
    Json, Router,
};
use serde_json::{json, Value};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

async fn healthz() -> &'static str {
    "OK"
}

async fn solve_proxy(Json(payload): Json<Value>) -> Json<Value> {
    // Edge Gateway proxies request to downstream Tonic gRPC SchedulerService
    Json(json!({
        "status": "FORWARDED_TO_GRPC",
        "mock_result": {
            "schedule": [],
            "unresolved": [],
            "stats": { "totalRequested": 0, "totalScheduled": 0 }
        },
        "received_payload_keys": payload.as_object().map(|m| m.keys().cloned().collect::<Vec<_>>()).unwrap_or_default()
    }))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/api/v1/schedule/solve", post(solve_proxy))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("CSAC Gateway reverse proxy listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
