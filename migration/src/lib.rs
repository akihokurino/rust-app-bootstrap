pub use sea_orm_migration::prelude::*;

mod m20250907_074340_create_users;
mod m20250907_074341_create_orders;
mod m20250907_074342_create_order_details;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250907_074340_create_users::Migration),
            Box::new(m20250907_074341_create_orders::Migration),
            Box::new(m20250907_074342_create_order_details::Migration),
        ]
    }
}
