use crate::errors::AppError;

pub mod errors;
mod macros;
pub mod types;

pub type AppResult<T> = Result<T, AppError>;
