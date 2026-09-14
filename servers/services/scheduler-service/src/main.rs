mod consumer;
mod csp_solver;

use csac_proto::timetable::scheduler_service_server::{SchedulerService, SchedulerServiceServer};
use csac_proto::timetable::{
    HealthCheckRequest, HealthCheckResponse, SolveTimetableRequest, SolveTimetableResponse,
    SolverStats,
};
use rdkafka::producer::FutureProducer;
use rdkafka::ClientConfig;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
pub struct SchedulerServiceImpl;

#[tonic::async_trait]
impl SchedulerService for SchedulerServiceImpl {
    async fn health_check(
        &self,
        _request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        Ok(Response::new(HealthCheckResponse {
            status: "HEALTHY".into(),
        }))
    }

    async fn solve_timetable(
        &self,
        request: Request<SolveTimetableRequest>,
    ) -> Result<Response<SolveTimetableResponse>, Status> {
        let req = request.into_inner();
        let total_songs = req.songs.len() as i32;

        Ok(Response::new(SolveTimetableResponse {
            schedule: vec![],
            unresolved: vec![],
            conflicts: vec![],
            stats: Some(SolverStats {
                total_requested: total_songs,
                total_scheduled: 0,
                perfect_attendance_count: 0,
                partial_attendance_count: 0,
            }),
        }))
    }
}

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

    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
        .unwrap_or_else(|e| {
            tracing::warn!("Postgres connection failed in scheduler-service: {}. Proceeding in degraded mode.", e);
            PgPoolOptions::new().connect_lazy(&db_url).expect("Lazy pool creation")
        });

    let redis_client = redis::Client::open(redis_url.clone()).expect("Invalid Redis URL");
    let redis_conn = match redis::aio::ConnectionManager::new(redis_client).await {
        Ok(c) => c,
        Err(e) => {
            tracing::warn!("Redis connection failed in scheduler-service: {}. Creating fallback.", e);
            let c2 = redis::Client::open(redis_url).expect("Invalid Redis URL");
            redis::aio::ConnectionManager::new(c2).await.expect("Fallback connection manager")
        }
    };

    let kafka_producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", &kafka_brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Failed to create Kafka producer");

    // Spawn Kafka Consumer task in background
    let brokers_clone = kafka_brokers.clone();
    let db_clone = db.clone();
    let redis_conn_clone = redis_conn.clone();
    let producer_clone = kafka_producer.clone();
    tokio::spawn(async move {
        consumer::run_kafka_scheduler_consumer(
            brokers_clone,
            db_clone,
            redis_conn_clone,
            producer_clone,
        )
        .await;
    });

    let addr: SocketAddr = "0.0.0.0:50051".parse()?;
    let scheduler_svc = SchedulerServiceImpl::default();

    tracing::info!("CSAC SchedulerService (gRPC + Kafka Consumer) listening on {}", addr);

    Server::builder()
        .add_service(SchedulerServiceServer::new(scheduler_svc))
        .serve(addr)
        .await?;

    Ok(())
}
