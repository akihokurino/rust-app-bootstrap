use migration::Migrator;
use sea_orm::{Database, DbErr};
use sea_orm_migration::MigratorTrait;
use std::env;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let db_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432/app".to_string());

    let db = Database::connect(&db_url).await?;
    Migrator::up(&db, None).await?;
    println!("Migration completed successfully");

    Ok(())
}
