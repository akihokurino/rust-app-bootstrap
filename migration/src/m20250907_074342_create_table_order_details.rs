use crate::m20250907_074341_create_table_orders::Orders;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OrderDetails::Table)
                    .if_not_exists()
                    .col(string(OrderDetails::Id).primary_key())
                    .col(string(OrderDetails::OrderId))
                    .col(string(OrderDetails::ProductName))
                    .col(integer(OrderDetails::Quantity))
                    .col(
                        timestamp_with_time_zone(OrderDetails::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(OrderDetails::UpdatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_order_details_order")
                            .from(OrderDetails::Table, OrderDetails::OrderId)
                            .to(Orders::Table, Orders::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OrderDetails::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum OrderDetails {
    Table,
    Id,
    OrderId,
    ProductName,
    Quantity,
    CreatedAt,
    UpdatedAt,
}
