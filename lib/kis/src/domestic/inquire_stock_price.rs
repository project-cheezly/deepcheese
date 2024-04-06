use serde::Deserialize;
use reqwest::Url;
use crate::config::{endpoint, tr, uri};

use super::KISDomestic;
use crate::kis_parse::parse_to_i32;

#[derive(Deserialize, Debug)]
struct Response {
    #[serde(rename = "rt_cd", deserialize_with = "parse_to_i32")]
    return_code: i32,
    #[serde(rename= "msg_cd")]
    pub(crate) response_code: String,
    #[serde(rename = "msg1")]
    pub(crate) response_message: String,
    #[serde(rename = "output")]
    pub(crate) output: Option<InquireStockPriceResponse>,
}

#[derive(Deserialize, Debug)]
pub struct InquireStockPriceResponse {
    #[serde(rename = "stck_prpr", deserialize_with = "parse_to_i32")]
    pub 현재가: i32,
    #[serde(rename = "stck_oprc", deserialize_with = "parse_to_i32")]
    pub 시가: i32,
    #[serde(rename = "stck_hgpr", deserialize_with = "parse_to_i32")]
    pub 고가: i32,
    #[serde(rename = "stck_lwpr", deserialize_with = "parse_to_i32")]
    pub 저가: i32
}

impl KISDomestic {
    /// 국내 주식 현재가 시세를 조회합니다.
    ///
    /// ## Arguments
    /// - `code`: 종목 코드
    pub async fn inquire_stock_price(&self, code: &str)
        -> Result<InquireStockPriceResponse, Box<dyn std::error::Error>>
    {
        self.wait().await;

        let auth_header = self.get_default_header();
        let client = reqwest::Client::new();
        let uri = Url::parse(format!(
            "{}{}",
            uri::PRODUCTION,
            endpoint::DOMESTIC_INQUIRE_STOCK_PRICE
        ).as_ref())?;

        let response = client.get(uri)
            .headers(auth_header.await)
            .header("tr_id", tr::DOMESTIC_INQUIRE_STOCK_PRICE)
            .query(&[
                ("FID_COND_MRKT_DIV_CODE", "J"),
                ("FID_INPUT_ISCD", code)
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
