use std::cmp::Ordering;
use std::sync::Arc;
use chrono::NaiveDate;

use serde::Deserialize;
use tokio::sync::Mutex;

use crate::config::AppConfig;
use crate::auth::KISAuth;
use crate::Candle;
use crate::rate_limiter::RateLimiter;
use crate::kis_parse::{parse_to_naive_date, parse_to_i32};

pub struct KISDomestic {
    rate_limiter: Arc<Mutex<RateLimiter>>,
    auth: Arc<Mutex<KISAuth>>,
    config: Arc<AppConfig>,
}

impl KISDomestic {
    pub(crate) fn new(
        rate_limiter: Arc<Mutex<RateLimiter>>,
        auth: Arc<Mutex<KISAuth>>,
        config: Arc<AppConfig>
    ) -> Self {
        KISDomestic {
            rate_limiter,
            auth,
            config
        }
    }

    async fn wait(&self) {
        self.rate_limiter.lock().await.wait().await;
    }

    async fn get_default_header(&self) -> reqwest::header::HeaderMap {
        self.auth.lock().await.get_header_map().await.unwrap()
    }
}

#[derive(Deserialize, Debug)]
pub struct DomesticStockCandle {
    #[serde(rename = "stck_bsop_date", deserialize_with = "parse_to_naive_date")]
    pub 일자: NaiveDate,
    #[serde(rename = "stck_clpr", deserialize_with = "parse_to_i32")]
    pub 종가: i32,
    #[serde(rename = "stck_oprc", deserialize_with = "parse_to_i32")]
    pub 시가: i32,
    #[serde(rename = "stck_hgpr", deserialize_with = "parse_to_i32")]
    pub 고가: i32,
    #[serde(rename = "stck_lwpr", deserialize_with = "parse_to_i32")]
    pub 저가: i32
}

impl Eq for DomesticStockCandle {}

impl PartialEq<Self> for DomesticStockCandle {
    fn eq(&self, other: &Self) -> bool {
        self.일자 == other.일자
    }
}

impl PartialOrd for DomesticStockCandle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.일자.partial_cmp(&other.일자)
    }
}

impl Candle for DomesticStockCandle {
    fn get_date(&self) -> NaiveDate {
        self.일자
    }
}

mod inquire_stock_price;
mod inquire_recent_price;
mod inquire_interval_price;
