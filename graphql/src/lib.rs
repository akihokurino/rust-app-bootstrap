pub mod api;
pub mod errors;
mod shared;

pub type GraphResult<T> = Result<T, errors::Error>;
