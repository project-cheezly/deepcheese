use crate::client;
use crate::client::cheese_api::FutureOptionDepositRequest;
use crate::core::Account;

pub async fn get_value(account: &Account)
    -> Result<i32, Box<dyn std::error::Error + Sync + Send>>
{
    let mut client = client::new().await?;

    Ok(client
        .lookup_future_option_deposit(FutureOptionDepositRequest {
                account_number: account.number.to_string(),
                password: account.password.to_string(),
        }).await?
        .into_inner()
        .liquidated_total_value
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_value()
        -> Result<(), Box<dyn std::error::Error + Sync + Send>>
    {
        let account = Account {
            number: "12345678901".to_string(),
            password: "1234".to_string(),
        };

        let value = get_value(&account).await?;
        assert!(value > 0);

        Ok(())
    }
}
