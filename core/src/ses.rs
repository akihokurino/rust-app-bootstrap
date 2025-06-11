use crate::domain::Email;
use crate::errors::impl_from_err_to_app_internal_err;
use crate::AppResult;
use aws_sdk_sesv2::error::*;
use aws_sdk_sesv2::operation::send_email::SendEmailError;
use aws_sdk_sesv2::types::{Body, Content, Destination, EmailContent, Message};
use aws_sdk_sesv2::Client;
use rfc2047::rfc2047_encode;

impl_from_err_to_app_internal_err!(SdkError<SendEmailError>);

#[derive(Clone, Debug)]
pub struct Adapter {
    client: Client,
    from: String,
}

impl Adapter {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            from: rfc2047_encode("RustApp <noreply@rust.jp>").to_string(),
        }
    }

    pub async fn send(&self, to: Email, subject: &str, message: &str) -> AppResult<()> {
        let dest = Destination::builder().to_addresses(to.to_string()).build();
        let subject_content = Content::builder()
            .data(subject)
            .charset("UTF-8")
            .build()
            .unwrap();
        let body_content = Content::builder()
            .data(message)
            .charset("UTF-8")
            .build()
            .unwrap();
        let body = Body::builder().text(body_content).build();

        let msg = Message::builder()
            .subject(subject_content)
            .body(body)
            .build();

        let email_content = EmailContent::builder().simple(msg).build();

        self.client
            .send_email()
            .from_email_address(self.from.clone())
            .destination(dest)
            .content(email_content)
            .send()
            .await?;

        Ok(())
    }
}
