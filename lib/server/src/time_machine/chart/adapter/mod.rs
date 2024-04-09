use chrono::NaiveDate;
use crate::time_machine::asset::AssetId;
use crate::time_machine::chart::Chart;

pub mod kis_adapter;
pub mod database_adapter;

pub trait ChartRecordListSource {
    async fn get_chart_record_list(
        &self,
        date: NaiveDate,
        asset_id: &AssetId
    ) -> Result<Chart, Box<dyn std::error::Error>>;
}
