use crate::errors::Kind::{Duplicate, Internal, NotFound};
use sea_orm::{DbErr, SqlErr};

pub fn map_insert_error(error: DbErr) -> crate::AppError {
    if let Some(SqlErr::UniqueConstraintViolation(_)) = error.sql_err() {
        return Duplicate.default();
    }

    match &error {
        DbErr::RecordNotInserted => Internal.with("record not inserted"),
        _ => Internal.from_src(error),
    }
}

pub fn map_update_error(error: DbErr) -> crate::AppError {
    match &error {
        DbErr::RecordNotUpdated => NotFound.default(),
        _ => Internal.from_src(error),
    }
}
