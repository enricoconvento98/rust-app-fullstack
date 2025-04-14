use clickhouse::Client;
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
    Client::default()
        .with_url("http://clickhouse:8123")
        .with_user("default")
        .with_password("password")
        .with_database("default")
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
    
    let result: u32 = client.query("SELECT 42")
        .fetch_one()
        .await?;
        
    assert_eq!(result, 42);
    Ok(())
}

// #[tokio::test]
// #[serial]
// async fn test_custom_struct_mapping() -> Result<()> {
//     let client = create_test_client().await?;
    
//     // Test with column alias that matches struct field
//     let res: MyRow = client.query("SELECT 123 as value")
//         .fetch_one()
//         .await?;
        
//     assert_eq!(res.value, 123);
    
//     // Test JSON serialization
//     let json = serde_json::to_string(&res)?;
//     assert_eq!(json, r#"{"value":123}"#);
    
//     Ok(())
// }

// #[tokio::test]
// #[serial]
// async fn test_multiple_rows() -> Result<()> {
//     let client = create_test_client().await?;
    
//     // Setup test data
//     client.query("CREATE TEMPORARY TABLE IF NOT EXISTS test_data (value UInt32) ENGINE = Memory")
//         .execute()
//         .await?;
        
//     client.query("INSERT INTO test_data VALUES (1), (2), (3)")
//         .execute()
//         .await?;
    
//     // Fetch and verify
//     let rows: Vec<MyRow> = client.query("SELECT value FROM test_data ORDER BY value")
//         .fetch_all()
//         .await?;
        
//     assert_eq!(rows.len(), 3);
//     assert_eq!(rows[0].value, 1);
//     assert_eq!(rows[1].value, 2);
//     assert_eq!(rows[2].value, 3);
    
//     Ok(())
// }

// #[tokio::test]
// #[serial]
// async fn test_error_handling() {
//     let client = create_test_client().await.unwrap();
    
//     // Test invalid query
//     let result: Result<MyRow, _> = client.query("SELECT invalid_column")
//         .fetch_one()
//         .await;
        
//     assert!(result.is_err());
    
//     // Test type mismatch
//     let result: Result<u32, _> = client.query("SELECT 'not_a_number'")
//         .fetch_one()
//         .await;
        
//     assert!(result.is_err());
// }