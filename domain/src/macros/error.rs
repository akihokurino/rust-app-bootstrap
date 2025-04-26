macro_rules! impl_from_err_to_app_internal_err {
    ($T:ty) => {
        impl From<$T> for crate::errors::AppError {
            fn from(v: $T) -> Self {
                crate::errors::Kind::Internal.from_src(v)
            }
        }
    };
}
pub(crate) use impl_from_err_to_app_internal_err;
