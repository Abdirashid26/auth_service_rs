use chrono::{DateTime, Datelike, NaiveDate, Utc};
use time::{Date, Month, OffsetDateTime};
use time::format_description::well_known::Rfc3339;

pub fn convert_naivedate_to_timedate(naive_date: NaiveDate) -> Date {
    let (year, month, day) = (naive_date.year(), naive_date.month(), naive_date.day());

    let month_enum = Month::try_from(month as u8).expect("Invalid month value");

    Date::from_calendar_date(year, month_enum, day as u8).unwrap()
}

pub fn convert_timedate_to_naivedate(time_date: Date) -> NaiveDate {
    NaiveDate::from_ymd_opt(time_date.year(), time_date.month() as u32, time_date.day() as u32).unwrap()
}


pub fn convert_time_to_chrono(time: OffsetDateTime) -> DateTime<Utc> {
    let formatted = time.format(&Rfc3339).unwrap(); // Convert to RFC3339 string
    DateTime::parse_from_rfc3339(&formatted).unwrap().into()
}