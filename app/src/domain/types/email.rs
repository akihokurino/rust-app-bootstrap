use crate::domain::types::string::FromUnchecked;
use derive_more::{AsRef, Display, Into};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, Into, Display, AsRef,
)]
pub struct Email(String);
impl TryFrom<String> for Email {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !email_address::EmailAddress::is_valid(&value) {
            return Err("不正なメールアドレスです".into());
        }
        Ok(Email(value))
    }
}
impl FromUnchecked<String> for Email {
    fn from_unchecked(value: String) -> Self {
        Self(value)
    }
}
