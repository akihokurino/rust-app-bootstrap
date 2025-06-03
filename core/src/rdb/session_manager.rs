use crate::errors::Kind::Internal;
use crate::AppResult;
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::Acquire;
use std::env;
use std::ops::DerefMut;

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

    // プールの参照を直接返すメソッド
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn transaction<F, T, Fut>(&self, f: F) -> AppResult<T>
    where
        F: for<'a> FnOnce(&'a mut sqlx::PgConnection) -> Fut,
        Fut: Future<Output = AppResult<T>>,
    {
        let mut tx = self.pool.begin().await.map_err(Internal.from_srcf())?;

        let result = f(tx.deref_mut()).await?;

        tx.commit().await.map_err(Internal.from_srcf())?;
        Ok(result)
    }

    pub async fn close(&self) -> AppResult<()> {
        self.pool.close().await;
        Ok(())
    }
}
