use crate::error::AppError;

pub mod error;
pub mod model;

pub type AppResult<T> = Result<T, AppError>;
