pub mod balance;

use std::fmt::Formatter;
use std::str::FromStr;

pub type AssetAmount = i32;

#[derive(Hash, Debug, Clone, Eq, PartialEq)]
pub struct AssetId(pub MarketCode, pub String);

impl AssetId {
    pub fn is_domestic(&self) -> bool {
        match self.0 {
            MarketCode::KOSPI | MarketCode::KOSDAQ => true,
            _ => false
        }
    }
}

#[derive(Hash, Debug, Clone, Copy, Eq, PartialEq)]
pub enum MarketCode {
    KOSPI,
    KOSDAQ,
    HKS,
    NYS,
    BAY,
    NAS,
    BAQ,
    AMS,
    BAA
}

#[derive(Debug, Clone)]
pub struct ParseMarketCodeError { }

impl std::fmt::Display for ParseMarketCodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid market code")
    }
}

impl std::error::Error for ParseMarketCodeError { }

impl FromStr for MarketCode {
    type Err = ParseMarketCodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "KOSPI" => Ok(MarketCode::KOSPI),
            "KOSDAQ" => Ok(MarketCode::KOSDAQ),
            "HKS" => Ok(MarketCode::HKS),
            "NYS" => Ok(MarketCode::NYS),
            "BAY" => Ok(MarketCode::BAY),
            "NAS" => Ok(MarketCode::NAS),
            "BAQ" => Ok(MarketCode::BAQ),
            "AMS" => Ok(MarketCode::AMS),
            "BAA" => Ok(MarketCode::BAA),
            _ => Err(Self::Err { })
        }
    }
}

impl From<kis::MarketCode> for MarketCode {
    fn from(value: kis::MarketCode) -> Self {
        match value {
            kis::MarketCode::HKS => MarketCode::HKS,
            kis::MarketCode::NYS => MarketCode::NYS,
            kis::MarketCode::BAY => MarketCode::BAY,
            kis::MarketCode::NAS => MarketCode::NAS,
            kis::MarketCode::BAQ => MarketCode::BAQ,
            kis::MarketCode::AMS => MarketCode::AMS,
            kis::MarketCode::BAA => MarketCode::BAA,
        }
    }
}

impl std::fmt::Display for MarketCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            MarketCode::KOSPI => "KOSPI",
            MarketCode::KOSDAQ => "KOSDAQ",
            MarketCode::HKS => "HKS",
            MarketCode::NYS => "NYS",
            MarketCode::BAY => "BAY",
            MarketCode::NAS => "NAS",
            MarketCode::BAQ => "BAQ",
            MarketCode::AMS => "AMS",
            MarketCode::BAA => "BAA",
        };

        write!(f, "{}", output)
    }
}

#[derive(Debug, Clone)]
pub struct InvalidMarketCodeError;

impl std::fmt::Display for InvalidMarketCodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid market code")
    }
}

impl std::error::Error for InvalidMarketCodeError {}

impl TryInto<kis::MarketCode> for MarketCode {
    type Error = InvalidMarketCodeError;

    fn try_into(self) -> Result<kis::MarketCode, Self::Error> {
        match self {
            MarketCode::HKS => Ok(kis::MarketCode::HKS),
            MarketCode::NYS => Ok(kis::MarketCode::NYS),
            MarketCode::BAY => Ok(kis::MarketCode::BAY),
            MarketCode::NAS => Ok(kis::MarketCode::NAS),
            MarketCode::BAQ => Ok(kis::MarketCode::BAQ),
            MarketCode::AMS => Ok(kis::MarketCode::AMS),
            MarketCode::BAA => Ok(kis::MarketCode::BAA),
            _ => Err(InvalidMarketCodeError { })
        }
    }
}