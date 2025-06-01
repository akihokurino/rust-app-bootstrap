use crate::errors::AppError;

pub mod domain;
pub mod errors;
mod rdb;

pub type AppResult<T> = Result<T, AppError>;
