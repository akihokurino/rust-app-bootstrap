pub mod enum_value;

use app::domain::types;
use app::domain::types::time::ParseFromRfc3339;
use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, SimpleObject};
use async_graphql_value::ConstValue;
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
pub struct Date(pub types::time::Date);
#[Scalar]
impl ScalarType for Date {
    fn parse(value: ConstValue) -> InputValueResult<Self> {
        if let ConstValue::String(v) = value {
            Ok(Self(types::time::Date::parse_from_rfc3339(&v)?))
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> ConstValue {
        ConstValue::String(self.0.format("%Y-%m-%d").to_string())
    }
}

#[derive(Clone, Debug, From, Into)]
pub struct DateTime(pub types::time::LocalDateTime);
#[Scalar]
impl ScalarType for DateTime {
    fn parse(value: ConstValue) -> InputValueResult<Self> {
        if let ConstValue::String(v) = value {
            Ok(Self(types::time::LocalDateTime::parse_from_rfc3339(&v)?))
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> ConstValue {
        ConstValue::String(self.0.to_rfc3339())
    }
}
