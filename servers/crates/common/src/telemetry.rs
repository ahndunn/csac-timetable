use chrono::Utc;

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
