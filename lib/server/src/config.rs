use chrono::{DateTime, Timelike, Utc};

const START_DOMESTIC_HOUR: u32 = 0;
const END_DOMESTIC_HOUR: u32 = 7;

const START_FOREIGN_HOUR: u32 = 13;
const END_FOREIGN_HOUR: u32 = 22;

#[derive(Debug)]
pub struct DomesticMarket { }

impl DomesticMarket {
    pub fn is_open(date_time: DateTime<Utc>) -> bool {
        let hour = date_time.hour();
        hour >= START_DOMESTIC_HOUR && hour < END_DOMESTIC_HOUR
    }
}

#[derive(Debug)]
pub struct OverseasMarket { }

impl OverseasMarket {
    pub fn is_open(date_time: DateTime<Utc>) -> bool {
        let hour = date_time.hour();
        hour >= START_FOREIGN_HOUR || hour < END_FOREIGN_HOUR
    }
}