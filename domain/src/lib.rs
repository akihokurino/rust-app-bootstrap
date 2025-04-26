use crate::error::AppError;

mod error;
mod model;

pub type AppResult<T> = Result<T, AppError>;
