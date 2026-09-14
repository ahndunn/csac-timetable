use axum::{
    extract::State,
    http::HeaderMap,
    middleware::Next,
    response::Response,
};
use serde_json::json;
use std::{sync::Arc, time::Instant};

use crate::middleware::auth::extract_claims;
use crate::state::AppState;

pub async fn telemetry_middleware(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    request: axum::extract::Request,
    next: Next,
) -> Response {
    let start = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();
    let path = uri.path().to_string();

    let claims = extract_claims(&headers, &state.jwt_secret).await;
    let user_id = claims.as_ref().map(|c| c.sub.to_string());
    let role = claims.as_ref().map(|c| c.role.as_str());

    let response = next.run(request).await;
    let latency_ms = start.elapsed().as_millis() as u64;
    let status = response.status().as_u16();

    let meta = json!({
        "method": method.as_str(),
        "path": path,
        "status": status,
        "latency_ms": latency_ms,
        "user_id": user_id,
        "role": role,
    });

    if status >= 500 {
        tracing::error!(
            method = %method,
            path = %path,
            status = %status,
            latency_ms = %latency_ms,
            user_id = ?user_id,
            role = ?role,
            "HTTP Request Error"
        );
    } else if status >= 400 {
        tracing::warn!(
            method = %method,
            path = %path,
            status = %status,
            latency_ms = %latency_ms,
            user_id = ?user_id,
            role = ?role,
            "HTTP Request Warning"
        );
    } else {
        tracing::info!(
            method = %method,
            path = %path,
            status = %status,
            latency_ms = %latency_ms,
            user_id = ?user_id,
            role = ?role,
            "HTTP Request Handled"
        );
    }

    state
        .oo_client
        .emit_log(
            if status >= 500 { "ERROR" } else if status >= 400 { "WARN" } else { "INFO" },
            &format!("HTTP {} {}", method, path),
            meta,
        )
        .await;

    response
}
