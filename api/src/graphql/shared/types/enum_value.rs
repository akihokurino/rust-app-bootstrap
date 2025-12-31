use app::domain;
use async_graphql::{Object, OutputType};
use std::fmt::Display;

macro_rules! impl_enum_value {
    ($name:ident, $domain_type:ty, $graphql_name:literal) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            value: $domain_type,
        }

        impl From<$domain_type> for $name {
            fn from(value: $domain_type) -> Self {
                Self { value }
            }
        }

        #[Object(name = $graphql_name)]
        impl $name
        where
            $domain_type: Clone + Display + OutputType + Send + Sync,
        {
            async fn value(&self) -> $domain_type {
                self.value.clone()
            }

            async fn label(&self) -> String {
                self.value.to_string()
            }
        }
    };
}

impl_enum_value!(Gender, domain::user::Gender, "GenderValue");
