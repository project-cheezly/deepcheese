use std::fmt;
use std::fmt::Formatter;
use std::ops::Neg;
use std::str::FromStr;

mod transaction;
mod tr_record;
mod asset;
mod banking;
mod currency;
mod record_list;
mod chart;
pub mod machine;

#[derive(Debug, Copy, Clone)]
pub enum TrType {
    INFLOW,
    OUTFLOW,
    CONSISTENT,
}

impl TrType {
    pub fn get_signed_value<T>(&self, value: T) -> T
        where T: Default + Neg<Output=T>
    {
        match self {
            TrType::INFLOW => value,
            TrType::OUTFLOW => -value,
            TrType::CONSISTENT => T::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParseTrTypeError { }

impl fmt::Display for ParseTrTypeError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Invalid transaction type")
    }
}

impl std::error::Error for ParseTrTypeError { }

impl FromStr for TrType {
    type Err = ParseTrTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "INFLOW" => Ok(TrType::INFLOW),
            "OUTFLOW" => Ok(TrType::OUTFLOW),
            "CONSISTENT" => Ok(TrType::CONSISTENT),
            _ => Err(ParseTrTypeError {})
        }
    }
}

type CategoryId = i32;
