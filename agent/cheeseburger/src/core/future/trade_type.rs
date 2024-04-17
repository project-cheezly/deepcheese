use crate::client::cheese_api::TradeSep;
use crate::error::CheeseburgerError;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum TradeType {
    Bid,
    Ask,
    Modify,
    Cancel
}

impl TryFrom<i32> for TradeType {
    type Error = Box<dyn std::error::Error + Sync + Send>;

    fn try_from(sep: i32) -> Result<Self, Self::Error> {
        match sep {
            0 => Ok(Self::Bid),
            1 => Ok(Self::Ask),
            2 => Ok(Self::Modify),
            3 => Ok(Self::Cancel),
            _ => Err(CheeseburgerError::ParseError("Failed to parse TradeType {sep}".to_string()).into())
        }
    }
}

impl From<TradeSep> for TradeType {
    fn from(value: TradeSep) -> Self {
        match value {
            TradeSep::Bid => Self::Bid,
            TradeSep::Ask => Self::Ask,
            TradeSep::Modify => Self::Modify,
            TradeSep::Cancel => Self::Cancel
        }
    }
}

impl Into<i32> for TradeType {
    fn into(self) -> i32 {
        match self {
            Self::Bid => i32::from(TradeSep::Bid),
            Self::Ask => i32::from(TradeSep::Ask),
            Self::Modify => i32::from(TradeSep::Modify),
            Self::Cancel => i32::from(TradeSep::Cancel)
        }
    }
}