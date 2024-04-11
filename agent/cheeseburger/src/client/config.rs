use config::Config;
use serde::Deserialize;

use crate::error::CheeseburgerError;

#[derive(Deserialize, Debug, Clone)]
pub struct ClientConfig {
    pub host: String,
}

pub fn load() -> Result<ClientConfig, Box<dyn std::error::Error>> {
    Config::builder()
        .add_source(config::Environment::with_prefix("CHEESEBURGER"))
        .build()
        .and_then(|conf| conf.try_deserialize::<ClientConfig>())
        .or(Err(CheeseburgerError::ConfigLoadError.into()))
}

#[cfg(test)]
mod tests {
    use std::env;
    use super::*;

    #[test]
    fn test_load() {
        env::set_var("CHEESEBURGER_HOST", "http://localhost:8080");
        let config = load().unwrap();

        assert_eq!(config.host, "http://localhost:8080");
    }
}