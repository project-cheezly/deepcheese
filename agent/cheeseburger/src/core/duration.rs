use chrono::{Local, NaiveTime};
use std::time::Duration;

pub fn get_duration(time: NaiveTime) -> Duration {
    let now = Local::now();
    let target = now.with_time(time).unwrap();

    (target - now).to_std().unwrap_or_else(|e| {
        log::warn!("Failed to get duration: {}", e);
        Duration::from_secs(0)
    })
}
