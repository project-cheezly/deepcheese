use std::sync::Arc;
use tokio::sync::Mutex;
use crate::core::future::stream::StreamManager;
use crate::model::cheeseburger::agent::in_market::in_market;
use crate::model::cheeseburger::agent::pre_market::pre_market;
use crate::model::cheeseburger::config;

pub async fn start_cheeseburger_service(stream_manager: Arc<Mutex<StreamManager>>)
    -> Result<(), Box<dyn std::error::Error + Sync + Send>>
{
    let config = config::load()?;
    let pre_market_result = pre_market(&config.account).await;

    if let Err(e) = pre_market_result {
        log::error!("Error in pre_market: {}", e);
    }

    if let Err(e) = in_market(&config.account, config.strategy, stream_manager).await {
        log::error!("Error in in_market: {}", e);
    }

    Ok(())
}