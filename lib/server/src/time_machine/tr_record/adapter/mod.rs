use crate::time_machine::record_list::RecordList;
use crate::time_machine::tr_record::TrRecord;

pub mod database;

pub trait TrRecordListSource {
    async fn get_tr_record_list(&self) -> Result<RecordList<TrRecord>, Box<dyn std::error::Error>>;
}