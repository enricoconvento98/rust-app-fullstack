use backend::app;  // Only import what's needed
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() {
    // Initialize tracing
    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info"))
        )
        .init();

    // Build application using shared app() function
    let app = app()
        // Add additional layers specific to main application
        .layer(TraceLayer::new_for_http());

    // Configure bind address
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind to port 3000");
    
    tracing::info!("Server running on {}", addr);
    
    axum::serve(listener, app)
        .await
        .expect("Server failed");
}