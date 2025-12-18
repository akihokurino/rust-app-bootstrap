use crate::adapter::{DBSession, DbConn};
use crate::errors::Kind::Internal;
use crate::AppResult;
use async_trait::async_trait;
use sea_orm::{
    ConnectOptions, Database, DatabaseConnection, DatabaseTransaction, TransactionTrait,
};
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct SessionManager {
    db: DatabaseConnection,
}

impl SessionManager {
    pub async fn new(database_url: &str) -> AppResult<Self> {
        let mut opt = ConnectOptions::new(database_url);
        opt.max_connections(2)
            .min_connections(1)
            .connect_timeout(Duration::from_secs(10))
            .acquire_timeout(Duration::from_secs(10))
            .idle_timeout(Duration::from_secs(300))
            .max_lifetime(Duration::from_secs(1800));

        let db = Database::connect(opt).await.map_err(Internal.from_srcf())?;

        Ok(Self { db })
    }

    #[allow(dead_code)]
    pub async fn from_env() -> AppResult<Self> {
        let database_url =
            env::var("DATABASE_URL").map_err(|_| Internal.with("Failed for database url"))?;
        Self::new(&database_url).await
    }
}

#[async_trait]
impl DBSession for SessionManager {
    fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    fn conn(&self) -> DbConn<'_> {
        DbConn::Db(&self.db)
    }

    async fn begin_tx(&self) -> AppResult<TransactionGuard> {
        let tx = self.db.begin().await.map_err(Internal.from_srcf())?;
        Ok(TransactionGuard::new(tx))
    }
}

pub struct TransactionGuard {
    inner: Option<DatabaseTransaction>,
    committed: Arc<AtomicBool>,
}

impl TransactionGuard {
    fn new(tx: DatabaseTransaction) -> Self {
        Self {
            inner: Some(tx),
            committed: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn as_ref(&self) -> &DatabaseTransaction {
        self.inner.as_ref().expect("Transaction already consumed")
    }

    pub fn conn(&self) -> DbConn<'_> {
        DbConn::Tx(self)
    }

    pub async fn commit(mut self) -> AppResult<()> {
        let tx = self.inner.take().expect("Transaction already consumed");
        tx.commit().await.map_err(Internal.from_srcf())?;
        self.committed.store(true, Ordering::Relaxed);
        Ok(())
    }

    pub async fn rollback(mut self) -> AppResult<()> {
        let tx = self.inner.take().expect("Transaction already consumed");
        tx.rollback().await.map_err(Internal.from_srcf())?;
        self.committed.store(true, Ordering::Relaxed);
        Ok(())
    }
}

impl Drop for TransactionGuard {
    fn drop(&mut self) {
        if let Some(tx) = self.inner.take() {
            if !self.committed.load(Ordering::Relaxed) {
                tokio::spawn(async move {
                    let _ = tx.rollback().await;
                });
            }
        }
    }
}
impl std::ops::Deref for TransactionGuard {
    type Target = DatabaseTransaction;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}
impl sea_orm::ConnectionTrait for TransactionGuard {
    fn get_database_backend(&self) -> sea_orm::DatabaseBackend {
        self.as_ref().get_database_backend()
    }

    fn execute<'life0, 'async_trait>(
        &'life0 self,
        stmt: sea_orm::Statement,
    ) -> std::pin::Pin<
        Box<dyn Future<Output = Result<sea_orm::ExecResult, sea_orm::DbErr>> + Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move { self.as_ref().execute(stmt).await })
    }

    fn query_one<'life0, 'async_trait>(
        &'life0 self,
        stmt: sea_orm::Statement,
    ) -> std::pin::Pin<
        Box<
            dyn Future<Output = Result<Option<sea_orm::QueryResult>, sea_orm::DbErr>>
                + Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move { self.as_ref().query_one(stmt).await })
    }

    fn query_all<'life0, 'async_trait>(
        &'life0 self,
        stmt: sea_orm::Statement,
    ) -> std::pin::Pin<
        Box<
            dyn Future<Output = Result<Vec<sea_orm::QueryResult>, sea_orm::DbErr>>
                + Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move { self.as_ref().query_all(stmt).await })
    }

    fn support_returning(&self) -> bool {
        self.as_ref().support_returning()
    }

    fn execute_unprepared<'life0, 'life1, 'async_trait>(
        &'life0 self,
        sql: &'life1 str,
    ) -> std::pin::Pin<
        Box<dyn Future<Output = Result<sea_orm::ExecResult, sea_orm::DbErr>> + Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move { self.as_ref().execute_unprepared(sql).await })
    }
}
