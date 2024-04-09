use std::str::FromStr;
use chrono::NaiveDate;
use sqlx::{Executor, PgPool, Row};
use sqlx::postgres::PgRow;
use crate::time_machine::asset::{AssetId, MarketCode};
use crate::time_machine::{CategoryId, TrType};
use crate::time_machine::banking::BankingId;
use crate::time_machine::record_list::RecordList;
use crate::time_machine::tr_record::TrRecord;
use crate::time_machine::tr_record::adapter::TrRecordListSource;

fn parse_tr_record(row: PgRow) -> Result<TrRecord, Box<dyn std::error::Error>> {
    let date = row.get::<NaiveDate, _>("date");
    let category_id = row.get::<CategoryId, _>(1);
    let asset_id = AssetId(
        MarketCode::from_str(&row.get::<String, _>(2))?,
        row.get::<String, _>(3)
    );
    let tr_type = TrType::from_str(&row.get::<String, _>(4))?;
    let amount = row.get::<i32, _>(5);
    let banking_id = row.get::<Option<BankingId>, _>(6);

    Ok(TrRecord {
        date,
        category_id,
        asset_id,
        tr_type,
        amount,
        banking_id
    })
}

pub struct TrRecordListDatabaseAdapter<'a> {
    conn: &'a PgPool
}

impl<'a> TrRecordListDatabaseAdapter<'a> {
    pub fn new(conn: &'a PgPool) -> Self {
        TrRecordListDatabaseAdapter { conn }
    }
}

impl TrRecordListSource for TrRecordListDatabaseAdapter<'_> {
    async fn get_tr_record_list(&self) -> Result<RecordList<TrRecord>, Box<dyn std::error::Error>> {
        let query = include_str!("../../sql/load_entire_tr_record.sql");

        let mut conn = self.conn.acquire().await?;

        let result = conn
            .fetch_all(query)
            .await?
            .into_iter()
            .map(parse_tr_record)
            .filter_map(Result::ok)
            .collect::<Vec<_>>();

        Ok(RecordList::new(result))
    }
}

pub struct TrRecordDatabaseAdapter<'a> {
    conn: &'a PgPool,
    tr_id: i32
}

impl<'a> TrRecordDatabaseAdapter<'a> {
    pub fn new(conn: &'a PgPool, tr_id: i32) -> Self {
        TrRecordDatabaseAdapter { conn, tr_id }
    }
}

impl TrRecordListSource for TrRecordDatabaseAdapter<'_> {
    async fn get_tr_record_list(&self) -> Result<RecordList<TrRecord>, Box<dyn std::error::Error>> {
        let query_string = include_str!("../../sql/load_tr_record_by_id.sql");

        let mut conn = self.conn.acquire().await?;

        let result = conn
            .fetch_all(sqlx::query(query_string).bind(self.tr_id))
            .await?
            .into_iter()
            .map(parse_tr_record)
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
    async fn test_get_tr_record_list() -> Result<(), Box<dyn std::error::Error>>{
        let pool = connect_to_database().await.expect("Failed to connect to database");
        let adapter = TrRecordListDatabaseAdapter::new(&pool);
        let _ = adapter.get_tr_record_list().await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_get_tr_record_list_with_id() -> Result<(), Box<dyn std::error::Error>>{
        let pool = connect_to_database().await.expect("Failed to connect to database");
        let adapter = TrRecordDatabaseAdapter::new(&pool, 16);
        let _ = adapter.get_tr_record_list().await?;

        Ok(())
    }
}