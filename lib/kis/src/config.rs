use config::Config;
use serde::Deserialize;

pub struct AppConfig {
    pub auth: AuthConfig,
}

pub mod uri {
    /// 한국투자증권 실전 서버 URI
    pub const PRODUCTION: &str = "https://openapi.koreainvestment.com:9443";
    /// 한국투자증권 모의 서버 URI
    #[allow(dead_code)]
    pub const SANDBOX: &str = "https://openapivts.koreainvestment.com:29443";
}

pub(crate) mod endpoint {
    /// 액세스 토큰 발급
    pub const AUTH_PUBLISH_TOKEN: &str = "/oauth2/tokenP";

    /// 국내주식 현재가 조회
    pub const DOMESTIC_INQUIRE_STOCK_PRICE: &str = "/uapi/domestic-stock/v1/quotations/inquire-price";
    /// 국내주식 최근 일/주/월별 주가 조회
    pub const DOMESTIC_INQUIRE_DAILY_STOCK_PRICE: &str = "/uapi/domestic-stock/v1/quotations/inquire-daily-price";
    /// 국내주식 일/주/월별 주가 조회
    pub const DOMESTIC_INQUIRE_INTERVAL_STOCK_PRICE: &str = "/uapi/domestic-stock/v1/quotations/inquire-daily-itemchartprice";

    /// 해외주식 현재가 조회
    pub const OVERSEAS_INQUIRE_STOCK_PRICE: &str = "/uapi/overseas-price/v1/quotations/price";
    /// 해외주식 기간별 시세 조회
    pub const OVERSEAS_INQUIRE_INTERVAL_STOCK_PRICE: &str = "/uapi/overseas-price/v1/quotations/dailyprice";
    /// 해외 지수/환율 기간별 시세 조회
    pub const OVERSEAS_INQUIRE_INTERVAL_INDEX_VALUE: &str = "/uapi/overseas-price/v1/quotations/inquire-daily-chartprice";
}

pub(crate) mod tr {
    pub const DOMESTIC_INQUIRE_STOCK_PRICE: &str = "FHKST01010100";
    pub const DOMESTIC_INQUIRE_DAILY_STOCK_PRICE: &str = "FHKST01010400";
    pub const DOMESTIC_INQUIRE_INTERVAL_STOCK_PRICE: &str = "FHKST03010100";

    pub const OVERSEAS_INQUIRE_STOCK_PRICE: &str = "HHDFS00000300";
    pub const OVERSEAS_INQUIRE_INTERVAL_STOCK_PRICE: &str = "HHDFS76240000";
    pub const OVERSEAS_INQUIRE_INTERVAL_INDEX_VALUE: &str = "FHKST03030100";
}

#[derive(Deserialize, Debug)]
pub struct AuthConfig {
    pub app_id: String,
    pub app_secret: String,
}

pub fn load() -> AppConfig {
    AppConfig {
        auth: load_auth_config(),
    }
}

fn load_auth_config() -> AuthConfig {
    let auth_config_builder = Config::builder()
        .add_source(config::Environment::with_prefix("KIS"));

    auth_config_builder
        .build()
        .unwrap()
        .try_deserialize::<AuthConfig>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use uri;

    #[test]
    fn test_config() -> Result<(), Box<dyn std::error::Error>> {
        let _ = load();

        assert_eq!(uri::PRODUCTION, "https://openapi.koreainvestment.com:9443");
        Ok(())
    }
}
