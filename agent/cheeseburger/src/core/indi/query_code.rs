use std::fmt::Display;
use serde::Deserialize;
use crate::error::CheeseburgerError;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum QueryCode {
    FutureOptionCurrentPrice,
    FutureOptionLimitOrderBook
}

impl<'de> Deserialize<'de> for QueryCode {
    fn deserialize<D>(deserializer: D) -> Result<QueryCode, D::Error>
        where D: serde::Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        QueryCode::try_from(s).map_err(serde::de::Error::custom)
    }
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