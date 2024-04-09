use num_traits::FromPrimitive;
use chrono::NaiveDate;
use kis::KIS;
use sqlx::types::BigDecimal;
use crate::time_machine::chart::Chart;
use crate::time_machine::currency::adapter::CurrencyChartRecordListSource;
use crate::time_machine::currency::CurrencyType;

const QUERY_CHART_DATE_LEFT_OFFSET: chrono::Duration = chrono::Duration::days(10);
const QUERY_CHART_DATE_RIGHT_OFFSET: chrono::Duration = chrono::Duration::days(100);

pub struct KISCurrencyChartAdapter<'a> {
    kis: &'a KIS
}

impl<'a> KISCurrencyChartAdapter<'a> {
    pub fn new(kis: &'a KIS) -> Self {
        KISCurrencyChartAdapter { kis }
    }
}

impl CurrencyChartRecordListSource for KISCurrencyChartAdapter<'_> {
    async fn get_currency_chart_record_list(
        &self,
        date: NaiveDate,
        currency_type: &CurrencyType
    ) -> Result<Chart, Box<dyn std::error::Error>>
    {
        if currency_type != &CurrencyType::USD {
            unimplemented!("Only USD is supported");
        }

        let start_date = date - QUERY_CHART_DATE_LEFT_OFFSET;
        let end_date = date + QUERY_CHART_DATE_RIGHT_OFFSET;

        let response = self.kis.overseas
            .inquire_daily_forex_value(
                "FX@KRW",
                start_date,
                end_date)
            .await?
            .into_iter()
            .map(|(date, value)| (date, BigDecimal::from_f64(value.종가).unwrap()));

        Ok(Chart::from_iter(response))
    }
}