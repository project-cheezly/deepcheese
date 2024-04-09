use std::collections::HashMap;
use crate::time_machine::banking::BankRecord;
use crate::time_machine::CategoryId;
use crate::time_machine::currency::{CurrencyType, CurrencyValue};

#[derive(Debug, Clone)]
pub struct BankBalance {
    data: HashMap<CategoryId, HashMap<CurrencyType, CurrencyValue>>
}

impl BankBalance {
    pub fn new() -> Self {
        BankBalance {
            data: HashMap::new()
        }
    }

    pub fn update(&mut self, tr_list: Vec<BankRecord>) {
        for tr in tr_list {
            let category_currency = self.data
                .entry(tr.category_id)
                .or_insert(HashMap::new());

            let currency_value = category_currency
                .entry(tr.currency_type)
                .or_insert(CurrencyValue::default());

            *currency_value += tr.tr_type.get_signed_value(tr.value);
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use sqlx::types::BigDecimal;
    use crate::time_machine::TrType;
    use super::*;

    #[test]
    fn test_update() {
        let test_tr_list: Vec<BankRecord> = vec![
            BankRecord {
                date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                category_id: 1,
                currency_type: CurrencyType::KRW,
                tr_type: TrType::INFLOW,
                value: BigDecimal::from(50)
            },
            BankRecord {
                date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                category_id: 2,
                currency_type: CurrencyType::KRW,
                tr_type: TrType::OUTFLOW,
                value: BigDecimal::from(50)
            },
            BankRecord {
                date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                category_id: 1,
                currency_type: CurrencyType::USD,
                tr_type: TrType::INFLOW,
                value: BigDecimal::from(100)
            }
        ];

        let mut bank_balance = BankBalance::new();
        bank_balance.update(test_tr_list);

        let category_1 = bank_balance.data.get(&1).unwrap();
        assert_eq!(category_1.get(&CurrencyType::KRW).unwrap(), &CurrencyValue::from(50));
        assert_eq!(category_1.get(&CurrencyType::USD).unwrap(), &CurrencyValue::from(100));

        let category_2 = bank_balance.data.get(&2).unwrap();
        assert_eq!(category_2.get(&CurrencyType::KRW).unwrap(), &CurrencyValue::from(-50));
    }
}