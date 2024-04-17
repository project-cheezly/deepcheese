use serde::Deserialize;

use crate::core::base_config;

#[derive(Deserialize, Debug, Clone)]
pub struct ClientConfig {
    pub host: String,
}

pub fn load() -> Result<ClientConfig, Box<dyn std::error::Error + Sync + Send>> {
    base_config::declare_config!("client", ClientConfig);
    base_config::load_mac!()
}

#[cfg(test)]
mod tests {
    use std::env;
    use super::*;

    #[test]
    fn test_load() {
        let config = load().unwrap();

        assert_eq!(config.host, "localhost:5000");
    }
}