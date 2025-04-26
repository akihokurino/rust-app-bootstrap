use crate::errors::AppError;

pub mod errors;
pub mod types;

pub type AppResult<T> = Result<T, AppError>;
