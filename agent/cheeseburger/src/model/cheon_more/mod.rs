mod config;

use std::fmt::Display;
use chrono::{Datelike, Local};
use tokio::time;
use tonic::transport::Channel;
use crate::client;
use crate::client::cheese_api::{
    cheese_api_client::CheeseApiClient,
    AccountDepositInfoRequest,
    FutureOptionDepositRequest,
    TransferDepositRequest
};

use crate::core::account::Account;
use crate::core::duration;

pub async fn start_cheon_more_service()
    -> Result<(), Box<dyn std::error::Error + Send + Sync>>
{
    let config = config::load().await?;

    if let Err(e) = validate() {
        log::error!("Failed to execute cheon_more: {}", e);
        return Err(e);
    }

    tokio::spawn(async move {
        let account = config.account;

        match transfer_deposit_to_futures(&account).await {
            Ok(deposit) => log::info!("Moved deposit to futures: {}", deposit),
            Err(e) => log::warn!("Failed to move deposit to futures: {}", e)
        }

        let duration_to_finish = duration::get_duration(config.close_time);

        time::sleep(duration_to_finish).await;

        match transfer_deposit_to_stock(&account).await {
            Ok(deposit) => log::info!("Moved deposit to stock: {}", deposit),
            Err(e) => log::warn!("Failed to move deposit to stock: {}", e)
        }

        log::info!("cheon_more service finished");
    });

    Ok(())
}

async fn transfer_deposit_to_stock(account: &Account)
    -> Result<i32, Box<dyn std::error::Error + Sync + Send>>
{
    let mut client = client::new().await?;

    let deposit = client
        .lookup_future_option_deposit(FutureOptionDepositRequest {
            account_number: account.number.to_string(),
            password: account.password.to_string(),
        }).await?
        .into_inner()
        .withdrawal_amount;

    transfer_deposit(&mut client, account, 10, 1, deposit).await?;
    Ok(deposit)
}

fn validate() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    is_weekend()
}

fn is_weekend() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let now = Local::now();
    if now.weekday() == chrono::Weekday::Sat
        || now.weekday() == chrono::Weekday::Sun
    {
        Err(Box::new(CheonMoreError::WeekendError))
    } else { Ok(()) }
}


async fn transfer_deposit_to_futures(account: &Account)
    -> Result<i32, Box<dyn std::error::Error + Sync + Send>>
{
    let mut client = client::new().await?;

    let deposit = client
        .lookup_account_deposit_info(AccountDepositInfoRequest {
            account_number: account.number.to_string(),
            password: account.password.to_string(),
        }).await?
        .into_inner()
        .withdrawal_amount;

    transfer_deposit(&mut client, account,1, 10, deposit).await?;
    Ok(deposit)
}

async fn transfer_deposit(
    client: &mut CheeseApiClient<Channel>,
    account: &Account,
    withdrawal_item: i32,
    deposit_item: i32,
    deposit: i32
) -> Result<(), Box<dyn std::error::Error + Sync + Send>>
{
    if deposit == 0 {
        return Ok(());
    }

    client.transfer_deposit(TransferDepositRequest {
        account_number: account.number.to_string(),
        withdrawal_item_number: format!("{:02}", withdrawal_item),
        password: account.password.to_string(),
        deposit_item_number: format!("{:02}", deposit_item),
        transfer_amount: deposit,
    }).await?;

    Ok(())
}

#[derive(Debug, Clone)]
pub enum CheonMoreError {
    WeekendError
}

impl Display for CheonMoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CheonMoreError::WeekendError => write!(f, "It's weekend")
        }
    }
}

impl std::error::Error for CheonMoreError {}
