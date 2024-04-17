use std::fmt::{Display, Formatter};
use serde::{Deserialize, Deserializer};
use crate::error::CheeseburgerError;

#[derive(Debug, Clone)]
pub enum FutureType {
    Kospi200,
    Kosdaq150,
}

impl FutureType {
    pub fn base_code(&self) -> &str {
        match self {
            FutureType::Kospi200 => "01",
            FutureType::Kosdaq150 => "06",
        }
    }
}

impl Display for FutureType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FutureType::Kospi200 => write!(f, "KOSPI"),
            FutureType::Kosdaq150 => write!(f, "KOSDAQ"),
        }
    }
}

impl TryFrom<String> for FutureType {
    type Error = CheeseburgerError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "KOSPI" => Ok(FutureType::Kospi200),
            "KOSDAQ" => Ok(FutureType::Kosdaq150),
            _ => Err(CheeseburgerError::ParseError(format!("Failed to parse: {}", s)))
        }
    }
}

impl TryFrom<&str> for FutureType {
    type Error = CheeseburgerError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "KOSPI" => Ok(FutureType::Kospi200),
            "KOSDAQ" => Ok(FutureType::Kosdaq150),
            _ => Err(CheeseburgerError::ParseError(format!("Failed to parse: {}", s)))
        }
    }
}

impl AsRef<str> for FutureType {
    fn as_ref(&self) -> &str {
        match self {
            FutureType::Kospi200 => "Kospi200",
            FutureType::Kosdaq150 => "Kosdaq150",
        }
    }
}

impl<'de> Deserialize<'de> for FutureType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        FutureType::try_from(s).map_err(serde::de::Error::custom)
    }
}