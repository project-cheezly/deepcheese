mod update_asset_price;
mod update_category_history;
mod update_currency_value;
mod config;
mod time_machine;
mod database;

use futures::TryFutureExt;
use kis::KIS;
use sqlx::postgres::PgListener;
use tokio::time::sleep;
use crate::config::{DomesticMarket, OverseasMarket};
use crate::database::connect_to_database;
use crate::time_machine::machine::Machine;

use crate::update_category_history::update_category_history;
use crate::update_currency_value::update_currency_value;
use crate::update_asset_price::update_current_stock_price;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = connect_to_database().await.expect("Failed to connect to database");

    let kis = KIS::new().await?;

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    let mut listener = PgListener::connect_with(&pool)
        .await?;

    listener.listen("tr_record_update").await?;

    tokio::spawn(async move {
        while let Ok(x) = listener.recv().await {
            let target = x.payload().parse::<i32>();

            if let Ok(target) = target {
                tx.send(target).expect("Failed to send message");
            }
        }
    });

    tokio::spawn(async move {
        let kis = KIS::new().await.expect("Failed to create KIS instance");
        let pool = connect_to_database().await.expect("Failed to connect to database");

        while let Some(x) = rx.recv().await {
            let mut machine = Machine::new(&kis, &pool).await;
            let _ = machine.run_in_place(x).await;
        }
    });

    loop {
        let current_time = chrono::offset::Utc::now();

        if DomesticMarket::is_open(current_time) || OverseasMarket::is_open(current_time) {
            let res = update_current_stock_price(&pool, &kis).await;
            dbg!(&res);
        }

        if DomesticMarket::is_open(current_time) {
            let _ = update_currency_value(&pool, &kis).await;
        }

        let res = update_category_history(&pool).await;
        dbg!(&res);

        sleep(std::time::Duration::from_secs(60)).await;
    }
}

