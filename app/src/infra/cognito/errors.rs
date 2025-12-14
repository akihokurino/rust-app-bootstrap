use crate::errors::AppError;
use crate::errors::Kind::{Internal, NotFound};
use aws_sdk_cognitoidentityprovider::error::SdkError;
use aws_sdk_cognitoidentityprovider::operation::admin_get_user::AdminGetUserError;

impl From<SdkError<AdminGetUserError>> for AppError {
    fn from(v: SdkError<AdminGetUserError>) -> Self {
        match v {
            SdkError::ServiceError(v) if v.err().is_user_not_found_exception() => {
                NotFound.into_err()
            }
            v => Internal.from_src(v),
        }
    }
}
