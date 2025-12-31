use async_graphql::Enum;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, EnumString, Enum)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ImageSize {
    Large,
    Medium,
    Small,
}

impl ImageSize {
    pub fn width(&self) -> u32 {
        match self {
            ImageSize::Large => 1200,
            ImageSize::Medium => 600,
            ImageSize::Small => 300,
        }
    }

    pub fn query_param(&self) -> String {
        format!("w={}", self.width())
    }
}
