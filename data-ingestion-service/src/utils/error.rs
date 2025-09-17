use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Redis error: {0}")]
    RedisError(#[from] redis::RedisError),
    
    #[error("Kafka error: {0}")]
    KafkaError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Configuration error: {0}")]
    ConfigError(#[from] config::ConfigError),
    
    #[error("System time error: {0}")]
    SystemTimeError(#[from] std::time::SystemTimeError),
    
    #[error("Unauthorized: {0}")]
    Unauthorized(String),  // Add this
    
    #[error("Internal server error: {0}")]
    InternalError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::RedisError(e) => (StatusCode::SERVICE_UNAVAILABLE, e.to_string()),
            AppError::KafkaError(e) => (StatusCode::SERVICE_UNAVAILABLE, e),
            AppError::SerializationError(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            AppError::ConfigError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::SystemTimeError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::Unauthorized(e) => (StatusCode::UNAUTHORIZED, e),  // Add this
            AppError::InternalError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e),
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16()
        }));

        tracing::error!("Application error: {} - {}", status, error_message);
        (status, body).into_response()
    }
}
