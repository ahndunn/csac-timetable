use axum::http::HeaderMap;
use csac_common::{verify_jwt, Claims};

pub async fn extract_claims(headers: &HeaderMap, secret: &str) -> Option<Claims> {
    if let Some(auth_header) = headers.get("Authorization").and_then(|h| h.to_str().ok()) {
        if auth_header.starts_with("Bearer ") {
            let token = &auth_header[7..];
            return verify_jwt(token, secret).ok();
        }
    }
    None
}
