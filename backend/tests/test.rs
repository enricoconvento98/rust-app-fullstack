use clickhouse::Client;
use clickhouse::Row;
use serde::{Serialize, Deserialize};
use anyhow::{Result, Context};
use serial_test::serial;
use pretty_assertions::assert_eq;

#[derive(Debug, Row, Serialize, Deserialize, PartialEq)]
struct MyRow {
    value: u32,
}

// Test setup helper
async fn create_test_client() -> Result<Client> {
    Ok(Client::default()
        .with_url("http://clickhouse:8123")
        .with_user("default")
        .with_password("password")
        .with_database("default"))
}

#[tokio::test]
#[serial] // Ensures tests run sequentially
async fn test_connection_basic() -> Result<()> {
    let client = create_test_client().await?;
    
    let version: String = client.query("SELECT version()")
        .fetch_one()
        .await
        .context("Failed to fetch ClickHouse version")?;
        
    println!("Connected to ClickHouse version: {}", version);
    assert!(!version.is_empty());
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_primitive_type_query() -> Result<()> {
    let client = create_test_client().await?;
    
    let result: u32 = client.query("SELECT toUInt32(42) as value")
        .fetch_one()
        .await
        .context("Failed to fetch result")?;
    
    println!("Result: {}", result);    
    assert_eq!(result, 42);
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_custom_struct_mapping() -> Result<()> {
    let client = create_test_client().await?;
    
    // Test with column alias that matches struct field
    let res: MyRow = client.query("SELECT toUInt32(123) as value")
        .fetch_one()
        .await?;
        
    assert_eq!(res.value, 123);
    
    // Test JSON serialization
    let json = serde_json::to_string(&res)?;
    assert_eq!(json, r#"{"value":123}"#);
    
    Ok(())
}
