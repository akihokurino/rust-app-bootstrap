use crate::errors::Kind::{Duplicate, Internal};
use sea_orm::DbErr;

pub mod repository;
pub mod session_manager;
mod types;

pub fn map_insert_error(error: DbErr) -> crate::AppError {
    match &error {
        DbErr::RecordNotInserted => Internal.with("record not inserted"),
        DbErr::Exec(sea_orm::RuntimeErr::SqlxError(sqlx_err)) => {
            if let Some(db_err) = sqlx_err.as_database_error() {
                if db_err.code() == Some(std::borrow::Cow::Borrowed("23505")) {
                    Duplicate.default()
                } else {
                    Internal.from_src(error)
                }
            } else {
                Internal.from_src(error)
            }
        }
        _ => Internal.from_src(error),
    }
}
