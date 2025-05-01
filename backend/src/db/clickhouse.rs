use anyhow::Result;
use clickhouse::Client;
use std::sync::Arc;
use tokio::sync::OnceCell;
use serde::{Serialize, Deserialize};
use serde_json::Value;

/// Global instance of the ClickHouse client
static CLICKHOUSE_CLIENT: OnceCell<Arc<ClickhouseClient>> = OnceCell::const_new();

/// Configuration for the ClickHouse client
#[derive(Clone, Debug)]
pub struct ClickhouseConfig {
    pub url: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub database: Option<String>,
}

impl Default for ClickhouseConfig {
    fn default() -> Self {
        Self {
            url: "http://clickhouse:8123".to_string(),
            username: Some("default".to_string()),
            password: Some("password".to_string()),
            database: Some("default".to_string()),
        }
    }
}

/// ClickHouse client wrapper
#[derive(Clone)]
pub struct ClickhouseClient {
    client: Client,
}

impl ClickhouseClient {
    pub fn new(config: ClickhouseConfig) -> Result<Self> {
        let mut client = Client::default().with_url(&config.url);
        if let Some(username) = config.username {
            client = client.with_user(username);
        }
        if let Some(password) = config.password {
            client = client.with_password(password);
        }
        if let Some(database) = config.database {
            client = client.with_database(database);
        }
    
        Ok(Self { client })
    }

    pub async fn global() -> Result<Arc<Self>> {
        CLICKHOUSE_CLIENT
            .get_or_try_init(|| async {
                let config = ClickhouseConfig::default();
                Ok(Arc::new(Self::new(config)?))
            })
            .await
            .cloned()
    }

    pub fn inner(&self) -> &Client {
        &self.client
    }

    pub async fn ping(&self) -> Result<String> {
        let version: String = self.client.query("SELECT version()").fetch_one().await?;
        Ok(version)
    }

    /// Query with strongly-typed results using serde
    pub async fn query_typed<T>(&self, query: &str) -> Result<Vec<T>>
    where
        T: for<'a> Deserialize<'a> + clickhouse::Row,
    {
        self.client
            .query(query)
            .fetch_all()
            .await
            .map_err(|e| anyhow::anyhow!("Query failed: {}", e))
    }

    /// Query with JSON values
    pub async fn query_json(&self, query: &str) -> Result<Vec<Value>> {
        // First fetch as strings and then parse to JSON
        let rows: Vec<String> = self.client
            .query(query)
            .fetch_all()
            .await
            .map_err(|e| anyhow::anyhow!("Query failed: {}", e))?;
        
        // Convert each row to a JSON value
        let json_rows = rows.into_iter()
            .map(|row| serde_json::from_str(&row))
            .collect::<std::result::Result<Vec<Value>, _>>()
            .map_err(|e| anyhow::anyhow!("JSON parsing failed: {}", e))?;
            
        Ok(json_rows)
    }

   
}