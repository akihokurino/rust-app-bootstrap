use crate::errors::Kind::Internal;
use crate::AppResult;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;

#[derive(Debug, Clone)]
pub struct SessionManager {
    pool: PgPool,
}

impl SessionManager {
    pub async fn new(database_url: &str) -> AppResult<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await
            .map_err(Internal.from_srcf())?;

        Ok(Self { pool })
    }

    pub async fn from_env() -> AppResult<Self> {
        let database_url =
            env::var("DATABASE_URL").map_err(|_| Internal.with("Failed for database url"))?;
        Self::new(&database_url).await
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn tx_begin(&self) -> AppResult<sqlx::Transaction<sqlx::Postgres>> {
        self.pool.begin().await.map_err(Internal.from_srcf())
    }

    pub async fn tx_commit(&self, tx: sqlx::Transaction<'_, sqlx::Postgres>) -> AppResult<()> {
        tx.commit().await.map_err(Internal.from_srcf())
    }

    pub async fn close(&self) -> AppResult<()> {
        self.pool.close().await;
        Ok(())
    }
}
