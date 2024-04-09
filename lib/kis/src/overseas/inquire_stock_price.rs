use serde::Deserialize;
use reqwest::Url;
use crate::config::{endpoint, tr, uri};

use super::KISOverseas;
use crate::kis_parse::{parse_to_f64, parse_to_i32};
use crate::MarketCode;

#[derive(Deserialize, Debug)]
struct Response {
    #[serde(rename = "rt_cd", deserialize_with = "parse_to_i32")]
    return_code: i32,
    #[serde(rename= "msg_cd")]
    pub(crate) response_code: String,
    #[serde(rename = "msg1")]
    pub(crate) response_message: String,
    pub(crate) output: Option<InquireStockPriceResponse>,
}

#[derive(Deserialize, Debug)]
pub struct InquireStockPriceResponse {
    #[serde(rename = "last", deserialize_with = "parse_to_f64")]
    pub 현재가: f64
}

impl KISOverseas {
    /// 해외 주식 종목의 현재 체결가를 조회합니다.
    ///
    /// ## Arguments
    /// - `stock_code`: 종목 코드
    /// - `market_code`: 시장 코드
    pub async fn inquire_stock_price(
        &self,
        stock_code: &str,
        market_code: MarketCode
    ) -> Result<InquireStockPriceResponse, Box<dyn std::error::Error>>
    {
        self.wait().await;

        let auth_header = self.get_default_header();
        let client = reqwest::Client::new();
        let uri = Url::parse(format!(
            "{}{}",
            uri::PRODUCTION,
            endpoint::OVERSEAS_INQUIRE_STOCK_PRICE
        ).as_ref())?;

        let response = client.get(uri)
            .headers(auth_header.await)
            .header("tr_id", tr::OVERSEAS_INQUIRE_STOCK_PRICE)
            .query(&[
                ("AUTH", ""),
                ("EXCD", &market_code.to_string()),
                ("SYMB", stock_code)
            ])
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            return Err(format!("HTTP 오류: {} - {}", status, text).into());
        }

        let response = response.json::<Response>().await?;

        if response.return_code != 0 {
            return Err(format!(
                "API 오류: {} - {}",
                response.response_code,
                response.response_message
            ).into());
        }

        Ok(response.output.unwrap())
    }
}