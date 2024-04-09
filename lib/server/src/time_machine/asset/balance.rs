use std::collections::HashMap;
use crate::time_machine::asset::{AssetAmount, AssetId};
use crate::time_machine::{CategoryId};
use crate::time_machine::tr_record::TrRecord;

#[derive(Debug, Clone)]
pub struct AssetBalance {
    data: HashMap<CategoryId, HashMap<AssetId, AssetAmount>>
}

impl AssetBalance {
    pub fn new() -> Self {
        AssetBalance {
            data: HashMap::new()
        }
    }

    pub fn update(&mut self, tr_list: Vec<TrRecord>) {
        for tr in tr_list {
            let category_asset = self.data
                .entry(tr.category_id)
                .or_insert(HashMap::new());

            let asset_amount = category_asset
                .entry(tr.asset_id)
                .or_insert(AssetAmount::default());

            *asset_amount += tr.tr_type.get_signed_value(tr.amount);
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use crate::time_machine::asset::{AssetId, MarketCode};
    use crate::time_machine::TrType;
    use super::*;

    #[test]
    fn test_update() {
        let test_tr_list: Vec<TrRecord> = vec![
            TrRecord {
                date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                category_id: 1,
                asset_id: AssetId(MarketCode::KOSPI, "005930".to_string()),
                tr_type: TrType::INFLOW,
                amount: 100,
                banking_id: None
            },
            TrRecord {
                date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                category_id: 1,
                asset_id: AssetId(MarketCode::KOSPI, "005930".to_string()),
                tr_type: TrType::OUTFLOW,
                amount: 50,
                banking_id: None
            },
            TrRecord {
                date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
                category_id: 2,
                asset_id: AssetId(MarketCode::KOSPI, "005935".to_string()),
                tr_type: TrType::INFLOW,
                amount: 200,
                banking_id: None
            }
        ];

        let mut balance = AssetBalance::new();
        balance.update(test_tr_list);

        let category_1 = balance.data.get(&1).unwrap();
        assert_eq!(category_1.get(&AssetId(MarketCode::KOSPI, "005930".to_string())).unwrap(), &50);
        assert_eq!(category_1.len(), 1);

        let category_2 = balance.data.get(&2).unwrap();
        assert_eq!(category_2.get(&AssetId(MarketCode::KOSPI, "005935".to_string())).unwrap(), &200);
    }
}