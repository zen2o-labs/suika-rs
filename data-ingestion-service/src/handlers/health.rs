use axum::{
    extract::State,
    response::Json,
    routing::get,
    Router,
};
use std::time::SystemTime;

use crate::{
    clients::AppState,
    models::HealthResponse,
    utils::error::AppError,
};

pub fn routes() -> Router<AppState> {
    Router::new().route("/health", get(health_check))
}

pub async fn health_check(
    State(state): State<AppState>,
) -> Result<Json<HealthResponse>, AppError> {
    // Basic health checks
    let _redis_conn = state.redis_client.get_multiplexed_async_connection().await?;
    
    let uptime = SystemTime::now()
        .duration_since(state.startup_time)?
        .as_secs();

    Ok(Json(HealthResponse {
        status: "healthy".to_string(),
        service: "data-ingestion-service".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime,
    }))
}
