use tonic::transport::Channel;
use crate::client;
use crate::client::cheese_api::cheese_api_client::CheeseApiClient;
use crate::client::cheese_api::FutureOptionContractRequest;
use crate::core::Account;
use crate::core::future::contract::Contract;
use crate::core::future::order::sell_market_order;
use crate::core::future::TradeType;

pub(crate) async fn pre_market(account: &Account)
    -> Result<(), Box<dyn std::error::Error + Send + Sync>>
{
    tracing::info!("Start pre-market");
    let mut client = client::new().await?;

    tracing::info!("Get previous contracts");
    let mut contracts = get_previous_contract(&mut client, account).await?;
    filter_contracts(&mut contracts);

    for contract in contracts {
        tracing::info!("Sell contract: {}, {}", contract.code, contract.closable_amount);
        sell_market_order(&mut client, account, &contract.code, contract.closable_amount).await?;
    }

    Ok(())
}

async fn get_previous_contract(client: &mut CheeseApiClient<Channel>, account: &Account)
    -> Result<Vec<Contract>, Box<dyn std::error::Error + Send + Sync>>
{
    let mut wait_second = 1;

    let contracts = loop {
        let contracts = client.lookup_future_option_contract(
            FutureOptionContractRequest {
                account_number: account.number.to_string(),
                password: account.password.to_string(),
            }
        ).await;

        if let Ok(contracts) = contracts {
            break contracts;
        } else {
            wait_second *= 2;
            tracing::warn!("Failed to get previous contract. Retry after {} seconds", wait_second);
        }
    };

    Ok(contracts
        .into_inner()
        .list
        .into_iter()
        .filter_map(|con| match Contract::try_from(con) {
            Ok(c) => Some(c),
            Err(e) => {
                tracing::error!("while parsing contract, error occurred: {}", e);
                None
            }
        }).collect::<Vec<_>>()
    )
}

fn filter_contracts(contracts: &mut Vec<Contract>) {
    contracts.retain(|con| con.trade_type == TradeType::Bid);
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::config;

    #[tokio::test]
    async fn test_get_previous_contract()
        -> Result<(), Box<dyn std::error::Error + Sync + Send>>
    {
        let mut client = client::new().await?;
        let config = config::load()?;

        let result = get_previous_contract(&mut client, &config.account).await?;
        dbg!(result);

        Ok(())
    }
}