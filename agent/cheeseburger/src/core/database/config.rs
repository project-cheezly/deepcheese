use std::fmt::Display;
use serde::Deserialize;
use tracing::error;
use crate::core::base_config;

#[derive(Deserialize, Debug, Clone)]
pub(super) struct DatabaseConfig {
    pub(super) host: String,
    pub(super) user: String,
    pub(super) password: String,
    pub(super) port: u16,
    pub(super) name: String
}

impl Display for DatabaseConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "postgresql://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.name)
    }
}

pub(super) async fn load()
    -> Result<DatabaseConfig, Box<dyn std::error::Error + Sync + Send>>
{
    base_config::declare_config!("database", DatabaseConfig);
    base_config::load_mac!()
        .or_else(|e| {
            error!("Failed to load config: {}", e);
            Err(e)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_load() {
        let config = load().await.unwrap();
        assert_eq!(config.host, "localhost");
        assert_eq!(config.user, "postgres");
        assert_eq!(config.password, "");
        assert_eq!(config.port, 5432);
        assert_eq!(config.name, "");
    }

    #[tokio::test]
    async fn test_display() {
        let config = load().await.unwrap();
        assert_eq!(config.to_string(), "postgresql://postgres:@localhost:5432/");
    }
}