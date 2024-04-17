use chrono::NaiveTime;
use serde::Deserialize;

use crate::core::{Account, base_config};

#[derive(Deserialize, Debug, Clone)]
pub(super) struct RecorderConfig {
    pub(super) close_time: NaiveTime,
    pub(super) category_id: i32,
    pub(super) asset_id: i32,
    pub(super) account_id: i32,
    pub(super) account: Account
}

pub(super) async fn load()
    -> Result<RecorderConfig, Box<dyn std::error::Error + Sync + Send>>
{
    base_config::declare_config!("recorder", RecorderConfig);
    base_config::load_mac!()
        .or_else(|e| {
            log::error!("Failed to load config: {}", e);
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

        assert_eq!(conf.close_time, NaiveTime::from_hms_opt(15, 50, 0).unwrap());

        Ok(())
    }
}