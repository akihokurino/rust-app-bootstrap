use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, SimpleObject};
use async_graphql_value::ConstValue;
use app::domain;
use app::domain::time::ParseFromRfc3339;
use derive_more::{From, Into};

#[derive(SimpleObject)]
pub struct BoolPayload {
    pub is_ok: bool,
}
impl From<bool> for BoolPayload {
    fn from(v: bool) -> Self {
        Self { is_ok: v }
    }
}

#[derive(Clone, Debug, From, Into)]
pub struct Date(pub domain::time::Date);
#[Scalar]
impl ScalarType for Date {
    fn parse(value: ConstValue) -> InputValueResult<Self> {
        if let ConstValue::String(v) = value {
            Ok(Self(domain::time::Date::parse_from_rfc3339(&v)?))
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> ConstValue {
        ConstValue::String(self.0.format("%Y-%m-%d").to_string())
    }
}

#[derive(Clone, Debug, From, Into)]
pub struct DateTime(pub domain::time::LocalDateTime);
#[Scalar]
impl ScalarType for DateTime {
    fn parse(value: ConstValue) -> InputValueResult<Self> {
        if let ConstValue::String(v) = value {
            Ok(Self(domain::time::LocalDateTime::parse_from_rfc3339(&v)?))
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> ConstValue {
        ConstValue::String(self.0.to_rfc3339())
    }
}
