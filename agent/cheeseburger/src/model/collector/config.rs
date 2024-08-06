use std::ops::Deref;
use chrono::NaiveTime;
use serde::Deserialize;
use crate::core::{
    base_config,
    indi::QueryCode
};

#[derive(Deserialize, Debug, Clone)]
pub(super) struct CollectorConfig {
    pub(super) end_time: NaiveTime,
    pub(super) target: Vec<Target>
}

impl Deref for CollectorConfig {
    type Target = Vec<Target>;

    fn deref(&self) -> &Self::Target {
        &self.target
    }
}

#[derive(Deserialize, Debug, Clone)]
pub(super) struct Target {
    pub(super) query_code: QueryCode,
    pub(super) stock_code: String,
}

pub (super) async fn load()
    -> Result<CollectorConfig, Box<dyn std::error::Error + Sync + Send>>
{
    base_config::declare_config!("collector", CollectorConfig);
    base_config::load_mac!()
        .or_else(|e| {
            tracing::error!("Failed to load config: {}", e);
            Err(e)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_load()
        -> Result<(), Box<dyn std::error::Error + Sync + Send>>
    {
        let conf = load().await?;

        assert_eq!(conf[0].query_code, QueryCode::FutureOptionLimitOrderBook);
        assert_eq!(conf[0].stock_code, "106V6".to_string());

        assert_eq!(conf[1].query_code, QueryCode::FutureOptionCurrentPrice);
        assert_eq!(conf[1].stock_code, "106V6".to_string());

        Ok(())
    }
}