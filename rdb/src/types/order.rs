use crate::macros::repository::{
    impl_repository, impl_repository_delete, impl_repository_insert, impl_repository_update,
};
use crate::schema::orders;
use diesel::prelude::*;
use domain::errors::Kind::Internal;
use domain::models::order::{Id, Order};
use domain::models::time::LocalDateTime;
use domain::AppResult;

#[derive(Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = orders)]
pub struct OrderModel {
    pub id: String,
    pub user_id: String,
    pub created_at: LocalDateTime,
    pub updated_at: LocalDateTime,
}
impl TryFrom<OrderModel> for Order {
    type Error = String;
    fn try_from(v: OrderModel) -> Result<Self, Self::Error> {
        Ok(Self {
            id: domain::models::order::Id::from(v.id),
            user_id: domain::models::user::Id::from(v.user_id),
            created_at: v.created_at,
            updated_at: v.updated_at,
        })
    }
}
impl Into<OrderModel> for Order {
    fn into(self) -> OrderModel {
        OrderModel {
            id: self.id.into(),
            user_id: self.user_id.into(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl_repository!(
    OrderRepository,
    orders::table,
    OrderModel,
    Order,
    Id,
    orders::id
);
impl_repository_insert!(OrderRepository, orders::table, OrderModel, Order);
impl_repository_update!(
    OrderRepository,
    orders::table,
    OrderModel,
    Order,
    orders::id
);
impl_repository_delete!(OrderRepository, orders::table, Id, orders::id);
impl OrderRepository {}
