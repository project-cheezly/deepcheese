pub mod cheese_api {
    tonic::include_proto!("cheese_api");
}

mod config;

use tonic::transport::Channel;
use cheese_api::cheese_api_client::CheeseApiClient;
use crate::error::CheeseburgerError;

pub async fn new() -> Result<CheeseApiClient<Channel>, Box<dyn std::error::Error>> {
    CheeseApiClient::connect(config::load()?.host)
        .await
        .or_else(|e| Err(CheeseburgerError::ConnectionError(e.to_string()).into()))
}

#[cfg(test)]
mod tests {
    use std::env;
    use super::*;

    #[tokio::test]
    async fn test_new() {
        env::set_var("CHEESEBURGER_HOST", "http://localhost:8080");
        let client = new().await;

        assert!(client.is_err());
    }
}