use axum::{
    routing::get,
    Router,
    response::Json,
};
use serde::Serialize;
use tokio::net::TcpListener;

#[derive(Serialize)]
struct HealthCheck {
    status: String,
    message: String,
}

async fn health_check() -> Json<HealthCheck> {
    Json(HealthCheck {
        status: "ok".to_string(),
        message: "Server is running".to_string(),
    })
}

#[tokio::main]
async fn main() {
    // Build our application with a single route
    let app = Router::new()
        .route("/health", get(health_check));

    // Create a TCP listener
    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    println!("Server running on http://localhost:3000");
    
    // Serve the application
    axum::serve(listener, app)
        .await
        .unwrap();
}