use chrono::Utc;
use csac_common::sprint_schedule_channel;
use csac_kafka_events::{
    ScheduleCompletedEvent, ScheduleRequestedEvent, TOPIC_SCHEDULER_COMPLETED,
    TOPIC_SCHEDULER_REQUESTED,
};
use rdkafka::{
    consumer::{CommitMode, Consumer, StreamConsumer},
    producer::{FutureProducer, FutureRecord},
    ClientConfig, Message,
};
use redis::AsyncCommands;
use sqlx::PgPool;
use std::time::Duration;

use crate::csp_solver::{solve_sprint_csp, ScheduleRequest};

pub async fn run_kafka_scheduler_consumer(
    brokers: String,
    db: PgPool,
    mut redis_conn: redis::aio::ConnectionManager,
    producer: FutureProducer,
) {
    let consumer: StreamConsumer = match ClientConfig::new()
        .set("bootstrap.servers", &brokers)
        .set("group.id", "csac-scheduler-service-group")
        .set("enable.auto.commit", "true")
        .set("auto.offset.reset", "earliest")
        .create()
    {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to create Kafka scheduler consumer: {:?}", e);
            return;
        }
    };

    if let Err(e) = consumer.subscribe(&[TOPIC_SCHEDULER_REQUESTED]) {
        tracing::error!("Failed to subscribe to Kafka topic {}: {:?}", TOPIC_SCHEDULER_REQUESTED, e);
        return;
    }

    tracing::info!("Kafka Scheduler consumer subscribed to topic: {}", TOPIC_SCHEDULER_REQUESTED);

    loop {
        match consumer.recv().await {
            Err(e) => {
                tracing::error!("Kafka scheduler recv error: {:?}", e);
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
            Ok(msg) => {
                if let Some(payload) = msg.payload() {
                    if let Ok(event) = serde_json::from_slice::<ScheduleRequestedEvent>(payload) {
                        tracing::info!(
                            "Received ScheduleRequestedEvent: run_id={}, sprint_id={}",
                            event.run_id,
                            event.sprint_id
                        );

                        // 1. Mark status = 'processing' in PostgreSQL
                        let _ = sqlx::query(
                            "UPDATE sprint_schedule_runs SET status = 'processing' WHERE id = $1"
                        )
                        .bind(event.run_id)
                        .execute(&db)
                        .await;

                        // 2. Broadcast 'processing' to Redis Pub/Sub
                        let channel = sprint_schedule_channel(&event.sprint_id.to_string());
                        let proc_payload = serde_json::json!({
                            "event": "schedule_status",
                            "sprint_id": event.sprint_id.to_string(),
                            "run_id": event.run_id.to_string(),
                            "status": "processing"
                        });
                        let _: Result<(), _> = redis_conn.publish(&channel, proc_payload.to_string()).await;

                        // 3. Execute CSP solver heuristic
                        let req = ScheduleRequest {
                            sprint_id: event.sprint_id.to_string(),
                            run_id: event.run_id.to_string(),
                            triggered_by: event.triggered_by.to_string(),
                        };
                        let result = solve_sprint_csp(&req);

                        // 4. Update status = 'completed' in PostgreSQL
                        let assignments_json = serde_json::to_value(&result.assignments).unwrap_or_default();
                        let duration_i32 = (result.duration_ms.min(i32::MAX as u128)) as i32;
                        let conflict_i32 = result.conflict_count as i32;

                        let _ = sqlx::query(
                            "UPDATE sprint_schedule_runs
                             SET status = 'completed',
                                 duration_ms = $1,
                                 score = $2,
                                 conflict_count = $3,
                                 assignments = $4,
                                 completed_at = NOW()
                             WHERE id = $5"
                        )
                        .bind(duration_i32)
                        .bind(result.score)
                        .bind(conflict_i32)
                        .bind(assignments_json)
                        .bind(event.run_id)
                        .execute(&db)
                        .await;

                        // 5. Broadcast 'schedule_updated' to Redis Pub/Sub
                        let completed_payload = serde_json::json!({
                            "event": "schedule_updated",
                            "sprint_id": event.sprint_id.to_string(),
                            "run_id": event.run_id.to_string(),
                            "status": "completed",
                            "score": result.score,
                            "conflict_count": result.conflict_count,
                            "duration_ms": result.duration_ms
                        });
                        let _: Result<(), _> = redis_conn.publish(&channel, completed_payload.to_string()).await;

                        // 6. Publish ScheduleCompletedEvent to Kafka
                        let comp_event = ScheduleCompletedEvent {
                            run_id: event.run_id,
                            sprint_id: event.sprint_id,
                            status: "completed".to_string(),
                            score: result.score,
                            conflict_count: result.conflict_count,
                            duration_ms: result.duration_ms as u64,
                            completed_at: Utc::now(),
                        };
                        if let Ok(comp_json) = serde_json::to_string(&comp_event) {
                            let key_str = event.sprint_id.to_string();
                            let record = FutureRecord::to(TOPIC_SCHEDULER_COMPLETED)
                                .payload(&comp_json)
                                .key(&key_str);
                            let _ = producer.send(record, Duration::from_secs(3)).await;
                        }

                        tracing::info!(
                            "Completed schedule calculation for sprint_id={}, score={}",
                            event.sprint_id,
                            result.score
                        );
                    }
                }
                let _ = consumer.commit_message(&msg, CommitMode::Async);
            }
        }
    }
}
