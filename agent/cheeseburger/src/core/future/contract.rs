use crate::client::cheese_api::{FutureOptionContract, TradeSep};
use crate::core::future::TradeType;

#[derive(Debug, Clone)]
pub struct Contract {
    pub(crate) code: String,
    pub(crate) trade_type: TradeType,
    pub(crate) _amount: i32,
    pub(crate) closable_amount: i32
}

impl TryFrom<FutureOptionContract> for Contract {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_from(contract: FutureOptionContract) -> Result<Self, Self::Error> {
        Ok(Self {
            code: contract.code,
            trade_type: TradeType::from(TradeSep::try_from(contract.trade_sep)?),
            _amount: contract.amount,
            closable_amount: contract.closable_amount
        })
    }
}
