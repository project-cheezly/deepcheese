use std::sync::Arc;

use tokio::sync::Mutex;

use crate::auth::KISAuth;
use crate::config;
use crate::rate_limiter::RateLimiter;
use crate::domestic::KISDomestic;
use crate::overseas::KISOverseas;

/// 한국투자증권 API 구조체
pub struct KIS {
    pub domestic: KISDomestic,
    pub overseas: KISOverseas,
}

impl KIS {
    /// 새로운 KIS 구조체를 생성합니다.
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config = Arc::new(config::load());

        let rate_limiter = Arc::new(Mutex::new(RateLimiter::new()));
        let auth = Arc::new(Mutex::new(KISAuth::new(config.clone()).await));

        Ok(KIS {
            domestic: KISDomestic::new(rate_limiter.clone(), auth.clone()),
            overseas: KISOverseas::new(rate_limiter, auth),
        })
    }
}
