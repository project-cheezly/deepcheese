use std::error::Error;
use std::str::FromStr;
use chrono::NaiveDate;
use sqlx::postgres::PgRow;
use sqlx::{Executor, PgPool, Row};
use crate::time_machine::banking::{BankingId, BankRecord};
use crate::time_machine::{CategoryId, TrType};
use crate::time_machine::banking::adapter::BankRecordListSource;
use crate::time_machine::currency::{CurrencyType, CurrencyValue};
use crate::time_machine::record_list::RecordList;

fn parse_bank_record(row: PgRow) -> Result<BankRecord, Box<dyn std::error::Error>> {
    let date = row.get::<NaiveDate, _>("date");
    let category_id = row.get::<CategoryId, _>("category_id");
    let currency_type = CurrencyType::from_str(&row.get::<String, _>("currency_type"))?;
    let tr_type = TrType::from_str(&row.get::<String, _>("tr_type"))?;
    let value = row.get::<CurrencyValue, _>("value");

    Ok(BankRecord {
        date,
        category_id,
        currency_type,
        tr_type,
        value
    })
}

pub struct BankRecordListDatabaseAdapter<'a> {
    conn: &'a PgPool
}

impl<'a> BankRecordListDatabaseAdapter<'a> {
    pub fn new(conn: &'a PgPool) -> Self {
        BankRecordListDatabaseAdapter { conn }
    }
}

impl BankRecordListSource for BankRecordListDatabaseAdapter<'_> {
    async fn get_banking_record_list(&self) -> Result<RecordList<BankRecord>, Box<dyn Error>> {
        let query = include_str!("../../sql/load_entire_bank_record.sql");

        let mut conn = self.conn.acquire().await?;

        let result = conn
            .fetch_all(query)
            .await?
            .into_iter()
            .map(parse_bank_record)
            .filter_map(Result::ok)
            .collect::<Vec<_>>();

        Ok(RecordList::new(result))
    }
}

pub struct BankRecordDatabaseAdapter<'a> {
    conn: &'a PgPool,
    banking_id: BankingId
}

impl<'a> BankRecordDatabaseAdapter<'a> {
    pub fn new(conn: &'a PgPool, banking_id: BankingId) -> Self {
        BankRecordDatabaseAdapter { conn, banking_id }
    }
}

impl BankRecordListSource for BankRecordDatabaseAdapter<'_> {
    async fn get_banking_record_list(&self) -> Result<RecordList<BankRecord>, Box<dyn Error>> {
        let query_string = include_str!("../../sql/load_bank_record_by_id.sql");

        let mut conn = self.conn.acquire().await?;

        let result = conn
            .fetch_all(sqlx::query(query_string).bind(self.banking_id))
            .await?
            .into_iter()
            .map(parse_bank_record)
            .filter_map(Result::ok)
            .collect::<Vec<_>>();

        Ok(RecordList::new(result))
    }
}

#[cfg(test)]
mod tests {
    use crate::database::connect_to_database;
    use super::*;

    #[tokio::test]
    async fn test_get_banking_record_list() {
        let pool = connect_to_database().await.unwrap();

        let adapter = BankRecordListDatabaseAdapter::new(&pool);
        let _ = adapter.get_banking_record_list().await.unwrap();

        pool.close().await;
    }

    #[tokio::test]
    async fn test_get_banking_record_by_id() {
        let pool = connect_to_database().await.unwrap();

        let adapter = BankRecordDatabaseAdapter::new(&pool, 17);
        let _ = adapter.get_banking_record_list().await.unwrap();

        pool.close().await;
    }
}