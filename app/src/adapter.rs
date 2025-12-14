use async_trait::async_trait;
use bytes::Bytes;
use http::Uri;
use std::future::Future;

use crate::{domain, AppResult};

use crate::domain::admin_user;
use crate::domain::types::email::Email;
use crate::domain::types::s3_key::S3Key;
pub use crate::infra::rdb::session_manager::TransactionGuard;
pub use crate::infra::s3::types::HeadObjectResponse;
pub use sea_orm::DatabaseConnection;

#[async_trait]
pub trait Storage: Send + Sync {
    async fn presign_for_upload(&self, key: &S3Key) -> AppResult<Uri>;
    async fn presign_for_get(&self, key: &S3Key) -> AppResult<Uri>;
    async fn download_object(&self, key: &S3Key) -> AppResult<Bytes>;
    async fn head_object(&self, key: &S3Key) -> AppResult<HeadObjectResponse>;
    async fn copy_object(&self, src_key: &S3Key, dest_key: &S3Key) -> AppResult<()>;
}

#[async_trait]
pub trait TaskQueue: Send + Sync {
    async fn publish(&self, input: serde_json::Value, arn: String) -> AppResult<()>;
}

#[async_trait]
pub trait RemoteFunction: Send + Sync {
    async fn invoke(&self, input: serde_json::Value, arn: String) -> AppResult<serde_json::Value>;
}

#[async_trait]
pub trait DBSession: Send + Sync {
    fn db(&self) -> &DatabaseConnection;
    fn conn(&self) -> DbConn<'_>;
    async fn begin_tx(&self) -> AppResult<TransactionGuard>;
}

pub enum DbConn<'a> {
    Db(&'a DatabaseConnection),
    Tx(&'a TransactionGuard),
}

impl<'a> From<&'a DatabaseConnection> for DbConn<'a> {
    fn from(db: &'a DatabaseConnection) -> Self {
        DbConn::Db(db)
    }
}

impl<'a> From<&'a TransactionGuard> for DbConn<'a> {
    fn from(tx: &'a TransactionGuard) -> Self {
        DbConn::Tx(tx)
    }
}

impl sea_orm::ConnectionTrait for DbConn<'_> {
    fn get_database_backend(&self) -> sea_orm::DatabaseBackend {
        match self {
            DbConn::Db(db) => db.get_database_backend(),
            DbConn::Tx(tx) => tx.get_database_backend(),
        }
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
        match self {
            DbConn::Db(db) => db.execute(stmt),
            DbConn::Tx(tx) => tx.execute(stmt),
        }
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
        match self {
            DbConn::Db(db) => db.execute_unprepared(sql),
            DbConn::Tx(tx) => tx.execute_unprepared(sql),
        }
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
        match self {
            DbConn::Db(db) => db.query_one(stmt),
            DbConn::Tx(tx) => tx.query_one(stmt),
        }
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
        match self {
            DbConn::Db(db) => db.query_all(stmt),
            DbConn::Tx(tx) => tx.query_all(stmt),
        }
    }

    fn support_returning(&self) -> bool {
        match self {
            DbConn::Db(db) => db.support_returning(),
            DbConn::Tx(tx) => tx.support_returning(),
        }
    }
}

#[async_trait]
pub trait UserAuth: Send + Sync {
    async fn delete(&self, user_id: &domain::user::Id) -> AppResult<()>;
    async fn verify(&self, token: &str) -> AppResult<domain::user::Id>;
}

#[async_trait]
pub trait AdminAuth: Send + Sync {
    async fn get(&self, id: &str) -> AppResult<admin_user::User>;
    async fn create(&self, id: String, email: Email) -> AppResult<admin_user::User>;
    async fn delete(&self, id: &str) -> AppResult<()>;
    async fn verify(&self, token_str: &str) -> AppResult<admin_user::User>;
}
