use serde::Deserialize;
use crate::core::{
    Account,
    base_config,
    future::FutureType
};

#[derive(Deserialize, Debug, Clone)]
pub struct CheeseburgerConfig {
    pub account: Account,
    pub strategy: Vec<CheeseburgerStrategyConfig>
}

#[derive(Deserialize, Debug, Clone)]
pub struct CheeseburgerStrategyConfig {
    pub strategy_type: String,
    pub bet: f64,
    pub ratio: f64,
    pub target_code: FutureType,
}

pub fn load() -> Result<CheeseburgerConfig, Box<dyn std::error::Error + Send + Sync>> {
    base_config::declare_config!("cheeseburger", CheeseburgerConfig);
    base_config::load_mac!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_test() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let config = load()?;

        assert_eq!(config.account.number, "12345678901");
        assert_eq!(config.account.password, "1234");

        Ok(())
    }
}