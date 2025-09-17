pub mod redis_client;
pub mod kafka_client;

use std::{sync::Arc, time::SystemTime};
use rdkafka::producer::FutureProducer;

use crate::{
    config::Settings,
    utils::error::AppError,
};

#[derive(Clone)]
pub struct AppState {
    pub settings: Settings,
    pub redis_client: redis::Client,
    pub kafka_producer: Arc<FutureProducer>,
    pub startup_time: SystemTime,
}

impl AppState {
    pub async fn new(settings: &Settings) -> Result<Self, AppError> {
        let redis_client = redis_client::redis_client(settings).await?;
        let kafka_producer = kafka_client::kafka_producer(settings).await?;

        Ok(Self {
            settings: settings.clone(),
            redis_client,
            kafka_producer: Arc::new(kafka_producer),
            startup_time: SystemTime::now(),
        })
    }
}
