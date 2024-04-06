use std::sync::Arc;

use serde::Deserialize;
use tokio::sync::Mutex;

use crate::config::AppConfig;
use crate::auth::KISAuth;
use crate::rate_limiter::RateLimiter;
use crate::kis_parse::parse_to_f64;

pub struct KISOverseas {
    rate_limiter: Arc<Mutex<RateLimiter>>,
    auth: Arc<Mutex<KISAuth>>,
    config: Arc<AppConfig>,
}

impl KISOverseas {
    pub(crate) fn new(
        rate_limiter: Arc<Mutex<RateLimiter>>,
        auth: Arc<Mutex<KISAuth>>,
        config: Arc<AppConfig>
    ) -> Self {
        KISOverseas {
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
pub struct OverseasStockCandle {
    #[serde(rename = "stck_clpr", deserialize_with = "parse_to_f64")]
    pub 종가: f64,
    #[serde(rename = "stck_oprc", deserialize_with = "parse_to_f64")]
    pub 시가: f64,
    #[serde(rename = "stck_hgpr", deserialize_with = "parse_to_f64")]
    pub 고가: f64,
    #[serde(rename = "stck_lwpr", deserialize_with = "parse_to_f64")]
    pub 저가: f64
}

mod inquire_stock_price;