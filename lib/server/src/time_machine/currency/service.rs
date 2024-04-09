use std::collections::HashMap;
use chrono::NaiveDate;
use crate::time_machine::currency::adapter::kis_adapter::KISCurrencyChartAdapter;
use crate::time_machine::chart::Chart;
use crate::time_machine::currency::{CurrencyType, CurrencyValue};
use crate::time_machine::currency::adapter::CurrencyChartRecordListSource;

struct QueryHistory {
    data: HashMap<CurrencyType, NaiveDate>
}

impl QueryHistory {
    pub fn new()  -> Self { QueryHistory { data: HashMap::new() } }

    pub fn is_query_needed(&self, currency_type: &CurrencyType, date: NaiveDate) -> bool {
        match self.data.get(&currency_type) {
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

    pub fn update_query_date(&mut self, currency_type: &CurrencyType, mut date: NaiveDate) {
        let entry = self.data.entry(*currency_type).or_insert(date);
        *entry = *entry.max(&mut date);
    }
}

struct ChartList {
    data: HashMap<CurrencyType, Chart>
}

impl ChartList {
    pub fn new() -> Self { ChartList { data: HashMap::new() } }

    pub fn get(&mut self, currency_type: &CurrencyType) -> &Chart {
        if !self.data.contains_key(currency_type) {
            self.data.insert(*currency_type, Chart::new());
        }

        self.data.get(currency_type).unwrap()
    }

    pub fn insert(&mut self, currency_type: &CurrencyType, chart: Chart) {
        if !self.data.contains_key(&currency_type) {
            self.data.insert(*currency_type, chart);
        } else {
            self.data.get_mut(&currency_type).unwrap().concat(chart);
        }
    }
}

pub struct CurrencyChartService<'a> {
    query_history: QueryHistory,
    kis_adapter: KISCurrencyChartAdapter<'a>,
    charts: ChartList
}

impl<'a> CurrencyChartService<'a> {
    pub fn new(kis_adapter: KISCurrencyChartAdapter<'a>) -> Self {
        CurrencyChartService {
            query_history: QueryHistory::new(),
            kis_adapter,
            charts: ChartList::new()
        }
    }

    pub async fn get_currency_chart(
        &mut self,
        date: NaiveDate,
        currency_type: &CurrencyType
    ) -> Option<CurrencyValue> {
        if currency_type == &CurrencyType::KRW {
            return Some(CurrencyValue::from(1));
        }

        if !self.query_history.is_query_needed(currency_type, date) {
            return self.charts.get(currency_type).get_nearest(date);
        }

        let chart = self.kis_adapter
            .get_currency_chart_record_list(date, currency_type)
            .await;

        if let Ok(chart) = chart {
            if let Some(final_date) = chart.get_final_date() {
                self.query_history.update_query_date(currency_type, final_date);
            }
            self.charts.insert(currency_type, chart);
        } else {
            self.query_history
                .update_query_date(
                    currency_type,
                    NaiveDate::from_ymd_opt(2999, 12, 31).unwrap()
                );
        }

        self.charts.get(currency_type).get_nearest(date)
    }
}