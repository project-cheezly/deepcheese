pub mod adapter;
pub mod service;

use std::collections::BTreeMap;
use chrono::NaiveDate;
use sqlx::types::BigDecimal;
use crate::time_machine::currency::CurrencyValue;

#[derive(Debug, Clone)]
pub struct Chart {
    data: BTreeMap<NaiveDate, CurrencyValue>
}

impl Default for Chart {
    fn default() -> Self {
        Self::new()
    }
}

impl Chart {
    pub fn new() -> Self {
        Self {
            data: BTreeMap::new()
        }
    }

    pub fn get(&self, date: NaiveDate) -> Option<&CurrencyValue> {
        self.data.get(&date)
    }

    pub fn get_nearest(&self, date: NaiveDate) -> Option<CurrencyValue> {
        self.data.range(..=date).next_back().map(|(_, v)| v.clone())
    }

    pub fn concat(&mut self, other: Self) {
        self.data.extend(other.data);
    }

    pub fn get_initial_date(&self) -> Option<NaiveDate> {
        self.data.keys().next().cloned()
    }

    pub fn get_final_date(&self) -> Option<NaiveDate> {
        self.data.keys().next_back().cloned()
    }
}

impl FromIterator<(NaiveDate, BigDecimal)> for Chart {
    fn from_iter<T: IntoIterator<Item=(NaiveDate, BigDecimal)>>(iter: T) -> Self {
        Self {
            data: BTreeMap::<NaiveDate, CurrencyValue>::from_iter(iter)
        }
    }
}
