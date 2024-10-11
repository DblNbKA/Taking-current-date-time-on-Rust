use std::thread::sleep;
use std::time::Duration;
use chrono::{Datelike, Timelike, Utc};

fn main() {
        let current = Utc::now();
        println!("{}:{}:{}\n{}:{}:{}\n", current.day(), current.month(), current.year(),
                                        current.hour(), current.minute(), current.second());
        sleep(Duration::from_secs(1));
}
