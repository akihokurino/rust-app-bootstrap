use crate::domain::admin_user;
use crate::domain::types::asset_key::AssetKey;
use crate::domain::types::email::Email;
use crate::domain::types::image_size::ImageSize;
use crate::errors::AppError;
pub use crate::infra::rdb::session_manager::TransactionGuard;
pub use crate::infra::s3::types::HeadObjectResponse;
pub use crate::infra::sentry::InitGuard as ErrorNotifierGuard;
use crate::{AppResult, domain};
use async_trait::async_trait;
use bytes::Bytes;
use http::Uri;
use once_cell::sync::OnceCell;
pub use sea_orm::DatabaseConnection;
use serde::Serialize;
use std::sync::Arc;

#[async_trait]
pub trait Storage: Send + Sync {
    async fn presign_for_upload(&self, key: &AssetKey) -> AppResult<Uri>;
    async fn presign_for_get(&self, key: &AssetKey) -> AppResult<Uri>;
    async fn download_object(&self, key: &AssetKey) -> AppResult<Bytes>;
    async fn head_object(&self, key: &AssetKey) -> AppResult<HeadObjectResponse>;
    async fn copy_object(&self, src_key: &AssetKey, dest_key: &AssetKey) -> AppResult<()>;
    async fn delete_object(&self, key: &AssetKey) -> AppResult<()>;
}

#[async_trait]
pub trait TaskQueue: Send + Sync {
    async fn publish(&self, input: serde_json::Value, target: String) -> AppResult<()>;
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
#[async_trait]
impl sea_orm::ConnectionTrait for DbConn<'_> {
    fn get_database_backend(&self) -> sea_orm::DatabaseBackend {
        match self {
            DbConn::Db(db) => db.get_database_backend(),
            DbConn::Tx(tx) => tx.get_database_backend(),
        }
    }

    async fn execute_raw(
        &self,
        stmt: sea_orm::Statement,
    ) -> Result<sea_orm::ExecResult, sea_orm::DbErr> {
        match self {
            DbConn::Db(db) => db.execute_raw(stmt).await,
            DbConn::Tx(tx) => tx.execute_raw(stmt).await,
        }
    }

    async fn execute_unprepared(&self, sql: &str) -> Result<sea_orm::ExecResult, sea_orm::DbErr> {
        match self {
            DbConn::Db(db) => db.execute_unprepared(sql).await,
            DbConn::Tx(tx) => tx.execute_unprepared(sql).await,
        }
    }

    async fn query_one_raw(
        &self,
        stmt: sea_orm::Statement,
    ) -> Result<Option<sea_orm::QueryResult>, sea_orm::DbErr> {
        match self {
            DbConn::Db(db) => db.query_one_raw(stmt).await,
            DbConn::Tx(tx) => tx.query_one_raw(stmt).await,
        }
    }

    async fn query_all_raw(
        &self,
        stmt: sea_orm::Statement,
    ) -> Result<Vec<sea_orm::QueryResult>, sea_orm::DbErr> {
        match self {
            DbConn::Db(db) => db.query_all_raw(stmt).await,
            DbConn::Tx(tx) => tx.query_all_raw(stmt).await,
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
pub trait AdminAuth: Send + Sync {
    async fn verify(&self, token: &str) -> AppResult<admin_user::User>;
    async fn get(&self, id: &admin_user::Id) -> AppResult<admin_user::User>;
    async fn create(&self, id: admin_user::Id, email: Email) -> AppResult<()>;
    async fn delete(&self, id: &admin_user::Id) -> AppResult<()>;
}

#[async_trait]
pub trait UserAuth: Send + Sync {
    async fn verify(&self, token: &str) -> AppResult<domain::user::Id>;
    async fn get(&self, id: &domain::user::Id) -> AppResult<UserPrincipal>;
    async fn delete(&self, id: &domain::user::Id) -> AppResult<()>;
}
#[derive(Debug, Clone, Default, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UserPrincipal {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    pub email: Option<String>,
    pub provider_ids: Vec<String>,
    pub last_login_at: Option<i64>,
}
impl UserPrincipal {
    pub fn has_any_id(&self) -> bool {
        self.uid.is_some()
    }

    pub fn user_id(&self) -> Option<domain::user::Id> {
        self.uid
            .as_ref()
            .map(|v| domain::user::Id::from(v.as_str()))
    }
}

#[async_trait]
pub trait ImageCdn: Send + Sync {
    async fn presign_for_get(&self, key: &AssetKey, size: ImageSize) -> AppResult<Uri>;
}

#[async_trait]
pub trait Mail: Send + Sync {
    async fn send_text(&self, to: Email, subject: &str, text: &str) -> AppResult<()>;
}

pub trait ErrorNotifier: Send + Sync {
    fn init(&self) -> ErrorNotifierGuard;
    fn send(&self, err: AppError);
}

static GLOBAL_ERROR_NOTIFIER: OnceCell<Arc<dyn ErrorNotifier>> = OnceCell::new();

pub fn set_global_error_notifier(notifier: Arc<dyn ErrorNotifier>) {
    let _ = GLOBAL_ERROR_NOTIFIER.set(notifier);
}

pub fn notify_error(err: &AppError) {
    if let Some(notifier) = GLOBAL_ERROR_NOTIFIER.get() {
        notifier.send(err.clone());
    }
}
