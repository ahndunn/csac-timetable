#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("CSAC Vote Ingestion Service started. Listening for Kafka upload events...");
}
