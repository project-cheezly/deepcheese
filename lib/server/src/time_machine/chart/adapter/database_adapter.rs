use chrono::NaiveDate;
use sqlx::{Executor, PgPool, Row};
use sqlx::types::BigDecimal;
use crate::time_machine::chart::adapter::ChartRecordListSource;
use crate::time_machine::chart::Chart;

pub struct DatabaseChartAdapter<'a> {
    pool: &'a PgPool
}

impl<'a> DatabaseChartAdapter<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        DatabaseChartAdapter { pool }
    }
}

impl<'a> ChartRecordListSource for DatabaseChartAdapter<'a> {
    async fn get_chart_record_list(
        &self,
        date: NaiveDate,
        asset_id: &crate::time_machine::asset::AssetId
    ) -> Result<Chart, Box<dyn std::error::Error>>
    {
        let mut conn = self.pool.acquire().await?;
        let query_str = include_str!("../../sql/load_chart.sql");

        let rows = conn.fetch_all(sqlx::query(query_str)
                .bind(date)
                .bind(asset_id.0.to_string())
                .bind(asset_id.1.clone()))
            .await?
            .into_iter()
            .map(|row| (
                row.get::<NaiveDate, _>("date"),
                row.get::<BigDecimal, _>("value")));


        Ok(Chart::from_iter(rows))
    }
}

#[cfg(test)]
mod tests {
    use crate::database::connect_to_database;
    use crate::time_machine::asset::{AssetId, MarketCode};
    use super::*;

    #[tokio::test]
    async fn test_get_chart_record_list() {
        let pool = connect_to_database().await.unwrap();
        let adapter = DatabaseChartAdapter::new(&pool);

        let date = NaiveDate::from_ymd_opt(2021, 1, 1).unwrap();
        let asset_id = AssetId(MarketCode::AMS, "VT".to_string());

        let chart = adapter.get_chart_record_list(date, &asset_id).await.unwrap();
        assert_eq!(chart.data.len(), 6);
    }
}