use sea_orm::entity::prelude::*;
use sea_orm::Set;
use serde::{Deserialize, Serialize};

use crate::domain::order::detail::Detail;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "order_details")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub order_id: String,
    pub product_name: String,
    pub quantity: i32,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl TryFrom<Model> for Detail {
    type Error = String;
    fn try_from(v: Model) -> Result<Self, Self::Error> {
        Ok(Self {
            id: v.id.into(),
            order_id: v.order_id.into(),
            product_name: v.product_name.try_into()?,
            quantity: v.quantity as u32,
            created_at: v.created_at.into(),
            updated_at: v.updated_at.into(),
        })
    }
}

impl From<Detail> for ActiveModel {
    fn from(v: Detail) -> Self {
        Self {
            id: Set(v.id.into()),
            order_id: Set(v.order_id.into()),
            product_name: Set(v.product_name.into()),
            quantity: Set(v.quantity as i32),
            created_at: Set(v.created_at.into()),
            updated_at: Set(v.updated_at.into()),
        }
    }
}
