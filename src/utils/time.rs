use crate::chrono::{Local, NaiveDate};

pub fn now_date() -> NaiveDate {
    Local::now().date_naive()
}
