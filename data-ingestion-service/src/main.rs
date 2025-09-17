use data_ingestion_service::{
    config::Settings,
    handlers::create_router,
    clients::AppState,
    middleware::logging::init_tracing,
};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize configuration
    let settings = Settings::new()?;
    
    // Initialize tracing
    init_tracing(&settings);

    // Create application state
    let state = AppState::new(&settings).await?;

    // Build application router
    let app = create_router()
        .layer(CorsLayer::permissive())
        .with_state(state);

    // Start server
    let bind_address = format!("{}:{}", settings.server.host, settings.server.port);
    let listener = tokio::net::TcpListener::bind(&bind_address).await?;
    
    tracing::info!("Data ingestion service listening on {}", bind_address);

    axum::serve(listener, app).await?;

    Ok(())
}