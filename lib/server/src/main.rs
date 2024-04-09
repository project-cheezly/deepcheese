mod update_asset_price;
mod update_category_history;
mod update_currency_value;
mod config;

use std::env;

use kis::KIS;
use sqlx::postgres::{PgPoolOptions, PgConnectOptions};
use crate::config::{DomesticMarket, OverseasMarket};

use crate::update_category_history::update_category_history;
use crate::update_currency_value::update_currency_value;
use crate::update_asset_price::update_current_stock_price;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connect_options = PgConnectOptions::new()
        .host(&env::var("DB_HOST").unwrap())
        .username(&env::var("DB_USER").unwrap())
        .password(&env::var("DB_PASSWORD").unwrap())
        .port(env::var("DB_PORT").unwrap().parse::<u16>().unwrap())
        .database(&env::var("DB_NAME").unwrap());

    let pool_connect_result = PgPoolOptions::new()
        .max_connections(3)
        .connect_with(connect_options).await?;

    let kis = KIS::new().await?;

    loop {
        let current_time = chrono::offset::Utc::now();

        if DomesticMarket::is_open(current_time) || OverseasMarket::is_open(current_time) {
            let _ = update_current_stock_price(&pool_connect_result, &kis).await;
        }

        if DomesticMarket::is_open(current_time) {
            let _ = update_currency_value(&pool_connect_result, &kis).await;
        }

        let _ = update_category_history(&pool_connect_result).await;
    }
}