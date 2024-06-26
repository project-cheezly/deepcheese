use std::cmp::Ordering;
use std::sync::Arc;
use chrono::NaiveDate;

use serde::Deserialize;
use tokio::sync::Mutex;

use crate::auth::KISAuth;
use crate::Candle;
use crate::rate_limiter::RateLimiter;
use crate::kis_parse::{parse_to_f64, parse_to_naive_date};

pub struct KISOverseas {
    rate_limiter: Arc<Mutex<RateLimiter>>,
    auth: Arc<Mutex<KISAuth>>
}

impl KISOverseas {
    pub(crate) fn new(
        rate_limiter: Arc<Mutex<RateLimiter>>,
        auth: Arc<Mutex<KISAuth>>
    ) -> Self {
        KISOverseas {
            rate_limiter,
            auth
        }
    }

    async fn wait(&self) {
        self.rate_limiter.lock().await.wait().await;
    }

    async fn get_default_header(&self) -> reqwest::header::HeaderMap {
        self.auth.lock().await.get_header_map().await.unwrap()
    }
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct OverseasStockCandle {
    #[serde(rename = "xymd", deserialize_with = "parse_to_naive_date")]
    pub 일자: NaiveDate,
    #[serde(rename = "clos", deserialize_with = "parse_to_f64")]
    pub 종가: f64,
    #[serde(rename = "open", deserialize_with = "parse_to_f64")]
    pub 시가: f64,
    #[serde(rename = "high", deserialize_with = "parse_to_f64")]
    pub 고가: f64,
    #[serde(rename = "low", deserialize_with = "parse_to_f64")]
    pub 저가: f64
}

impl Candle for OverseasStockCandle {
    type Value = f64;

    fn date(&self) -> NaiveDate {
        self.일자
    }

    fn 시가(&self) -> Self::Value { self.시가 }

    fn 고가(&self) -> Self::Value { self.고가 }

    fn 저가(&self) -> Self::Value { self.저가 }

    fn 종가(&self) -> Self::Value { self.종가 }
}

impl Eq for OverseasStockCandle { }

impl PartialEq<Self> for OverseasStockCandle {
    fn eq(&self, other: &Self) -> bool {
        self.일자 == other.일자
    }
}

impl PartialOrd<Self> for OverseasStockCandle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.일자.partial_cmp(&other.일자)
    }
}

impl Ord for OverseasStockCandle {
    fn cmp(&self, other: &Self) -> Ordering {
        self.일자.partial_cmp(&other.일자).unwrap()
    }

}

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct OverseasIndexCandle {
    #[serde(rename = "stck_bsop_date", deserialize_with = "parse_to_naive_date")]
    pub 일자: NaiveDate,
    #[serde(rename = "ovrs_nmix_prpr", deserialize_with = "parse_to_f64")]
    pub 종가: f64,
    #[serde(rename = "ovrs_nmix_oprc", deserialize_with = "parse_to_f64")]
    pub 시가: f64,
    #[serde(rename = "ovrs_nmix_hgpr", deserialize_with = "parse_to_f64")]
    pub 고가: f64,
    #[serde(rename = "ovrs_nmix_lwpr", deserialize_with = "parse_to_f64")]
    pub 저가: f64
}

impl Eq for OverseasIndexCandle { }

impl PartialEq<Self> for OverseasIndexCandle {
    fn eq(&self, other: &Self) -> bool {
        self.일자 == other.일자
    }
}

impl PartialOrd<Self> for OverseasIndexCandle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.일자.partial_cmp(&other.일자)
    }
}

impl Candle for OverseasIndexCandle {
    type Value = f64;

    fn date(&self) -> NaiveDate {
        self.일자
    }

    fn 시가(&self) -> Self::Value { self.시가 }

    fn 고가(&self) -> Self::Value { self.고가 }

    fn 저가(&self) -> Self::Value { self.저가 }

    fn 종가(&self) -> Self::Value { self.종가 }
}

mod inquire_stock_price;
mod inquire_interval_stock_price;
mod inquire_interval_value;