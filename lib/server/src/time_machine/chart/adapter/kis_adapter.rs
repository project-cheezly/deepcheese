use std::error::Error;
use chrono::NaiveDate;
use kis::KIS;
use sqlx::types::BigDecimal;
use num_traits::cast::FromPrimitive;
use crate::time_machine::asset::AssetId;
use crate::time_machine::chart::Chart;
use crate::time_machine::chart::adapter::ChartRecordListSource;

const QUERY_CHART_DATE_LEFT_OFFSET: chrono::Duration = chrono::Duration::days(10);
const QUERY_CHART_DATE_RIGHT_OFFSET: chrono::Duration = chrono::Duration::days(100);

pub struct KISChartAdapter<'a> {
    kis: &'a KIS
}

impl<'a> KISChartAdapter<'a> {
    pub fn new(kis: &'a KIS) -> Self {
        KISChartAdapter { kis }
    }
}

impl KISChartAdapter<'_> {
    async fn get_domestic_chart(
        &self,
        date: NaiveDate,
        asset_id: &AssetId
    ) -> Result<Chart, Box<dyn Error>>
    {
        let start_date = date - QUERY_CHART_DATE_LEFT_OFFSET;
        let end_date = date + QUERY_CHART_DATE_RIGHT_OFFSET;

        let chart = self.kis.domestic
            .inquire_daily_stock_price(&asset_id.1, start_date, end_date)
            .await?
            .into_iter()
            .map(|(date, candle)| (date, BigDecimal::from(candle.종가)));

        Ok(Chart::from_iter(chart))
    }

    async fn get_overseas_chart(
        &self,
        date: NaiveDate,
        asset_id: &AssetId
    ) -> Result<Chart, Box<dyn Error>>
    {
        let start_date = date - QUERY_CHART_DATE_LEFT_OFFSET;
        let end_date = date + QUERY_CHART_DATE_RIGHT_OFFSET;

        let chart = self.kis.overseas
            .inquire_daily_stock_price(&asset_id.1, asset_id.0.try_into()?, start_date, end_date)
            .await?
            .into_iter()
            .map(|(date, candle)| (date, BigDecimal::from_f64(candle.종가).unwrap()));

        Ok(Chart::from_iter(chart))
    }
}

impl<'a> ChartRecordListSource for KISChartAdapter<'a> {
    async fn get_chart_record_list(
        &self,
        date: NaiveDate,
        asset_id: &AssetId
    ) -> Result<Chart, Box<dyn Error>> {
        if asset_id.is_domestic() {
            self.get_domestic_chart(date, asset_id).await
        } else {
            self.get_overseas_chart(date, asset_id).await
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::time_machine::asset::MarketCode;
    use super::*;

    #[tokio::test]
    async fn test_get_chart_record_list() {
        let kis = KIS::new().await.unwrap();
        let adapter = KISChartAdapter::new(&kis);

        let date = NaiveDate::from_ymd_opt(2021, 1, 1).unwrap();
        let asset_id = AssetId(MarketCode::AMS, "VT".to_string());

        let chart = adapter.get_chart_record_list(date, &asset_id).await.unwrap();
        assert_eq!(chart.data.len(), 74);
    }

    #[tokio::test]
    async fn test_get_chart_record_list_domestic() {
        let kis = KIS::new().await.unwrap();
        let adapter = KISChartAdapter::new(&kis);

        let date = NaiveDate::from_ymd_opt(2021, 1, 1).unwrap();
        let asset_id = AssetId(MarketCode::KOSPI, "005930".to_string());

        let chart = adapter.get_chart_record_list(date, &asset_id).await.unwrap();
        assert_eq!(chart.data.len(), 73);
    }

    #[tokio::test]
    async fn test_fail_get_chart_record_list() {
        let kis = KIS::new().await.unwrap();
        let adapter = KISChartAdapter::new(&kis);

        let date = NaiveDate::from_ymd_opt(2021, 1, 1).unwrap();
        let asset_id = AssetId(MarketCode::KOSPI, "000000".to_string());

        let result = adapter.get_chart_record_list(date, &asset_id).await;
        assert!(result.is_err());
    }
}