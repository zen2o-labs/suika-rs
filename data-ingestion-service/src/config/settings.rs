use config::{Config, ConfigError, Environment};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Settings {
    pub server: ServerConfig,
    pub redis: RedisConfig,
    pub kafka: KafkaConfig,
    pub processing: ProcessingConfig,
    pub auth: AuthConfig, // Add this
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RedisConfig {
    pub url: String,
    pub stream_max_len: usize,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KafkaConfig {
    pub bootstrap_servers: String,
    pub batch_topic: String,
    pub timeout_ms: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProcessingConfig {
    pub batch_size: usize,
    pub batch_timeout_secs: u64,
    pub realtime_threshold_secs: i64,
}

// New auth configuration
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AuthConfig {
    pub api_key: String,
    pub require_auth: bool,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 8080)?
            .set_default("redis.url", "redis://redis:6379")?
            .set_default("redis.stream_max_len", 10000)?
            .set_default("kafka.bootstrap_servers", "kafka:9092")?
            .set_default("kafka.batch_topic", "batch_processing_topic")?
            .set_default("kafka.timeout_ms", 5000)?
            .set_default("processing.batch_size", 1000)?
            .set_default("processing.batch_timeout_secs", 60)?
            .set_default("processing.realtime_threshold_secs", 5)?
            .set_default("auth.api_key", "railway-system-api-key-2025")? // Default key
            .set_default("auth.require_auth", true)?
            .add_source(
                Environment::with_prefix("APP")
                    .separator("__") // Use double underscore for nested keys
                    .try_parsing(true)
                    .ignore_empty(true),
            )
            .build()?;

        config.try_deserialize()
    }
}
