use sea_orm::entity::prelude::*;
use sea_orm::Set;
use serde::{Deserialize, Serialize};

use crate::domain::order::Order;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "orders")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub user_id: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl TryFrom<Model> for Order {
    type Error = String;
    fn try_from(v: Model) -> Result<Self, Self::Error> {
        Ok(Self {
            id: v.id.into(),
            user_id: v.user_id.into(),
            created_at: v.created_at.into(),
            updated_at: v.updated_at.into(),
        })
    }
}

impl From<Order> for ActiveModel {
    fn from(v: Order) -> Self {
        Self {
            id: Set(v.id.into()),
            user_id: Set(v.user_id.into()),
            created_at: Set(v.created_at.into()),
            updated_at: Set(v.updated_at.into()),
        }
    }
}
