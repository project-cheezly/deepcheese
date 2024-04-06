use serde::Deserialize;
use chrono::NaiveDate;
use reqwest::Url;
use crate::CandleData;

use super::{KISOverseas, OverseasIndexCandle};
use crate::kis_parse::parse_to_i32;

#[derive(Deserialize, Debug)]
struct Response {
    #[serde(rename = "rt_cd", deserialize_with = "parse_to_i32")]
    return_code: i32,
    #[serde(rename= "msg_cd")]
    pub(crate) response_code: String,
    #[serde(rename = "msg1")]
    pub(crate) response_message: String,
    pub(crate) output2: Option<Vec<OverseasIndexCandle>>,
}


impl KISOverseas {
    /// 해외 지수 일별 시세를 조회합니다.
    ///
    /// ## Arguments
    /// - `index_code`: 지수 코드
    /// - `start_date`: 조회 시작 일자
    /// - `end_date`: 조회 종료 일자
    pub async fn inquire_daily_index_value(
        &self,
        index_code: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<CandleData<OverseasIndexCandle>, Box<dyn std::error::Error>>
    {
        self.inquire_interval_index_value_loop(
            index_code,
            start_date,
            end_date,
            'D',
            'N'
        ).await
    }

    /// 해외 지수 주별 시세를 조회합니다.
    ///
    /// ## Arguments
    /// - `index_code`: 종목 코드
    /// - `start_date`: 조회 시작 일자
    /// - `end_date`: 조회 종료 일자
    pub async fn inquire_weekly_index_value(
        &self,
        index_code: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<CandleData<OverseasIndexCandle>, Box<dyn std::error::Error>>
    {
        self.inquire_interval_index_value_loop(
            index_code,
            start_date,
            end_date,
            'W',
            'N'
        ).await
    }

    /// 해외 지수 월별 시세를 조회합니다.
    ///
    /// ## Arguments
    /// - `index_code`: 종목 코드
    /// - `start_date`: 조회 시작 일자
    /// - `end_date`: 조회 종료 일자
    pub async fn inquire_monthly_index_value(
        &self,
        index_code: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<CandleData<OverseasIndexCandle>, Box<dyn std::error::Error>>
    {
        self.inquire_interval_index_value_loop(
            index_code,
            start_date,
            end_date,
            'M',
            'N'
        ).await
    }

    /// 환율 일별 시세를 조회합니다.
    ///
    /// ## Arguments
    /// - `currency_code`: 통화 코드
    /// - `start_date`: 조회 시작 일자
    /// - `end_date`: 조회 종료 일자
    pub async fn inquire_daily_forex_value(
        &self,
        currency_code: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<CandleData<OverseasIndexCandle>, Box<dyn std::error::Error>>
    {
        self.inquire_interval_index_value_loop(
            currency_code,
            start_date,
            end_date,
            'D',
            'X'
        ).await
    }

    /// 환율 주별 시세를 조회합니다.
    ///
    /// ## Arguments
    /// - `currency_code`: 통화 코드
    /// - `start_date`: 조회 시작 일자
    /// - `end_date`: 조회 종료 일자
    pub async fn inquire_weekly_forex_value(
        &self,
        currency_code: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<CandleData<OverseasIndexCandle>, Box<dyn std::error::Error>>
    {
        self.inquire_interval_index_value_loop(
            currency_code,
            start_date,
            end_date,
            'W',
            'X'
        ).await
    }

    /// 환율 월별 시세를 조회합니다.
    ///
    /// ## Arguments
    /// - `currency_code`: 통화 코드
    /// - `start_date`: 조회 시작 일자
    /// - `end_date`: 조회 종료 일자
    pub async fn inquire_monthly_forex_value(
        &self,
        currency_code: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<CandleData<OverseasIndexCandle>, Box<dyn std::error::Error>>
    {
        self.inquire_interval_index_value_loop(
            currency_code,
            start_date,
            end_date,
            'M',
            'X'
        ).await
    }

    async fn inquire_interval_index_value_loop(
        &self,
        code: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
        interval: char,
        div_code: char,
    ) -> Result<CandleData<OverseasIndexCandle>, Box<dyn std::error::Error>>
    {
        let (mut continuous, mut result) = self.inquire_interval_index_value(
            code,
            end_date,
            interval,
            div_code
        ).await?;

        while continuous {
            if let Some(date) = result.earliest_date() {
                if date <= start_date {
                    break;
                }
            }

            let (cont, data) = self.inquire_interval_index_value(
                code,
                result.earliest_date().unwrap(),
                interval,
                div_code,
            ).await?;

            continuous = cont;
            result.concat(data);
        }

        Ok(result)
    }

    async fn inquire_interval_index_value(
        &self,
        code: &str,
        end_date: NaiveDate,
        interval: char,
        div_code: char,
    ) -> Result<(bool, CandleData<OverseasIndexCandle>), Box<dyn std::error::Error>>
    {
        self.wait().await;

        let auth_header = self.get_default_header();
        let client = reqwest::Client::new();

        let url = Url::parse(format!(
            "{}{}",
            &self.config.kis.uri.production,
            &self.config.kis.endpoint.overseas.inquire_interval_index_value
        ).as_ref())?;

        let response = client.get(url)
            .headers(auth_header.await)
            .header("tr_id", &self.config.kis.tr.overseas.inquire_interval_index_value)
            .query(&[
                ("FID_COND_MRKT_DIV_CODE", div_code.to_string().as_str()),
                ("FID_INPUT_ISCD", code),
                ("FID_INPUT_DATE_1", "19000101"),
                ("FID_INPUT_DATE_2", end_date.format("%Y%m%d").to_string().as_str()),
                ("FID_PERIOD_DIV_CODE", &interval.to_string())
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