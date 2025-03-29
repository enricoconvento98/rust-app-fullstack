use backend::app;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use std::sync::Arc;
use tokio::sync::Barrier;
use tower::ServiceExt;

#[tokio::test(flavor = "multi_thread")]
async fn test_concurrent_requests() {
    let app = app();
    let num_requests = 10;  // Reduced for testing
    let barrier = Arc::new(Barrier::new(num_requests));

    let mut handles = vec![];

    for _ in 0..num_requests {
        let app = app.clone();
        let barrier = barrier.clone();
        
        handles.push(tokio::spawn(async move {
            barrier.wait().await;
            
            let response = app
                .oneshot(Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap())
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::OK);
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

#[tokio::test]
async fn test_single_request() {
    let app = app();
    
    let response = app
        .oneshot(Request::builder()
            .uri("/health")
            .body(Body::empty())
            .unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}