use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct IngestResponse {
    pub status: String,
    pub processing_type: String,
    pub message_id: String,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub version: String,
    pub uptime: u64,
}