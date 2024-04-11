use chrono::Duration;
use crate::core::Account;
use crate::model::cheon_more::start_cheon_more_service;

mod error;
mod client;
mod core;
mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let account = Account::load_from_env("CHEESEBURGER")?;

    if let Err(e) = start_cheon_more_service(account).await {
        log::error!("Error starting cheon more service: {}", e);
    };

    tokio::select! {
        _ = tokio::time::sleep(Duration::max_value().to_std().unwrap()) => {},
        _ = tokio::signal::ctrl_c() => {}
    }

    Ok(())
}