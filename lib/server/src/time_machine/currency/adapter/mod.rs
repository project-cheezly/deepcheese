pub mod kis_adapter;

use chrono::NaiveDate;
use crate::time_machine::chart::Chart;
use crate::time_machine::currency::CurrencyType;

pub trait CurrencyChartRecordListSource {
    async fn get_currency_chart_record_list(
        &self,
        date: NaiveDate,
        currency_type: &CurrencyType
    ) -> Result<Chart, Box<dyn std::error::Error>>;
}