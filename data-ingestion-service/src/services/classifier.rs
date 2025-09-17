use std::collections::HashSet;
use chrono::Utc;

use crate::{
    config::Settings,
    models::{GenericDataPayload, ProcessingType},
};

pub struct DataClassifier {
    realtime_sources: HashSet<String>,
    realtime_threshold_secs: i64,
}

impl DataClassifier {
    pub fn new(settings: &Settings) -> Self {
        let mut realtime_sources = HashSet::new();
        
        // Railway-specific real-time sources
        realtime_sources.insert("train_positions".to_string());
        realtime_sources.insert("sensor_readings".to_string());
        realtime_sources.insert("live_signals".to_string());
        realtime_sources.insert("gps_tracker".to_string());
        realtime_sources.insert("conflict_detection".to_string());
        realtime_sources.insert("emergency_system".to_string());

        Self {
            realtime_sources,
            realtime_threshold_secs: settings.processing.realtime_threshold_secs,
        }
    }

    pub fn classify(&self, payload: &GenericDataPayload) -> ProcessingType {
        // Explicit real-time flag takes precedence
        if payload.is_real_time {
            return ProcessingType::RealTime;
        }

        // Check source-based rules
        if self.realtime_sources.contains(&payload.source) {
            return ProcessingType::RealTime;
        }

        // Check metadata-based rules
        if let Some(urgency) = payload.metadata.get("urgency") {
            if let Some(urgency_str) = urgency.as_str() {
                if urgency_str == "high" || urgency_str == "critical" {
                    return ProcessingType::RealTime;
                }
            }
        }

        // Check time sensitivity
        let now = Utc::now();
        let time_diff = now.signed_duration_since(payload.timestamp);
        if time_diff.num_seconds().abs() < self.realtime_threshold_secs {
            return ProcessingType::RealTime;
        }

        ProcessingType::Batch
    }
}
