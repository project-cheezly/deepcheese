pub mod cheese_api {
    tonic::include_proto!("cheese_api");
}

pub mod config;

use tonic::transport::Channel;
use cheese_api::cheese_api_client::CheeseApiClient;
use crate::error::CheeseburgerError;

pub async fn new() -> Result<CheeseApiClient<Channel>, Box<dyn std::error::Error + Sync + Send>> {
    CheeseApiClient::connect(config::load()?.host)
        .await
        .or_else(|e| {
            log::error!("Failed to connect to cheeseburger: {}", e.to_string());
            Err(CheeseburgerError::ConnectionError(e.to_string()).into())
        })
}

#[cfg(test)]
mod tests {
    use std::env;
    use super::*;

    #[tokio::test]
    async fn test_new() {
        let client = new().await;

        assert!(client.is_err());
    }
}