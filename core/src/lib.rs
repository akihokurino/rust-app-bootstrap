use crate::errors::AppError;

pub mod domain;
pub mod errors;

pub type AppResult<T> = Result<T, AppError>;
