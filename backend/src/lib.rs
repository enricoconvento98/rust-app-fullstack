use axum::{
    Router,
    routing::get,
    response::Json,
    body::Body,
    http::Request,
    middleware::{self, Next}
};
use serde::Serialize;
use std::time::Instant;

#[derive(Serialize)]
pub struct HealthCheck {
    status: String,
    message: String,
}

pub async fn health_check() -> Json<HealthCheck> {
    Json(HealthCheck {
        status: "ok".to_string(),
        message: "Server is running".to_string(),
    })
}

pub async fn log_request(
    req: Request<Body>,
    next: Next,
) -> impl axum::response::IntoResponse {
    let start = Instant::now();
    let method = req.method().clone();
    let uri = req.uri().clone();
    
    let response = next.run(req).await;
    
    let duration = start.elapsed();
    println!(
        "{} {} - {}ms - Status: {}",
        method,
        uri,
        duration.as_millis(),
        response.status()
    );
    
    response
}

pub fn app() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .layer(middleware::from_fn(log_request))
}