use std::sync::Arc;
use chrono::Duration;
use sqlx::Postgres;
use tokio::sync::Mutex;
use tracing::{info, error};
use crate::core::database::get_pool;
use crate::core::future::stream::StreamManager;
use crate::model::cheon_more::start_cheon_more_service;
use crate::model::{start_cheeseburger_service, start_collector_service, start_recorder_service};

mod error;
mod core;
mod model;
pub mod client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    let _ = core::log::init_log().await;
    
    let stream_manager = Arc::new(Mutex::new(StreamManager::new()));

    let postgres = get_pool::<Postgres>().await;
    if let Ok(pool) = postgres {
        info!("Connected to database");

        let pool = Arc::new(pool);
        if let Err(e) = start_recorder_service(pool).await {
            error!("Error starting recorder service: {}", e);
        }
    } else {
        error!("Failed to connect to database");
    }

    if let Err(e) = start_cheon_more_service().await {
        error!("Error starting cheon more service: {}", e);
    };

    if let Err(e) = start_collector_service(stream_manager.clone()).await {
        error!("Error starting collector service: {}", e);
    };

    if let Err(e) = start_cheeseburger_service(stream_manager.clone()).await {
        error!("Error starting cheeseburger service: {}", e);
    };

    tokio::select! {
        _ = tokio::time::sleep(Duration::max_value().to_std().unwrap()) => {},
        _ = tokio::signal::ctrl_c() => {}
    }

    Ok(())
}
