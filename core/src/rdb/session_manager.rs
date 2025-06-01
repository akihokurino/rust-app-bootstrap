use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::{Acquire, Postgres, Transaction};
use std::env;
use crate::AppResult;
use crate::errors::Kind::Internal;

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
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| Internal.with("Failed for database url"))?;
        Self::new(&database_url).await
    }
    
    pub async fn transaction<F, T, Fut>(&self, f: F) -> AppResult<T>
    where
        F: FnOnce(&mut Transaction<'_, Postgres>) -> Fut,
        Fut: Future<Output = AppResult<T>>,
    {
        let mut tx = self.pool.begin().await.map_err(Internal.from_srcf())?;

        let result = f(&mut tx).await?;

        tx.commit().await.map_err(Internal.from_srcf())?;

        Ok(result)
    }
    
    pub async fn read<F, T, Fut>(&self, f: F) -> AppResult<T>
    where
        F: FnOnce(&PgPool) -> Fut,
        Fut: Future<Output = AppResult<T>>,
    {
        f(&self.pool).await
    }
    
    pub async fn write<F, T, Fut>(&self, f: F) -> AppResult<T>
    where
        F: FnOnce(&PgPool) -> Fut,
        Fut: Future<Output = AppResult<T>>,
    {
        f(&self.pool).await
    }
    
    pub async fn health_check(&self) -> AppResult<()> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .map_err(Internal.from_srcf())?;
        Ok(())
    }
    
    pub async fn close(&self) -> AppResult<()> {
        self.pool.close().await;
        Ok(())
    }
}