use axum::{
    extract::{Request, State},
    http::HeaderMap,
    middleware::Next,
    response::Response,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

use crate::{clients::AppState, utils::error::AppError};

pub async fn api_key_auth(
    State(state): State<AppState>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // Skip auth if disabled in config
    if !state.settings.auth.require_auth {
        return Ok(next.run(request).await);
    }

    // Check for API key in Authorization header
    let auth_header = headers
        .get("authorization")
        .and_then(|value| value.to_str().ok());

    if let Some(auth_value) = auth_header {
        // Support both "Bearer <key>" and just "<key>" formats
        let provided_key = if auth_value.starts_with("Bearer ") {
            auth_value.strip_prefix("Bearer ").unwrap_or("")
        } else if auth_value.starts_with("ApiKey ") {
            auth_value.strip_prefix("ApiKey ").unwrap_or("")
        } else {
            auth_value
        };

        if provided_key == state.settings.auth.api_key {
            tracing::info!("API key authentication successful");
            return Ok(next.run(request).await);
        }
    }

    // Check for API key in X-API-Key header (alternative)
    if let Some(api_key_header) = headers.get("x-api-key") {
        if let Ok(provided_key) = api_key_header.to_str() {
            if provided_key == state.settings.auth.api_key {
                tracing::info!("API key authentication successful (X-API-Key header)");
                return Ok(next.run(request).await);
            }
        }
    }

    tracing::warn!("API key authentication failed");
    
    Err(AppError::Unauthorized("Invalid or missing API key".to_string()))
}

// Optional: Typed header extractor for cleaner code
pub async fn api_key_auth_typed(
    State(state): State<AppState>,
    auth: Option<TypedHeader<Authorization<Bearer>>>,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    if !state.settings.auth.require_auth {
        return Ok(next.run(request).await);
    }

    if let Some(TypedHeader(Authorization(bearer))) = auth {
        if bearer.token() == state.settings.auth.api_key {
            return Ok(next.run(request).await);
        }
    }

    Err(AppError::Unauthorized("Invalid or missing API key".to_string()))
}
