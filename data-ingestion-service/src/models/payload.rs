use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GenericDataPayload {
    #[serde(rename = "isRealTime")]
    pub is_real_time: bool,
    pub source: String,
    pub data: serde_json::Value,
    #[serde(default = "Utc::now")]
    pub timestamp: DateTime<Utc>,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone)]
pub enum ProcessingType {
    RealTime,
    Batch,
}

impl std::fmt::Display for ProcessingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessingType::RealTime => write!(f, "realtime"),
            ProcessingType::Batch => write!(f, "batch"),
        }
    }
}