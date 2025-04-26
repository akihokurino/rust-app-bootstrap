mod error;
pub mod string_model;

pub trait FromUnchecked<T> {
    fn from_unchecked(value: T) -> Self;
}
