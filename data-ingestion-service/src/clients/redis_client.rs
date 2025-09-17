use crate::{config::Settings, utils::error::AppError};

pub async fn redis_client(settings: &Settings) -> Result<redis::Client, AppError> {
    let client = redis::Client::open(settings.redis.url.as_str())?;
    
    // Test connection
    let mut conn = client.get_multiplexed_async_connection().await?;
    let _: String = redis::AsyncCommands::ping(&mut conn).await?;
    
    tracing::info!("Redis client initialized successfully");
    Ok(client)
}