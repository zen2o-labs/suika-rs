use rdkafka::{producer::FutureProducer, ClientConfig};

use crate::{config::Settings, utils::error::AppError};

pub async fn kafka_producer(settings: &Settings) -> Result<FutureProducer, AppError> {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", &settings.kafka.bootstrap_servers)
        .set("message.timeout.ms", &settings.kafka.timeout_ms.to_string())
        .set("queue.buffering.max.messages", "100000")
        .set("queue.buffering.max.ms", "1000")
        .create()
        .map_err(|e| AppError::KafkaError(e.to_string()))?;

    tracing::info!("Kafka producer initialized successfully");
    Ok(producer)
}
