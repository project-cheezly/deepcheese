use std::collections::BTreeMap;
use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct RecordList<T> {
    data: BTreeMap<NaiveDate, Vec<T>>
}

pub trait DateAccessor {
    fn get_date(&self) -> NaiveDate;
}

impl<T> IntoIterator for RecordList<T> {
    type Item = (NaiveDate, Vec<T>);
    type IntoIter = std::collections::btree_map::IntoIter<NaiveDate, Vec<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<T> RecordList<T> {
    pub fn new(data: Vec<T>) -> RecordList<T> where T: DateAccessor {
        let mut map = BTreeMap::new();
        for tr in data {
            map.entry(tr.get_date())
                .or_insert_with(Vec::new)
                .push(tr);
        }

        Self { data: map }
    }

    #[allow(dead_code)]
    pub fn get(&self, date: NaiveDate) -> Option<&Vec<T>> {
        self.data.get(&date)
    }

    #[allow(dead_code)]
    pub fn get_initial_date(&self) -> Option<NaiveDate> {
        self.data.first_key_value().map(|(k, _)| *k)
    }
}