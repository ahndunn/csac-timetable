use csac_proto::timetable::scheduler_service_server::{SchedulerService, SchedulerServiceServer};
use csac_proto::timetable::{
    HealthCheckRequest, HealthCheckResponse, SolveTimetableRequest, SolveTimetableResponse,
    SolverStats,
};
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
    tracing_subscriber::fmt::init();

    let addr: SocketAddr = "0.0.0.0:50051".parse()?;
    let scheduler_svc = SchedulerServiceImpl::default();

    tracing::info!("CSAC SchedulerService (gRPC) listening on {}", addr);

    Server::builder()
        .add_service(SchedulerServiceServer::new(scheduler_svc))
        .serve(addr)
        .await?;

    Ok(())
}
