pub mod api;
pub mod errors;

pub type GraphResult<T> = Result<T, errors::Error>;
