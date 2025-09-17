use redis::AsyncCommands;

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
    let mut conn = state.redis_client.get_multiplexed_async_connection().await?;
    
    let stream_key = format!("realtime:{}", payload.source);
    
    // Prepare stream data
    let data = serde_json::to_string(&payload.data)?;
    let timestamp = &payload.timestamp.to_rfc3339();
    let metadata = &serde_json::to_string(&payload.metadata)?;
    let stream_data = vec![
        ("message_id", message_id),
        ("data", &data),
        ("timestamp", &timestamp),
        ("metadata", &metadata),
    ];

    // Add to Redis stream with max length limit
    let _: String = conn.xadd_maxlen(
        &stream_key,
        redis::streams::StreamMaxlen::Approx(state.settings.redis.stream_max_len),
        "*",
        &stream_data,
    ).await?;
    
    tracing::info!(
        stream_key = %stream_key,
        message_id = %message_id,
        "Data added to Redis stream"
    );

    Ok(())
}
