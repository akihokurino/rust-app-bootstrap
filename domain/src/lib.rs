use crate::errors::AppError;

mod errors;

pub type AppResult<T> = Result<T, AppError>;
