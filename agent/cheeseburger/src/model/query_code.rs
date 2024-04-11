use std::fmt::Display;
use crate::error::CheeseburgerError;

#[derive(Debug, Copy, Clone)]
pub enum QueryCode {
    FutureOptionCurrentPrice,
    FutureOptionLimitOrderBook
}

impl Display for QueryCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryCode::FutureOptionCurrentPrice => write!(f, "FC"),
            QueryCode::FutureOptionLimitOrderBook => write!(f, "FH"),
        }
    }
}

impl TryFrom<String> for QueryCode {
    type Error = CheeseburgerError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "FC" => Ok(QueryCode::FutureOptionCurrentPrice),
            "FH" => Ok(QueryCode::FutureOptionLimitOrderBook),
            _ => Err(CheeseburgerError::InvalidQueryCodeError(value))
        }
    }
}