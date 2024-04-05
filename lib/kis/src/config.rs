use config::Config;
use serde::Deserialize;

pub struct AppConfig {
    pub auth: AuthConfig,
    pub kis: KISConfig,
}

/// 한국투자증권 OpenAPI 설정값
#[derive(Deserialize, Debug)]
pub struct KISConfig {
    pub uri: UriConfig,
    pub endpoint: EndpointConfig,
    pub tr: TrConfig
}

/// 한국투자증권 URI
#[derive(Deserialize, Debug)]
pub struct UriConfig {
    /// 한국투자증권 실전 서버 URI
    pub production: String,
    /// 한국투자증권 모의투자 서버 URI
    pub sandbox: String,
}

/// 한국투자증권 엔드포인트 설정값
#[derive(Deserialize, Debug)]
pub struct EndpointConfig {
    pub auth: EndpointAuthConfig,
    pub domestic: EndpointDomesticConfig,
    pub overseas: EndpointOverseasConfig,
}

#[derive(Deserialize, Debug)]
pub struct EndpointAuthConfig {
    /// 액세스 토큰 발급
    pub publish_token: String,
}

#[derive(Deserialize, Debug)]
pub struct EndpointDomesticConfig {
    /// 국내주식 현재가 조회
    pub inquire_stock_price: String,
    /// 국내주식 최근 일/주/월별 주가 조회
    pub inquire_daily_stock_price: String,
    /// 국내주식 일/주/월별 주가 조회
    pub inquire_interval_stock_price: String,
}

#[derive(Deserialize, Debug)]
pub struct EndpointOverseasConfig {
    /// 해외주식 현재가 조회
    pub inquire_stock_price: String,
}

#[derive(Deserialize, Debug)]
pub struct AuthConfig {
    pub app_id: String,
    pub app_secret: String,
}

#[derive(Deserialize, Debug)]
pub struct TrConfig {
    pub domestic: TrDomesticConfig,
    pub overseas: TrOverseasConfig
}

#[derive(Deserialize, Debug)]
pub struct TrDomesticConfig {
    pub inquire_stock_price: String,
    pub inquire_daily_stock_price: String,
    pub inquire_interval_stock_price: String,
}

#[derive(Deserialize, Debug)]
pub struct TrOverseasConfig {
    pub inquire_stock_price: String,
}

pub fn load() -> AppConfig {
    AppConfig {
        auth: load_auth_config(),
        kis: load_kis_config(),
    }
}

fn load_kis_config() -> KISConfig {
    let config_builder = Config::builder()
        .add_source(config::File::new("src/config.toml", config::FileFormat::Toml));

    config_builder
        .build()
        .unwrap()
        .try_deserialize::<KISConfig>().unwrap()
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

    #[test]
    fn test_config() -> Result<(), Box<dyn std::error::Error>> {
        let app_config = load();

        assert_eq!(app_config.kis.uri.production, "https://openapi.koreainvestment.com:9443");
        Ok(())
    }
}
