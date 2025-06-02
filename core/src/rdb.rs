pub mod session_manager;
pub mod types;

use chrono::{DateTime, Local, Utc};

#[derive(Debug, Clone)]
pub struct UtcDateTime(pub DateTime<Utc>);
impl From<DateTime<Local>> for UtcDateTime {
    fn from(local: DateTime<Local>) -> Self {
        UtcDateTime(local.with_timezone(&Utc))
    }
}
impl sqlx::Type<sqlx::Postgres> for UtcDateTime {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <DateTime<Utc> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for UtcDateTime {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let dt = <DateTime<Utc> as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Ok(UtcDateTime(dt))
    }
}
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for UtcDateTime {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.0.encode_by_ref(buf)
    }
}

impl From<UtcDateTime> for DateTime<Local> {
    fn from(utc: UtcDateTime) -> Self {
        utc.0.with_timezone(&Local)
    }
}
