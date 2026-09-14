mod features;
mod middleware;
mod routes;
mod state;

use csac_common::OpenObserveClient;
use rdkafka::{producer::FutureProducer, ClientConfig};
use sqlx::postgres::PgPoolOptions;
use std::{net::SocketAddr, sync::Arc};

use crate::routes::create_router;
use crate::state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://csac_admin:csac_password@localhost:5432/csac_timetable".to_string());
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
    let kafka_brokers = std::env::var("KAFKA_BROKERS").unwrap_or_else(|_| "localhost:9092".to_string());
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "default_jwt_secret_for_local_testing".to_string());

    tracing::info!("Connecting to PostgreSQL: {}", db_url);
    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&db_url)
        .await
        .unwrap_or_else(|e| {
            tracing::warn!("PostgreSQL connection error in gateway: {}. Proceeding in degraded mode.", e);
            PgPoolOptions::new().connect_lazy(&db_url).expect("Lazy pool creation")
        });

    tracing::info!("Connecting to Redis: {}", redis_url);
    let redis_client = redis::Client::open(redis_url.clone()).expect("Invalid Redis URL");
    let redis_conn = match redis::aio::ConnectionManager::new(redis_client).await {
        Ok(c) => c,
        Err(e) => {
            tracing::warn!("Redis connection manager error: {}. Creating fallback.", e);
            let c2 = redis::Client::open(redis_url).expect("Invalid Redis URL");
            redis::aio::ConnectionManager::new(c2).await.expect("Fallback connection manager")
        }
    };

    tracing::info!("Initializing Kafka Producer: {}", kafka_brokers);
    let kafka_producer: Option<FutureProducer> = ClientConfig::new()
        .set("bootstrap.servers", &kafka_brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .ok();

    let oo_client = OpenObserveClient::new("gateway");

    let app_state = Arc::new(AppState {
        db,
        redis: redis_conn,
        kafka_producer,
        jwt_secret,
        oo_client,
    });

    let app = create_router(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("CSAC Gateway reverse proxy listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
