pub mod admin;
mod data_loader;
mod errors;
pub mod service;
mod shared;

pub type GraphResult<T> = Result<T, errors::Error>;
