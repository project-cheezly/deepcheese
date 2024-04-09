pub mod adapter;

use chrono::NaiveDate;
use crate::time_machine::{CategoryId, TrType};
use crate::time_machine::asset::{AssetAmount, AssetId};
use crate::time_machine::banking::BankingId;
use crate::time_machine::record_list::DateAccessor;

#[derive(Debug, Clone)]
pub struct TrRecord {
    pub date: NaiveDate,
    pub category_id: CategoryId,
    pub asset_id: AssetId,
    pub tr_type: TrType,
    pub amount: AssetAmount,
    pub banking_id: Option<BankingId>
}

impl DateAccessor for TrRecord {
    fn get_date(&self) -> NaiveDate {
        self.date
    }
}
