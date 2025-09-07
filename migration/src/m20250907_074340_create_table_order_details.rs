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
                    .col(string(OrderDetails::OrderId).not_null())
                    .col(string(OrderDetails::ProductName).not_null())
                    .col(integer(OrderDetails::Quantity).not_null())
                    .col(timestamp_with_time_zone(OrderDetails::CreatedAt).not_null().default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone(OrderDetails::UpdatedAt).not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_order_details_order")
                            .from(OrderDetails::Table, OrderDetails::OrderId)
                            .to(Orders::Table, Orders::Id)
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

#[derive(DeriveIden)]
enum Orders {
    Table,
    Id,
}
