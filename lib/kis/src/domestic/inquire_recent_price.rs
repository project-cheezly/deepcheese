use serde::Deserialize;
use reqwest::Url;

use super::{DomesticStockCandle, KISDomestic};
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
    pub(crate) output: Option<Vec<DomesticStockCandle>>,
}

impl KISDomestic {
    /// 최근 일자별 주식현재가 API입니다.
    /// 일별 주가를 확인할 수 있으며 최근 30일로 제한되어 있습니다.
    ///
    /// ## Arguments
    /// - code: 종목 코드
    ///
    /// ## Return
    /// 조회 일자를 포함해 이전 30일의 일별 주가를 반환합니다.
    pub async fn inquire_recent_daily_stock_price(&self, code: &str)
        -> Result<Vec<DomesticStockCandle>, Box<dyn std::error::Error>>
    {
        self.inquire_recent_interval_stock_price(code, 'D').await
    }

    /// 최근 주별 주식현재가 API입니다.
    /// 주별 주가를 확인할 수 있으며 최근 30주로 제한되어 있습니다.
    ///
    /// ## Arguments
    /// - code: 종목 코드
    ///
    /// ## Return
    /// 조회 일자를 포함해 이전 30주의 주별 주가를 반환합니다.
    pub async fn inquire_recent_weekly_stock_price(&self, code: &str)
        -> Result<Vec<DomesticStockCandle>, Box<dyn std::error::Error>>
    {
        self.inquire_recent_interval_stock_price(code, 'W').await
    }

    /// 최근 월별 주식현재가 API입니다.
    /// 월별 주가를 확인할 수 있으며 최근 30개월로 제한되어 있습니다.
    ///
    /// ## Arguments
    /// - code: 종목 코드
    ///
    /// ## Return
    /// 조회 일자를 포함해 이전 30개월의 월별 주가를 반환합니다.
pub async fn inquire_recent_monthly_stock_price(&self, code: &str)
        -> Result<Vec<DomesticStockCandle>, Box<dyn std::error::Error>>
    {
        self.inquire_recent_interval_stock_price(code,'M').await
    }

    async fn inquire_recent_interval_stock_price(&self, code: &str, interval: char)
        -> Result<Vec<DomesticStockCandle>, Box<dyn std::error::Error>>
    {
        self.wait().await;

        let auth_header = self.get_default_header();
        let client = reqwest::Client::new();
        let uri = Url::parse(format!(
            "{}{}",
            &self.config.kis.uri.production,
            &self.config.kis.endpoint.domestic.inquire_daily_stock_price
        ).as_ref())?;

        let response = client.get(uri)
            .headers(auth_header.await)
            .header("tr_id", &self.config.kis.tr.domestic.inquire_daily_stock_price)
            .query(&[
                ("FID_COND_MRKT_DIV_CODE", "J"),
                ("FID_INPUT_ISCD", code),
                ("FID_PERIOD_DIV_CODE", &interval.to_string()),
                ("FID_ORG_ADJ_PRC", "0")
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