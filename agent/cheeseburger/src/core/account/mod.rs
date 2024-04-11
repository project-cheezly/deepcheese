use config::Config;
use serde::Deserialize;

use crate::error::CheeseburgerError;

#[derive(Deserialize, Debug, Clone)]
pub struct Account {
    pub number: String,
    pub password: String,
}

impl Account {
    #[allow(dead_code)]
    pub fn new(number: String, password: String) -> Self {
        Self { number, password }
    }

    pub fn load_from_env(prefix: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Config::builder()
            .add_source(config::Environment::with_prefix(prefix))
            .build()
            .and_then(|conf| conf.try_deserialize::<Account>())
            .or(Err(CheeseburgerError::ConfigLoadError.into()))
    }
}