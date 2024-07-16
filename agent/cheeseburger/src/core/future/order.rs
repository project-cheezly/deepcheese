use tonic::transport::Channel;
use crate::client::cheese_api::cheese_api_client::CheeseApiClient;
use crate::client::cheese_api::TradeFutureOptionRequest;
use crate::core::Account;
use crate::core::future::TradeType;

pub async fn sell_market_order(
    client: &mut CheeseApiClient<Channel>,
    account: &Account,
    code: &str,
    amount: i32,
)
    -> Result<(), Box<dyn std::error::Error + Sync + Send>>
{
    let order_detail = Order {
        account_number: account.number.to_string(),
        password: account.password.to_string(),
        code: code.to_string(),
        price: 0,
        amount,
        trade_type: TradeType::Ask,
        trade_condition: TradeCondition::NORMAL,
        order_class: OrderClassification::MARKET,
    };

    order(client, order_detail).await
}

pub async fn buy_market_order(
    client: &mut CheeseApiClient<Channel>,
    account: &Account,
    code: &str,
    amount: i32
) -> Result<(), Box<dyn std::error::Error + Sync + Send>>
{
    order(client,Order {
        account_number: account.number.to_string(),
        password: account.password.to_string(),
        code: code.to_string(),
        price: 0,
        trade_type: TradeType::Bid,
        amount,
        trade_condition: TradeCondition::NORMAL,
        order_class: OrderClassification::MARKET,
    }).await
}

async fn order(
    client: &mut CheeseApiClient<Channel>,
    detail: Order
) -> Result<(), Box<dyn std::error::Error + Sync + Send>>
{
    let mut wait_time = 1;
    loop {
        let detail = detail.clone();
        let res = client.trade_future_option(
            TradeFutureOptionRequest {
                account_number: detail.account_number,
                password: detail.password,
                stock_code: detail.code,
                transaction_amount: detail.amount,
                price: detail.price,
                trade_condition: detail.trade_condition as i32,
                trade_classification: detail.trade_type as i32,
                order_classification: detail.order_class as i32,
                arbitrage: None,
                modify_amount: None,
                original_order_number: None,
                reservation_order: None,
            }).await;

        if let Ok(_) = res {
            return Ok(());
        } else {
            let wait_second = 2_u64.pow(wait_time);

            log::warn!("Failed to trade future option. Retry after {} seconds", wait_second);
            tokio::time::sleep(std::time::Duration::from_secs(wait_second)).await;
            wait_time += 1;

            if wait_time > 5 {
                return Err("Failed to trade future option".into());
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum TradeCondition {
    NORMAL = 0,
    IOC = 1,
    FOK = 2,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum OrderClassification {
    LIMIT = 0,
    MARKET = 1,
    CONDITIONAL = 2,
    BEST = 3,
}

#[derive(Debug, Clone)]
pub struct Order {
    account_number: String,
    password: String,
    code: String,
    price: i32,
    amount: i32,
    trade_type: TradeType,
    trade_condition: TradeCondition,
    order_class: OrderClassification,
}