use std::sync::Arc;
use log::warn;
use tokio::sync::{
    broadcast::Receiver,
    Mutex
};
use tokio::task::JoinHandle;
use tonic::transport::Channel;

use crate::client;
use crate::client::{cheese_api::{
    cheese_api_client::CheeseApiClient,
    FutureCurrentPriceResponse,
    FuturePreviousCandleRequest
}, cheese_api};
use crate::client::cheese_api::FutureOptionDepositRequest;
use crate::core::{
    Account,
    future::{
        order::{buy_market_order, sell_market_order},
        stream::StreamManager
    }
};
use crate::core::future::FutureType;
use crate::error::CheeseburgerError;
use crate::model::cheeseburger::config::CheeseburgerStrategyConfig;


pub(crate) async fn in_market(
    account: &Account,
    config: Vec<CheeseburgerStrategyConfig>,
    stream_manager: Arc<Mutex<StreamManager>>
) -> Result<
    Vec<JoinHandle<Result<(), Box<dyn std::error::Error + Sync + Send>>>>,
    Box<dyn std::error::Error + Send + Sync>
>
{
    let mut handles = Vec::new();

    for strategy in config {
        match strategy.strategy_type.as_ref() {
            "LONG" => {},
            x => {
                warn!("Unknown strategy: {}", x);
                continue;
            }
        }

        let (code, multiplier) = get_future_code_and_multiplier(&strategy.target_code).await?;

        let receiver = stream_manager
            .lock()
            .await
            .get_future_price_receiver(&code)
            .await;

        let handle = tokio::spawn(
            long_strategy(receiver, account.clone(), strategy, multiplier, code)
        );
        handles.push(handle);
    }
    Ok(handles)
}

async fn long_strategy(
    mut receiver: Receiver<FutureCurrentPriceResponse>,
    account: Account,
    config: CheeseburgerStrategyConfig,
    multiplier: i32,
    code: String
) -> Result<(), Box<dyn std::error::Error + Sync + Send>>
{
    let mut client = client::new().await?;
    let value_range = calculate_target_range(&mut client, config.ratio, &code).await?;

    log::info!("Start long strategy: {}, {}", code, value_range);

    let mut amount = 0;

    while let Ok(response) = receiver.recv().await {
        if let Some(candle) = response.candle {
            if candle.open + value_range < candle.close {
                amount = calculate_amount(
                    &mut client,
                    &account,
                    config.bet,
                    candle.close,
                    multiplier
                ).await?;
                let trade_res = buy_market_order(&mut client, &account, &code, amount).await;

                if let Err(e) = trade_res {
                    warn!("Failed to buy: {}", e);
                    return Err(CheeseburgerError::TradeError.into());
                }

                break;
            }
        }
    }

    while let Ok(response) = receiver.recv().await {
        if let Some(candle) = response.candle {
            if candle.open - value_range > candle.close {
                let trade_res = sell_market_order(&mut client, &account, &code, amount).await;

                if let Err(e) = trade_res {
                    warn!("Failed to sell: {}", e);
                    return Err(CheeseburgerError::TradeError.into());
                }

                break;
            }
        }
    }

    Ok(())
}

async fn get_future_code_and_multiplier(future_type: &FutureType)
    -> Result<(String, i32), Box<dyn std::error::Error + Sync + Send>>
{
    let mut client = client::new().await?;

    let mut response = client
        .lookup_futures_info(cheese_api::Empty {})
        .await?
        .into_inner()
        .list
        .into_iter()
        .filter(|x|
            x.base_asset_code == future_type.base_code()
            && x.spread_lead_month_standard_code.is_some()
        )
        .filter_map(|x| match x.spread_back_month_standard_code {
            Some(ref code) if code == " " => Some(x),
            _ => None
        })
        .filter_map(|x| match x.final_trade_date {
            Some(date) => Some((x.abbr_code, convert_to_naive_date(date), x.multiplier)),
            None => None
        })
        .collect::<Vec<_>>();

    response.sort_by(|a, b| a.1.cmp(&b.1));

    if response.len() == 0 {
        Err(CheeseburgerError::ParseError("Can't get future code".to_string()).into())
    } else {
        if response[0].1 + chrono::Duration::days(1) <= chrono::Local::now().date_naive() {
            Ok((response[1].0.clone(), response[1].2))
        } else {
            Ok((response[0].0.clone(), response[0].2))
        }
    }
}

fn convert_to_naive_date(date: cheese_api::Date) -> chrono::NaiveDate
{
    chrono::NaiveDate::from_ymd_opt(date.year, date.month as u32, date.day as u32).unwrap()
}

async fn calculate_amount(
    client: &mut CheeseApiClient<Channel>,
    account: &Account,
    ratio: f64,
    price: i32,
    multiplier: i32
) -> Result<i32, Box<dyn std::error::Error + Sync + Send>>
{
    let deposit = client
        .lookup_future_option_deposit(FutureOptionDepositRequest {
            account_number: account.number.to_string(),
            password: account.password.to_string(),
        }).await?
        .into_inner()
        .tradable_deposit as f64;

    let deposit = (deposit * ratio) as i32;
    let divisor = price * multiplier / 100;
    let amount = deposit / divisor + (deposit % divisor).signum();

    Ok(amount)
}

async fn calculate_target_range(
    client: &mut CheeseApiClient<Channel>,
    ratio: f64,
    code: &str
) -> Result<i32, Box<dyn std::error::Error + Sync + Send>>
{
    let res = client
        .lookup_future_previous_candle(FuturePreviousCandleRequest {
            code: code.to_string(),
            class: "FU".to_string(),
        })
        .await?
        .into_inner();

    Ok(((res.high - res.low) as f64 * ratio) as i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::config;

    #[tokio::test]
    async fn test_calculate_target_range()
        -> Result<(), Box<dyn std::error::Error + Sync + Send>>
    {
        let mut client = client::new().await?;
        let config = config::load()?;
        let code = "106V6";

        let result = calculate_target_range(&mut client, config.strategy[0].ratio, code).await?;
        dbg!(result);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_future_code()
        -> Result<(), Box<dyn std::error::Error + Sync + Send>>
    {
        let future_type = FutureType::try_from("KOSDAQ")?;
        let (code, multiplier) = get_future_code_and_multiplier(&future_type).await?;
        dbg!(code, multiplier);

        Ok(())
    }
}