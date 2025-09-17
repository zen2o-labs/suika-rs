use rdkafka::{
    message::{Header, OwnedHeaders},
    producer::FutureRecord,
};

use crate::{
    clients::AppState,
    models::GenericDataPayload,
    utils::error::AppError,
};

pub async fn handle(
    state: &AppState,
    payload: &GenericDataPayload,
    message_id: &str,
) -> Result<(), AppError> {
    let topic = &state.settings.kafka.batch_topic;
    
    // Create message payload
    let message_payload = serde_json::json!({
        "message_id": message_id,
        "source": payload.source,
        "data": payload.data,
        "timestamp": payload.timestamp,
        "metadata": payload.metadata
    });

    // Create headers
    let headers = OwnedHeaders::new()
        .insert(Header {
            key: "source",
            value: Some(&payload.source),
        })
        .insert(Header {
            key: "message_id",
            value: Some(message_id),
        });

    // Send to Kafka
    let val = &serde_json::to_string(&message_payload)?;
    let record = FutureRecord::to(topic)
        .payload(val)
        .key(&payload.source)
        .headers(headers);

    let timeout = tokio::time::Duration::from_millis(state.settings.kafka.timeout_ms);
    state.kafka_producer.send(record, timeout).await
        .map_err(|(e, _)| AppError::KafkaError(e.to_string()))?;

    tracing::info!(
        topic = %topic,
        message_id = %message_id,
        source = %payload.source,
        "Data sent to Kafka"
    );

    Ok(())
}
