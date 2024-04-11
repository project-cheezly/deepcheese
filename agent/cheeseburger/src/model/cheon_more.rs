use std::fmt::Display;
use chrono::{Datelike, Local, NaiveTime};
use log::{error, info, warn};
use tokio::time;
use tokio::time::Duration;
use tonic::transport::Channel;
use crate::client;
use crate::client::cheese_api::cheese_api_client::CheeseApiClient;
use crate::client::cheese_api::{
    AccountDepositInfoRequest,
    FutureOptionDepositRequest,
    TransferDepositRequest
};

use crate::core::account::Account;

const MOVE_TO_FUTURE_TIME: Option<NaiveTime> = NaiveTime::from_hms_opt(8, 30, 0);
const MOVE_TO_STOCK_TIME: Option<NaiveTime> = NaiveTime::from_hms_opt(15, 50, 0);

pub async fn start_cheon_more_service(account: Account) -> Result<(), Box<dyn std::error::Error>> {
    let client = client::new()
        .await
        .or_else(|e| {
            error!("Failed to create client: {}", e);
            Err(e)
        })?;

    let mut cheon_more = CheonMore::new(client, account);

    if let Err(e) = cheon_more.validate() {
        error!("Failed to execute cheon_more: {}", e);
        return Err(e);
    }

    tokio::spawn(async move {
        let future_duration = get_duration(MOVE_TO_FUTURE_TIME.unwrap());
        let stock_duration = get_duration(MOVE_TO_STOCK_TIME.unwrap());

        time::sleep(future_duration).await;

        match cheon_more.transfer_deposit_to_futures().await {
            Ok(deposit) => info!("Moved deposit to futures: {}", deposit),
            Err(e) => warn!("Failed to move deposit to futures: {}", e)
        }

        time::sleep(stock_duration).await;

        match cheon_more.transfer_deposit_to_stock().await {
            Ok(deposit) => info!("Moved deposit to stock: {}", deposit),
            Err(e) => warn!("Failed to move deposit to stock: {}", e)
        }

        info!("cheon_more service finished");
    });

    Ok(())
}

fn get_duration(time: NaiveTime) -> Duration {
    let now = Local::now();
    let target = now.with_time(time).unwrap();

    (target - now).to_std().unwrap_or_else(|e| {
        warn!("Failed to get duration: {}", e);
        Duration::from_secs(0)
    })
}

struct CheonMore {
    client: CheeseApiClient<Channel>,
    account: Account
}

impl CheonMore {
    pub fn new(client: CheeseApiClient<Channel>, account: Account) -> Self {
        CheonMore {
            client,
            account
        }
    }

    pub fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.is_weekend()
    }

    fn is_weekend(&self) -> Result<(), Box<dyn std::error::Error>> {
        let now = Local::now();
        if now.weekday() == chrono::Weekday::Sat
            || now.weekday() == chrono::Weekday::Sun
        {
            Err(Box::new(CheonMoreError::WeekendError))
        } else { Ok(()) }
    }

    async fn transfer_deposit_to_stock(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let deposit = self.client
            .lookup_future_option_deposit(FutureOptionDepositRequest {
                account_number: self.account.number.clone(),
                password: self.account.password.clone(),
            }).await?
            .into_inner()
            .withdrawal_amount;

        self.transfer_deposit(10, 1, deposit).await?;
        Ok(deposit)
    }

    async fn transfer_deposit_to_futures(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let deposit = self.client
            .lookup_account_deposit_info(AccountDepositInfoRequest {
                account_number: self.account.number.clone(),
                password: self.account.password.clone(),
            }).await?
            .into_inner()
            .withdrawal_amount;

        self.transfer_deposit(1, 10, deposit).await?;
        Ok(deposit)
    }

    async fn transfer_deposit(&mut self, withdrawal_item: i32, deposit_item: i32, deposit: i32)
        -> Result<(), Box<dyn std::error::Error>>
    {
        if deposit == 0 {
            return Ok(());
        }

        self.client.transfer_deposit(TransferDepositRequest {
            account_number: self.account.number.clone(),
            withdrawal_item_number: format!("{:02}", withdrawal_item),
            password: self.account.password.clone(),
            deposit_item_number: format!("{:02}", deposit_item),
            transfer_amount: deposit,
        }).await?;

        Ok(())
    }
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


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_duration() {
        let time = NaiveTime::from_hms_opt(8, 30, 0).unwrap();
        let duration = get_duration(time);
        assert_eq!(duration.as_secs(), 0);
    }
}