use anyhow::Result;
use clickhouse::Client;
use serde::{Deserialize, Serialize};
use clickhouse::Row;

async fn test_client() -> backend::ClickhouseClient {
    backend::ClickhouseClient::new(backend::ClickhouseConfig::default())
        .expect("Failed to create test client")
}

#[tokio::test]
async fn test_ping() {
    let client = test_client().await;
    let version = client.ping().await.expect("Ping failed");
    assert!(!version.is_empty());
    println!("ClickHouse version: {}", version);
}

#[tokio::test]
async fn test_inner() -> Result<()> {
    let client = test_client().await;
    client.inner();
    Ok(())
}


#[tokio::test]
async fn test_query_as_json() -> Result<()> {
    let client = backend::ClickhouseClient::global().await?;
    println!("test 1");
    // Query as JSON
    let json_result = client.query_json("SELECT 1 as value").await?;
    
    // Use debug format for printing the Vec<Value>
    println!("JSON Result:\n{:#?}", json_result.len());
    
    // Or print it as a JSON string
    let json_string = serde_json::to_string_pretty(&json_result)
        .expect("Failed to serialize JSON");
    println!("Pretty JSON:\n{}", json_string);
    
    // Verify the result
    assert!(!json_result.is_empty());
    assert_eq!(json_result[0]["value"], 1);
    
    Ok(())
}