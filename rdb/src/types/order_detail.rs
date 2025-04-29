use crate::macros::repository::{impl_repository, impl_repository_insert};
use crate::schema::order_details;
use diesel::prelude::*;
use domain::errors::Kind::Internal;
use domain::models::order::detail::{Detail, Id};
use domain::models::time::LocalDateTime;
use domain::AppResult;

#[derive(Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = order_details)]
struct OrderDetailModel {
    pub id: String,
    pub order_id: String,
    pub product_name: String,
    pub quantity: i32,
    pub created_at: LocalDateTime,
    pub updated_at: LocalDateTime,
}
impl TryFrom<OrderDetailModel> for Detail {
    type Error = String;
    fn try_from(v: OrderDetailModel) -> Result<Self, Self::Error> {
        Ok(Self {
            id: domain::models::order::detail::Id::from(v.id),
            order_id: domain::models::order::Id::from(v.order_id),
            product_name: v.product_name.try_into()?,
            quantity: v.quantity as u32,
            created_at: v.created_at,
            updated_at: v.updated_at,
        })
    }
}
impl Into<OrderDetailModel> for Detail {
    fn into(self) -> OrderDetailModel {
        OrderDetailModel {
            id: self.id.into(),
            order_id: self.order_id.into(),
            product_name: self.product_name.into(),
            quantity: self.quantity as i32,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl_repository!(
    OrderDetailRepository,
    order_details::table,
    OrderDetailModel,
    Detail,
    Id,
    order_details::id
);
impl_repository_insert!(
    OrderDetailModel,
    order_details::table,
    OrderDetailModel,
    Detail
);
impl OrderDetailRepository {}
