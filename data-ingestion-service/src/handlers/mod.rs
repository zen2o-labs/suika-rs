pub mod ingestion;
pub mod health;

use axum::Router;
use crate::clients::AppState;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .merge(ingestion::routes())
        .merge(health::routes())
}

pub fn create_router_with_auth() -> Router<AppState> {
    Router::new()
        .merge(ingestion::routes())
        .merge(health::routes())
}

