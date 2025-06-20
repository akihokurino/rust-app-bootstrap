use crate::errors::Kind::{Internal, NotFound};
use crate::errors::{impl_from_err_to_app_internal_err, AppError};
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::delete_object::DeleteObjectError;
use aws_sdk_s3::operation::get_object::GetObjectError;
use aws_sdk_s3::operation::head_object::HeadObjectError;
use aws_sdk_s3::operation::put_object::PutObjectError;

impl From<SdkError<GetObjectError>> for AppError {
    fn from(v: SdkError<GetObjectError>) -> Self {
        match v {
            SdkError::ServiceError(v) if v.err().is_no_such_key() => NotFound.into_err(),
            v => Internal.from_src(v),
        }
    }
}
impl_from_err_to_app_internal_err!(SdkError<HeadObjectError>);
impl_from_err_to_app_internal_err!(SdkError<PutObjectError>);
impl_from_err_to_app_internal_err!(SdkError<DeleteObjectError>);
