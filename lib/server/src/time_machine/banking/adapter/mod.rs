pub mod database;

use crate::time_machine::banking::BankRecord;
use crate::time_machine::record_list::RecordList;

pub trait BankRecordListSource {
    async fn get_banking_record_list(&self) -> Result<RecordList<BankRecord>, Box<dyn std::error::Error>>;
}