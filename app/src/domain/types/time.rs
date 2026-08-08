use chrono::{Datelike, FixedOffset, Local, NaiveDate, Offset, TimeZone};
use std::fmt::Display;

pub trait ParseFromRfc3339<T> {
    fn parse_from_rfc3339(s: &str) -> Result<T, String>;
}
pub trait ToRfc3339 {
    fn to_rfc3339(&self) -> String;
}
pub trait FromTimestamp<T> {
    fn from_timestamp(timestamp: i64) -> Result<T, String>;
}
pub trait ToTimestamp {
    fn to_timestamp(&self) -> i64;
}

pub const JST_OFFSET: i32 = 9 * 3600;

pub type Date = NaiveDate;
impl ParseFromRfc3339<Self> for Date {
    fn parse_from_rfc3339(s: &str) -> Result<Self, String> {
        Date::parse_from_str(s, "%Y-%m-%d").map_err(|e| e.to_string())
    }
}
impl ToRfc3339 for Date {
    fn to_rfc3339(&self) -> String {
        self.format("%Y-%m-%d").to_string()
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
impl FromTimestamp<Self> for LocalDateTime {
    fn from_timestamp(timestamp: i64) -> Result<Self, String> {
        Local
            .timestamp_opt(timestamp, 0)
            .single()
            .ok_or_else(|| "Invalid timestamp".to_string())
    }
}
impl ToTimestamp for LocalDateTime {
    fn to_timestamp(&self) -> i64 {
        self.timestamp()
    }
}

pub trait LocalDateTimeExt {
    fn to_jst(&self) -> chrono::DateTime<FixedOffset>;
}
impl LocalDateTimeExt for LocalDateTime {
    fn to_jst(&self) -> chrono::DateTime<FixedOffset> {
        let jst = FixedOffset::east_opt(JST_OFFSET).unwrap();
        self.with_timezone(&jst)
    }
}

pub fn now() -> LocalDateTime {
    Local::now()
}

#[derive(Debug, Clone)]
pub struct YM {
    pub year: i32,
    pub month: u32,
}
impl TryFrom<&str> for YM {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 {
            return Err("Invalid YM format".to_string());
        }
        let year = parts[0]
            .parse::<i32>()
            .map_err(|e| format!("Invalid year: {}", e))?;
        let month = parts[1]
            .parse::<u32>()
            .map_err(|e| format!("Invalid month: {}", e))?;
        if month < 1 || month > 12 {
            return Err("Month must be between 1 and 12".to_string());
        }
        Ok(YM { year, month })
    }
}
impl From<YM> for String {
    fn from(v: YM) -> Self {
        format!("{:04}-{:02}", v.year, v.month)
    }
}
impl From<YM> for Date {
    fn from(v: YM) -> Self {
        Date::from_ymd_opt(v.year, v.month, 1).unwrap()
    }
}
impl Display for YM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}-{:02}", self.year, self.month)
    }
}
impl From<Date> for YM {
    fn from(date: Date) -> Self {
        YM {
            year: date.year(),
            month: date.month(),
        }
    }
}
