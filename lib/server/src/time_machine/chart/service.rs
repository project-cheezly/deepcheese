use std::collections::HashMap;
use chrono::NaiveDate;
use crate::time_machine::asset::AssetId;
use crate::time_machine::chart::adapter::ChartRecordListSource;
use crate::time_machine::chart::adapter::kis_adapter::KISChartAdapter;
use crate::time_machine::chart::Chart;
use crate::time_machine::currency::CurrencyValue;

struct QueryHistory {
    data: HashMap<AssetId, NaiveDate>
}

impl QueryHistory {
    pub fn new() -> Self {
        QueryHistory { data: HashMap::new() }
    }

    pub fn is_query_needed(&self, asset_id: &AssetId, date: NaiveDate) -> bool {
        match self.data.get(&asset_id) {
            Some(last_query_date) => {
                if date > *last_query_date {
                    true
                } else {
                    false
                }
            },
            None => true
        }
    }

    pub fn update_query_date(&mut self, asset_id: &AssetId, mut date: NaiveDate) {
        let entry = self.data.entry(asset_id.clone()).or_insert(date);
        *entry = *entry.max(&mut date);
    }
}

struct ChartList {
    data: HashMap<AssetId, Chart>
}

impl ChartList {
    pub fn new() -> Self {
        ChartList { data: HashMap::new() }
    }

    pub fn get(&mut self, asset_id: &AssetId) -> &Chart {
        if !self.data.contains_key(asset_id) {
            self.data.insert(asset_id.clone(), Chart::new());
        }

        self.data.get(asset_id).unwrap()
    }

    pub fn insert(&mut self, asset_id: &AssetId, chart: Chart) {
        if !self.data.contains_key(&asset_id) {
            self.data.insert(asset_id.clone(), chart);
        } else {
            self.data.get_mut(&asset_id).unwrap().concat(chart);
        }
    }
}

pub struct ChartService<'a> {
    kis_adapter: KISChartAdapter<'a>,
    charts: ChartList,
    query_history: QueryHistory
}

impl<'a> ChartService<'a> {
    pub fn new(kis_adapter: KISChartAdapter<'a>) -> Self {
        ChartService {
            kis_adapter,
            charts: ChartList::new(),
            query_history: QueryHistory::new()
        }
    }

    pub async fn get_value(
        &mut self,
        date: NaiveDate,
        asset_id: &AssetId
    ) -> Option<CurrencyValue>
    {
        if !self.query_history.is_query_needed(&asset_id, date) {
            dbg!(&self.charts.get(&asset_id).get_nearest(date));
            return self.charts.get(&asset_id).get_nearest(date);
        }

        let chart = self.kis_adapter
            .get_chart_record_list(date, &asset_id)
            .await;

        if let Ok(chart) = chart {
            if let Some(final_date) = chart.get_final_date() {
                self.query_history.update_query_date(&asset_id, final_date);
            }
            dbg!(&chart);
            self.charts.insert(&asset_id, chart);
        } else {
            self.query_history
                .update_query_date(
                    &asset_id,
                    NaiveDate::from_ymd_opt(2999, 12, 31).unwrap()
                );
        }

        self.charts.get(&asset_id).get_nearest(date)
    }
}