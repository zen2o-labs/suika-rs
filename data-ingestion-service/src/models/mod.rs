pub mod payload;
pub mod response;

pub use payload::{GenericDataPayload, ProcessingType};
pub use response::{IngestResponse, HealthResponse};