use std::collections::BTreeMap;
use std::fmt::Display;
use std::str::FromStr;
use chrono::NaiveDate;

mod config;
pub mod kis;
mod auth;
mod rate_limiter;
mod domestic;
mod kis_parse;
mod overseas;

pub use kis::KIS;

#[derive(Clone, Copy)]
pub enum MarketCode {
    /// 홍콩
    HKS,
    /// 뉴욕증권거래소
    NYS,
    /// 뉴욕증권거래소 (주간)
    BAY,
    /// 나스닥
    NAS,
    /// 나스닥 (주간)
    BAQ,
    /// 아멕스
    AMS,
    /// 아멕스 (주간)
    BAA
}

impl Display for MarketCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            MarketCode::HKS => "HKS",
            MarketCode::NYS => "NYS",
            MarketCode::BAY => "BAY",
            MarketCode::NAS => "NAS",
            MarketCode::BAQ => "BAQ",
            MarketCode::AMS => "AMS",
            MarketCode::BAA => "BAA"
        };

        write!(f, "{}", code)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct MarketCodeParseError(String);

impl Display for MarketCodeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid market code: {}", self.0)
    }
}

impl std::error::Error for MarketCodeParseError {}

impl FromStr for MarketCode {
    type Err = MarketCodeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HKS" => Ok(Self::HKS),
            "NYS" => Ok(Self::NYS),
            "BAY" => Ok(Self::BAY),
            "NAS" => Ok(Self::NAS),
            "BAQ" => Ok(Self::BAQ),
            "AMS" => Ok(Self::AMS),
            "BAA" => Ok(Self::BAA),
            _ => Err(MarketCodeParseError(s.to_string()))
        }
    }
}

impl AsRef<str> for MarketCode {
    fn as_ref(&self) -> &str {
        match self {
            MarketCode::HKS => "HKS",
            MarketCode::NYS => "NYS",
            MarketCode::BAY => "BAY",
            MarketCode::NAS => "NAS",
            MarketCode::BAQ => "BAQ",
            MarketCode::AMS => "AMS",
            MarketCode::BAA => "BAA"
        }
    }
}

pub trait Candle: Eq + PartialOrd + PartialEq + Clone + Copy {
    type Value;

    fn date(&self) -> NaiveDate;
    fn 시가(&self) -> Self::Value;
    fn 고가(&self) -> Self::Value;
    fn 저가(&self) -> Self::Value;
    fn 종가(&self) -> Self::Value;
}

#[derive(Debug)]
pub struct CandleData<T> {
    data: BTreeMap<NaiveDate, T>,
}

impl<T> IntoIterator for CandleData<T>
    where T: Candle
{
    type Item = (NaiveDate, T);
    type IntoIter = std::collections::btree_map::IntoIter<NaiveDate, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<T: Candle> CandleData<T>
{
    pub fn new(candles: Vec<T>) -> Self
    {
        let mut data = BTreeMap::new();

        for candle in candles {
            data.insert(candle.date(), candle);
        }

        Self { data }
    }

    pub fn earliest_date(&self) -> Option<NaiveDate> {
        self.data.first_key_value().map(|(date, _)| *date)
    }

    pub fn latest_date(&self) -> Option<NaiveDate> {
        self.data.last_key_value().map(|(date, _)| *date)
    }

    pub fn concat(&mut self, other: Self) {
        for (date, candle) in other.data {
            self.data.insert(date, candle);
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// 현재 일자의 주가 정보를 가져옵니다.
    ///
    /// ## Arguments
    /// - `date`: 일자
    ///
    /// ## Returns
    /// `date`에 해당하는 주가 정보가 있으면 `Some(OverseasStockCandle)`, 없으면 `None`을 반환합니다.
    pub fn get(&self, date: NaiveDate) -> Option<&T> {
        self.data.get(&date)
    }

    /// 현재 날짜로부터 가장 가까운 일자의 주가 정보를 가져옵니다.
    ///
    /// ## Arguments
    ///
    /// - `date`: 일자
    ///
    /// ## Returns
    ///
    /// `date` 이전의 가장 가까운 일자의 주가 정보가 있으면 `Some(OverseasStockCandle)`, 없으면 `None`을 반환합니다.
    pub fn get_recent(&self, date: NaiveDate) -> Option<&T> {
        self.data.range(..=date).next_back().map(|(_, candle)| candle)
    }

    /// 기준일 이전의 데이터를 모두 삭제합니다.
    ///
    /// ## Arguments
    ///
    /// - `date`: 기준일
    pub fn truncate(&mut self, date: NaiveDate) {
        while let Some((&earliest_date, _)) = self.data.first_key_value() {
            if earliest_date < date {
                self.data.remove(&earliest_date);
            } else {
                break;
            }
        }
    }
}

