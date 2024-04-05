use std::sync::Arc;

use serde::Deserialize;
use tokio::sync::Mutex;

use crate::config::AppConfig;
use crate::auth::KISAuth;
use crate::rate_limiter::RateLimiter;
use crate::kis_parse::parse_to_i32;

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
        self.auth.lock().await.get_header_map().unwrap()
    }
}

#[derive(Deserialize, Debug)]
pub struct DomesticStockCandle {
    #[serde(rename = "stck_clpr", deserialize_with = "parse_to_i32")]
    pub 종가: i32,
    #[serde(rename = "stck_oprc", deserialize_with = "parse_to_i32")]
    pub 시가: i32,
    #[serde(rename = "stck_hgpr", deserialize_with = "parse_to_i32")]
    pub 고가: i32,
    #[serde(rename = "stck_lwpr", deserialize_with = "parse_to_i32")]
    pub 저가: i32
}

mod inquire_stock_price;
mod inquire_recent_price;
mod inquire_interval_price;
