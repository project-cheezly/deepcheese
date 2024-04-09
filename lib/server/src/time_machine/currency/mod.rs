pub mod adapter;
pub mod service;

use std::fmt::{Display, Formatter};
use std::str::FromStr;
use sqlx::types::BigDecimal;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub enum CurrencyType {
    KRW,
    USD,
}

#[derive(Debug, Clone)]
pub struct ParseCurrencyTypeError(String);

impl Display for ParseCurrencyTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse currency type {}", self.0)
    }
}

impl std::error::Error for ParseCurrencyTypeError {}

impl FromStr for CurrencyType {
    type Err = ParseCurrencyTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "KRW" => Ok(CurrencyType::KRW),
            "USD" => Ok(CurrencyType::USD),
            _ => Err(ParseCurrencyTypeError(s.to_string()))
        }
    }
}

pub type CurrencyValue = BigDecimal;