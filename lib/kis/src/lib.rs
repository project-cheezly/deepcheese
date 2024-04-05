mod config;
pub mod kis;
mod auth;
mod rate_limiter;
mod domestic;
mod kis_parse;
mod overseas;

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

impl MarketCode {
    fn get_code(&self) -> &str {
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

