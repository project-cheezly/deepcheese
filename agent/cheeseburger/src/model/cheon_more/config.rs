use chrono::NaiveTime;
use serde::Deserialize;
use crate::core::{Account, base_config};

#[derive(Deserialize, Debug, Clone)]
pub(super) struct CheonMoreConfig {
    pub(super) account: Account,
    pub(super) close_time: NaiveTime
}

pub(super) async fn load()
    -> Result<CheonMoreConfig, Box<dyn std::error::Error + Sync + Send>>
{
    base_config::declare_config!("cheon_more", CheonMoreConfig);
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

        assert_eq!(conf.account.number, "12345678901");
        assert_eq!(conf.account.password, "1234");
        assert_eq!(conf.close_time, NaiveTime::from_hms_opt(16, 0, 0).unwrap());

        Ok(())
    }
}