use crate::errors::AppError;

pub mod errors;
mod macros;
pub mod models;

pub type AppResult<T> = Result<T, AppError>;
