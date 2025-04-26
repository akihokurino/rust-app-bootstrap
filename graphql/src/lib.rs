use actix_web::dev::ServiceFactory;

pub mod api;
pub mod error;

pub type GraphResult<T> = Result<T, error::Error>;
