pub use sea_orm_migration::prelude::*;

mod m20250907_074340_create_table_users;
mod m20250907_074340_create_table_orders;
mod m20250907_074340_create_table_order_details;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250907_074340_create_table_users::Migration),
            Box::new(m20250907_074340_create_table_orders::Migration),
            Box::new(m20250907_074340_create_table_order_details::Migration),
        ]
    }
}
