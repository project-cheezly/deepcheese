use chrono::NaiveDate;
use crate::time_machine::{CategoryId, TrType};
use crate::time_machine::currency::{CurrencyType, CurrencyValue};
use crate::time_machine::record_list::DateAccessor;

pub mod adapter;
pub mod balance;

pub type BankingId = i32;

#[derive(Debug, Clone)]
pub struct BankRecord {
    pub date: NaiveDate,
    pub category_id: CategoryId,
    pub currency_type: CurrencyType,
    pub tr_type: TrType,
    pub value: CurrencyValue
}

impl DateAccessor for BankRecord {
    fn get_date(&self) -> NaiveDate {
        self.date
    }
}