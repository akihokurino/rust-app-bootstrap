pub mod error;
pub mod string_model;

#[allow(unused)]
pub trait FromUnchecked<T> {
    fn from_unchecked(value: T) -> Self;
}
