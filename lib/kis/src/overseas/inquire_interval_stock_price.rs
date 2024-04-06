use serde::Deserialize;
use chrono::NaiveDate;
use reqwest::Url;

use super::{KISOverseas, OverseasStockCandle};
use crate::kis_parse::parse_to_i32;
use crate::{CandleData, MarketCode};
use crate::config::{endpoint, tr, uri};

#[derive(Deserialize, Debug)]
struct Response {
    #[serde(rename = "rt_cd", deserialize_with = "parse_to_i32")]
    return_code: i32,
    #[serde(rename= "msg_cd")]
    pub(crate) response_code: String,
    #[serde(rename = "msg1")]
    pub(crate) response_message: String,
    pub(crate) output2: Option<Vec<OverseasStockCandle>>,
}

impl KISOverseas {
    /// 해외 주식 일별 시세를 조회합니다.
    ///
    /// ## Arguments
    /// - `stock_code`: 종목 코드
    /// - `market_code`: 시장 코드
    /// - `start_date`: 조회 시작 일자
    /// - `end_date`: 조회 종료 일자
    pub async fn inquire_daily_stock_price(
        &self,
        stock_code: &str,
        market_code: MarketCode,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<CandleData<OverseasStockCandle>, Box<dyn std::error::Error>>
    {
        self.inquire_interval_stock_price_loop(
            stock_code,
            market_code,
            start_date,
            end_date,
            'D',
        ).await
    }

    /// 해외 주식 주별 시세를 조회합니다.
    ///
    /// ## Arguments
    /// - `stock_code`: 종목 코드
    /// - `market_code`: 시장 코드
    /// - `start_date`: 조회 시작 일자
    /// - `end_date`: 조회 종료 일자
    pub async fn inquire_weekly_stock_price(
        &self,
        stock_code: &str,
        market_code: MarketCode,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<CandleData<OverseasStockCandle>, Box<dyn std::error::Error>>
    {
        self.inquire_interval_stock_price_loop(
            stock_code,
            market_code,
            start_date,
            end_date,
            'W',
        ).await
    }

    /// 해외 주식 월별 시세를 조회합니다.
    ///
    /// ## Arguments
    /// - `stock_code`: 종목 코드
    /// - `market_code`: 시장 코드
    /// - `start_date`: 조회 시작 일자
    /// - `end_date`: 조회 종료 일자
    pub async fn inquire_monthly_stock_price(
        &self,
        stock_code: &str,
        market_code: MarketCode,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<CandleData<OverseasStockCandle>, Box<dyn std::error::Error>>
    {
        self.inquire_interval_stock_price_loop(
            stock_code,
            market_code,
            start_date,
            end_date,
            'M',
        ).await
    }

    async fn inquire_interval_stock_price_loop(
        &self,
        stock_code: &str,
        market_code: MarketCode,
        start_date: NaiveDate,
        end_date: NaiveDate,
        interval: char,
    ) -> Result<CandleData<OverseasStockCandle>, Box<dyn std::error::Error>>
    {
        let (mut continuous, mut result) = self.inquire_interval_stock_price(
            stock_code,
            market_code,
            end_date,
            interval,
        ).await?;

        while continuous {
            if let Some(date) = result.earliest_date() {
                if date <= start_date {
                    break;
                }
            }

            let (cont, data) = self.inquire_interval_stock_price(
                stock_code,
                market_code,
                result.earliest_date().unwrap(),
                interval,
            ).await?;

            continuous = cont;
            result.concat(data);
        }

        Ok(result)
    }

    async fn inquire_interval_stock_price(
        &self,
        stock_code: &str,
        market_code: MarketCode,
        end_date: NaiveDate,
        interval: char,
    ) -> Result<(bool, CandleData<OverseasStockCandle>), Box<dyn std::error::Error>>
    {
        self.wait().await;

        let auth_header = self.get_default_header();
        let client = reqwest::Client::new();

        let url = Url::parse(format!(
            "{}{}",
            uri::PRODUCTION,
            endpoint::OVERSEAS_INQUIRE_INTERVAL_STOCK_PRICE
        ).as_ref())?;

        let response = client.get(url)
            .headers(auth_header.await)
            .header("tr_id", tr::OVERSEAS_INQUIRE_INTERVAL_STOCK_PRICE)
            .query(&[
                ("AUTH", ""),
                ("EXCD", market_code.as_ref()),
                ("SYMB", stock_code),
                ("GUBN", &interval.to_string()),
                ("BYMD", &end_date.format("%Y%m%d").to_string()),
                ("MODP", "0"),
            ])
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            return Err(format!("HTTP 오류: {} - {}", status, text).into());
        }

        let response_header = response.headers();

        let continuous = match response_header.get("tr_cont") {
            Some(value) => {
                let code = value.to_str()?;
                if code == "F" || code == "M" {
                    true
                } else {
                    false
                }
            },
            _ => false
        };

        let response = response.json::<Response>().await?;

        if response.return_code != 0 {
            return Err(format!(
                "API 오류: {} - {}",
                response.response_code,
                response.response_message
            ).into());
        }

        let result = CandleData::new(response.output2.unwrap());
        Ok((continuous, result))
    }
}