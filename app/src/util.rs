use crate::domain::types::time::{LocalDateTime, now};

pub fn duration_date_string(date: LocalDateTime) -> String {
    let duration = now().signed_duration_since(date);

    if duration.num_seconds() < 60 {
        "たった今".to_string()
    } else if duration.num_minutes() < 60 {
        format!("{}分前", duration.num_minutes())
    } else if duration.num_hours() < 24 {
        format!("{}時間前", duration.num_hours())
    } else {
        date.format("%Y/%m/%d").to_string()
    }
}
