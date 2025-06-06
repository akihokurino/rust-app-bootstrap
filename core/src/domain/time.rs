use chrono::{Local, NaiveDate, Offset, TimeZone};

pub trait ParseFromRfc3339<T> {
    fn parse_from_rfc3339(s: &str) -> Result<T, String>;
}

pub type Date = NaiveDate;
impl ParseFromRfc3339<Self> for Date {
    fn parse_from_rfc3339(s: &str) -> Result<Self, String> {
        Date::parse_from_str(s, "%Y-%m-%d").map_err(|e| e.to_string())
    }
}

pub type LocalDateTime = chrono::DateTime<Local>;
impl ParseFromRfc3339<Self> for LocalDateTime {
    fn parse_from_rfc3339(s: &str) -> Result<Self, String> {
        chrono::DateTime::parse_from_rfc3339(s)
            .map_err(|e| e.to_string())
            .map(|dt| {
                Local
                    .from_local_datetime(&(dt.naive_utc() + Local::now().offset().fix()))
                    .unwrap()
            })
    }
}

pub fn now() -> LocalDateTime {
    Local::now()
}
