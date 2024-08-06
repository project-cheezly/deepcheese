use serde::Deserialize;
use crate::core::base_config;

#[derive(Deserialize, Debug, Clone)]
pub(super) struct LogConfig {
    pub(super) host: String,
}

pub(super)async fn load()
    -> Result<LogConfig, Box<dyn std::error::Error + Sync + Send>>
{
    base_config::declare_config!("log", LogConfig);
    base_config::load_mac!()
        .or_else(|e| {
            tracing::error!("Failed to load config: {}", e);
            Err(e)
        })
}
