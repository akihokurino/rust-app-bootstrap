use aws_sdk_sesv2::error::SdkError;
use aws_sdk_sesv2::operation::send_email::SendEmailError;

use crate::errors::AppError;
use crate::errors::Kind::Internal;

impl From<SdkError<SendEmailError>> for AppError {
    fn from(v: SdkError<SendEmailError>) -> Self {
        let message = match &v {
            SdkError::ServiceError(err) => {
                format!(
                    "SES ServiceError: {:?} - {}",
                    err.err(),
                    err.err().meta().message().unwrap_or("unknown")
                )
            }
            _ => format!("SES Error: {:?}", v),
        };
        tracing::error!("{}", message);
        Internal.with(message)
    }
}

impl From<aws_sdk_sesv2::error::BuildError> for AppError {
    fn from(v: aws_sdk_sesv2::error::BuildError) -> Self {
        Internal.with(v.to_string())
    }
}
