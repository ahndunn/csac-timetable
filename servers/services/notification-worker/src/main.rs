use csac_common::OpenObserveClient;
use csac_kafka_events::{
    AdminOtpGeneratedEvent, UserCreatedEvent, TOPIC_ADMIN_OTP_GENERATED, TOPIC_USER_CREATED,
};
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use rdkafka::{
    consumer::{CommitMode, Consumer, StreamConsumer},
    ClientConfig, Message as KafkaMessage,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    let oo_client = OpenObserveClient::new("notification-worker");
    tracing::info!("Starting CSAC Notification Worker...");
    oo_client
        .emit_log("INFO", "Notification worker initializing", serde_json::json!({}))
        .await;

    let kafka_brokers = env::var("KAFKA_BROKERS").unwrap_or_else(|_| "localhost:9092".to_string());
    let consumer_group = "csac-notification-group";

    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", &kafka_brokers)
        .set("group.id", consumer_group)
        .set("enable.auto.commit", "true")
        .set("auto.offset.reset", "earliest")
        .create()
        .expect("Failed to create Kafka consumer");

    consumer
        .subscribe(&[TOPIC_USER_CREATED, TOPIC_ADMIN_OTP_GENERATED])
        .expect("Failed to subscribe to Kafka topics");

    tracing::info!(
        "Subscribed to Kafka topics: [{}, {}]",
        TOPIC_USER_CREATED,
        TOPIC_ADMIN_OTP_GENERATED
    );

    let smtp_host = env::var("SMTP_HOST").unwrap_or_else(|_| "localhost".to_string());
    let smtp_port: u16 = env::var("SMTP_PORT")
        .unwrap_or_else(|_| "1025".to_string())
        .parse()
        .unwrap_or(1025);
    let smtp_from = env::var("SMTP_FROM").unwrap_or_else(|_| "noreply@csac.local".to_string());

    let mut mailer_builder = AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&smtp_host)
        .port(smtp_port);

    if let (Ok(user), Ok(pass)) = (env::var("SMTP_USER"), env::var("SMTP_PASSWORD")) {
        mailer_builder = mailer_builder.credentials(Credentials::new(user, pass));
    }

    let mailer = mailer_builder.build();

    loop {
        match consumer.recv().await {
            Err(e) => {
                tracing::error!("Kafka receive error: {:?}", e);
            }
            Ok(msg) => {
                let topic = msg.topic();
                if let Some(payload) = msg.payload() {
                    match topic {
                        TOPIC_USER_CREATED => {
                            if let Ok(event) = serde_json::from_slice::<UserCreatedEvent>(payload) {
                                tracing::info!(
                                    "Processing UserCreatedEvent for: {} ({})",
                                    event.email,
                                    event.role
                                );
                                let email_body = format!(
                                    "Hello {},\n\nYour CSAC Timetable account has been created with role: {}\n\nLogin Email: {}\nInitial Password: {}\n\nPlease log in and update your password.\n\nBest regards,\nCSAC Admin Team",
                                    event.full_name, event.role, event.email, event.initial_password
                                );

                                let email_msg = Message::builder()
                                    .from(smtp_from.parse().unwrap())
                                    .to(event.email.parse().unwrap())
                                    .subject("Welcome to CSAC Timetable Studio — Your Account Credentials")
                                    .header(ContentType::TEXT_PLAIN)
                                    .body(email_body);

                                match email_msg {
                                    Ok(m) => {
                                        if let Err(err) = mailer.send(m).await {
                                            tracing::error!("Failed to send welcome email to {}: {:?}", event.email, err);
                                        } else {
                                            tracing::info!("Welcome email successfully sent to {}", event.email);
                                            oo_client.emit_log("INFO", "User credentials dispatched via email", serde_json::json!({
                                                "recipient": event.email,
                                                "user_id": event.user_id,
                                                "role": event.role
                                            })).await;
                                        }
                                    }
                                    Err(err) => tracing::error!("Failed to build email message: {:?}", err),
                                }
                            }
                        }
                        TOPIC_ADMIN_OTP_GENERATED => {
                            if let Ok(event) = serde_json::from_slice::<AdminOtpGeneratedEvent>(payload) {
                                tracing::info!(
                                    "Processing AdminOtpGeneratedEvent for: {}",
                                    event.email
                                );
                                let email_body = format!(
                                    "Hello Admin,\n\nA request to downgrade Admin '{}' to '{}' is pending quorum approval.\n\nYour One-Time Password (OTP) is: {}\n\nThis OTP is valid for {} seconds.\nEnter this code on the /admin/approve portal to submit your vote.\n\nCSAC Security Governance",
                                    event.target_admin_name, event.target_role, event.otp_code, event.ttl_seconds
                                );

                                let email_msg = Message::builder()
                                    .from(smtp_from.parse().unwrap())
                                    .to(event.email.parse().unwrap())
                                    .subject("[Security Alert] CSAC Admin Downgrade Quorum OTP")
                                    .header(ContentType::TEXT_PLAIN)
                                    .body(email_body);

                                match email_msg {
                                    Ok(m) => {
                                        if let Err(err) = mailer.send(m).await {
                                            tracing::error!("Failed to send OTP email to {}: {:?}", event.email, err);
                                        } else {
                                            tracing::info!("OTP email successfully sent to {}", event.email);
                                            oo_client.emit_log("INFO", "Admin OTP dispatched via email", serde_json::json!({
                                                "recipient": event.email,
                                                "proposal_id": event.proposal_id,
                                                "admin_id": event.admin_id
                                            })).await;
                                        }
                                    }
                                    Err(err) => tracing::error!("Failed to build OTP email message: {:?}", err),
                                }
                            }
                        }
                        _ => {}
                    }
                }
                let _ = consumer.commit_message(&msg, CommitMode::Async);
            }
        }
    }
}
