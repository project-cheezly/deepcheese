use std::error::Error;
use serde::Deserialize;
use chrono::NaiveDate;
use reqwest::Url;
use crate::CandleData;

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
    pub(crate) output2: Option<Vec<DomesticStockCandle>>,
}

impl KISDomestic {
    /// 국내 주식 일별 시세를 조회합니다.
    /// 기준 일자를 포함해 최대 100일까지 조회 가능합니다.
    ///
    /// ## Arguments
    /// - `code`: 종목 코드
    /// - `date`: 기준 일자
    pub async fn inquire_daily_stock_price(&self, code: &str, start: NaiveDate, end: NaiveDate)
        -> Result<CandleData<DomesticStockCandle>, Box<dyn Error>>
    {
        self.inquire_interval_stock_price_loop(code, start, end, 'D').await
    }

    /// 국내 주식 주별 시세를 조회합니다.
    /// 기준 일자를 포함해 최대 100주까지 조회 가능합니다.
    ///
    /// ## Arguments
    /// - `code`: 종목 코드
    /// - `date`: 기준 일자
    pub async fn inquire_weekly_stock_price(&self, code: &str, start: NaiveDate, end: NaiveDate)
        -> Result<CandleData<DomesticStockCandle>, Box<dyn Error>>
    {
        self.inquire_interval_stock_price_loop(code, start, end, 'W').await
    }

    /// 국내 주식 월별 시세를 조회합니다.
    /// 기준 일자를 포함해 최대 100개월까지 조회 가능합니다.
    ///
    /// ## Arguments
    /// - `code`: 종목 코드
    /// - `start`: 시작 일자
    /// - `end`: 종료 일자
    pub async fn inquire_monthly_stock_price(&self, code: &str, start: NaiveDate, end: NaiveDate)
        -> Result<CandleData<DomesticStockCandle>, Box<dyn Error>>
    {
        self.inquire_interval_stock_price_loop(code, start, end, 'M').await
    }

    async fn inquire_interval_stock_price_loop(
        &self,
        stock_code: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
        interval: char
    ) -> Result<CandleData<DomesticStockCandle>, Box<dyn Error>>
    {
        let (mut continue_flag, mut result) = self.inquire_interval_stock_price(
            stock_code,
            end_date,
            interval
        ).await?;

        while continue_flag {
            if let Some(earliest_date) = result.earliest_date() {
                if earliest_date <= start_date {
                    break;
                }
            }

            let (cont, data) = self.inquire_interval_stock_price(
                stock_code,
                result.earliest_date().unwrap_or(end_date),
                interval
            ).await?;

            continue_flag = cont;
            result.concat(data);
        }

        result.truncate(start_date);
        Ok(result)
    }

    async fn inquire_interval_stock_price(&self, code: &str, date: NaiveDate, interval: char)
        -> Result<(bool, CandleData<DomesticStockCandle>), Box<dyn std::error::Error>>
    {
        self.wait().await;

        let auth_header = self.get_default_header();
        let client = reqwest::Client::new();
        let uri = Url::parse(format!(
            "{}{}",
            &self.config.kis.uri.production,
            &self.config.kis.endpoint.domestic.inquire_interval_stock_price
        ).as_ref())?;

        let response = client.get(uri)
            .headers(auth_header.await)
            .header("tr_id", &self.config.kis.tr.domestic.inquire_interval_stock_price)
            .query(&[
                ("FID_COND_MRKT_DIV_CODE", "J"),
                ("FID_INPUT_ISCD", code),
                ("FID_INPUT_DATE_1", "19000101"),
                ("FID_INPUT_DATE_2", date.format("%Y%m%d").to_string().as_ref()),
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

        let result = CandleData::new(response.output2.unwrap());
        let continuous = result.len() != 0;

        Ok((continuous, result))
    }
}