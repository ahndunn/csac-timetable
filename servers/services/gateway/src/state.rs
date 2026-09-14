use csac_common::OpenObserveClient;
use rdkafka::producer::FutureProducer;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub redis: redis::aio::ConnectionManager,
    pub kafka_producer: Option<FutureProducer>,
    pub jwt_secret: String,
    pub oo_client: OpenObserveClient,
}
