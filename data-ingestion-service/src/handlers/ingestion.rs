use axum::{
    extract::State,
    response::Json,
    routing::post,
    Router,
};
use uuid::Uuid;

use crate::{
    clients::AppState,
    models::{GenericDataPayload, IngestResponse, ProcessingType},
    services::{classifier::DataClassifier, realtime, batch},
    utils::error::AppError,
};

pub fn routes() -> Router<AppState> {
    Router::new().route("/ingest", post(ingest_data))
}

pub async fn ingest_data(
    State(state): State<AppState>,
    Json(payload): Json<GenericDataPayload>,
) -> Result<Json<IngestResponse>, AppError> {
    let message_id = Uuid::new_v4().to_string();
    
    tracing::info!(
        message_id = %message_id,
        source = %payload.source,
        is_real_time = payload.is_real_time,
        "Processing data ingestion"
    );

    // Classify the data
    let classifier = DataClassifier::new(&state.settings);
    let processing_type = classifier.classify(&payload);

    // Process based on type
    match processing_type {
        ProcessingType::RealTime => {
            realtime::handle(&state, &payload, &message_id).await?;
        }
        ProcessingType::Batch => {
            batch::handle(&state, &payload, &message_id).await?;
        }
    }

    Ok(Json(IngestResponse {
        status: "accepted".to_string(),
        processing_type: processing_type.to_string(),
        message_id,
    }))
}
