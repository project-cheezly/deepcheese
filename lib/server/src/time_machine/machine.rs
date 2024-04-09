use chrono::Duration;
use kis::KIS;
use sqlx::{PgPool, QueryBuilder};
use crate::time_machine::asset::balance::AssetBalance;
use crate::time_machine::banking::adapter::BankRecordListSource;
use crate::time_machine::banking::adapter::database::BankRecordDatabaseAdapter;
use crate::time_machine::banking::balance::BankBalance;
use crate::time_machine::chart::adapter::kis_adapter::KISChartAdapter;
use crate::time_machine::chart::service::ChartService;
use crate::time_machine::currency::adapter::kis_adapter::KISCurrencyChartAdapter;
use crate::time_machine::currency::CurrencyValue;
use crate::time_machine::currency::service::CurrencyChartService;
use crate::time_machine::tr_record::adapter::database::TrRecordDatabaseAdapter;
use crate::time_machine::tr_record::adapter::TrRecordListSource;
use crate::time_machine::tr_record::TrRecord;

const INSERT_CATEGORY_HISTORY_QUERY: &str = r#"
    INSERT INTO category_history (tr_date, category_id, value)
"#;

const INSERT_CATEGORY_HISTORY_CONFLICT_QUERY: &str = r#"
    ON CONFLICT (tr_date, category_id) DO UPDATE SET value = category_history.value + EXCLUDED.value
"#;

pub struct Machine<'a> {
    currency_service: CurrencyChartService<'a>,
    chart_service: ChartService<'a>,
    pool: &'a PgPool
}

impl<'a> Machine<'a> {
    pub async fn new(kis: &'a KIS, pool: &'a PgPool) -> Self {
        let currency_chart_adapter = KISCurrencyChartAdapter::new(kis);
        let currency_service = CurrencyChartService::new(currency_chart_adapter);

        let chart_adapter = KISChartAdapter::new(kis);
        let chart_service = ChartService::new(chart_adapter);

        Machine {
            currency_service,
            chart_service,
            pool
        }
    }

    pub async fn run_in_place(&mut self, tr_id: i32) -> Result<(), Box<dyn std::error::Error>> {
        let (start_date, tr_record) = TrRecordDatabaseAdapter::new(self.pool, tr_id)
            .get_tr_record_list()
            .await?
            .into_iter()
            .take(1)
            .next()
            .ok_or("Tr record not found")?;

        let tr_record = tr_record.get(0).ok_or("Tr record not found")?;

        let bank_record_id = tr_record.banking_id.ok_or("Banking id is not set")?;

        let (_, bank_record) = BankRecordDatabaseAdapter::new(self.pool, bank_record_id)
            .get_banking_record_list()
            .await?
            .into_iter()
            .take(1)
            .next()
            .ok_or("Bank record not found")?;

        let bank_record = bank_record.get(0).ok_or("Bank record not found")?;

        let category_id = &tr_record.category_id;

        let asset_amount = tr_record.tr_type.get_signed_value(tr_record.amount);
        let asset_id  = &tr_record.asset_id;

        let bank_amount = &bank_record.currency_type;
        let bank_value = &(CurrencyValue::from(bank_record.tr_type.get_signed_value(1)) * &bank_record.value);

        let end_date = chrono::Utc::now().date_naive();

        let mut query_parameters = Vec::new();
        let mut crit_date = start_date;

        while crit_date < end_date {
            let target_currency = self.currency_service
                .get_currency_chart(crit_date, bank_amount)
                .await
                .unwrap_or_else(|| CurrencyValue::from(1));
            let unit_asset_value = self.chart_service.get_value(crit_date, asset_id).await;

            let asset_value = unit_asset_value.unwrap_or_default() * CurrencyValue::from(asset_amount) * &target_currency;
            let bank_value = bank_value * &target_currency;

            dbg!(&crit_date, &asset_value, &bank_value);

            query_parameters.push((crit_date, asset_value + bank_value));

            crit_date += Duration::days(1);
        }

        QueryBuilder::new(INSERT_CATEGORY_HISTORY_QUERY)
            .push_values(query_parameters, |mut b, record| {
                b.push_bind(record.0)
                    .push_bind(category_id)
                    .push_bind(record.1);
            })
            .push(INSERT_CATEGORY_HISTORY_CONFLICT_QUERY)
            .build()
            .execute(self.pool)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use kis::KIS;
    use sqlx::PgPool;
    use std::collections::HashMap;
    use crate::database::connect_to_database;

    #[tokio::test]
    async fn test_machine() {
        let kis = KIS::new().await.unwrap();
        let pool = connect_to_database().await.unwrap();
        {
            let mut machine = Machine::new(&kis, &pool).await;

            for i in 15..19 {
                machine.run_in_place(i).await.unwrap();
            }
        }

    }
}