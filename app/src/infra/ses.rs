use async_graphql::async_trait::async_trait;
use aws_sdk_sesv2::Client;
use aws_sdk_sesv2::error::SdkError;
use aws_sdk_sesv2::operation::send_email::SendEmailError;
use aws_sdk_sesv2::types::{Body, Content, Destination, EmailContent, Message};

use crate::AppResult;
use crate::adapter::Mail;
use crate::domain::types::email::Email;
use crate::errors::Kind::Internal;

#[derive(Clone, Debug)]
pub struct Adapter {
    client: Client,
    from: String,
}

impl Adapter {
    pub fn new(client: Client, from: &str) -> Self {
        Self {
            client,
            from: from.to_string(),
        }
    }
}

#[async_trait]
impl Mail for Adapter {
    async fn send_text(&self, to: Email, subject: &str, text: &str) -> AppResult<()> {
        let destination = Destination::builder().to_addresses(to.to_string()).build();

        let subject_content = Content::builder()
            .data(subject)
            .charset("UTF-8")
            .build()
            .map_err(|e| Internal.with(e.to_string()))?;
        let body_content = Content::builder()
            .data(text)
            .charset("UTF-8")
            .build()
            .map_err(|e| Internal.with(e.to_string()))?;
        let body = Body::builder().text(body_content).build();
        let message = Message::builder()
            .subject(subject_content)
            .body(body)
            .build();
        let email_content = EmailContent::builder().simple(message).build();

        self.client
            .send_email()
            .from_email_address(&self.from)
            .destination(destination)
            .content(email_content)
            .send()
            .await
            .map_err(|e: SdkError<SendEmailError>| {
                let message = match &e {
                    SdkError::ServiceError(err) => {
                        format!(
                            "SES ServiceError: {:?} - {}",
                            err.err(),
                            err.err().meta().message().unwrap_or("unknown")
                        )
                    }
                    _ => format!("SES Error: {:?}", e),
                };
                tracing::error!("{}", message);
                Internal.with(message)
            })?;

        Ok(())
    }
}
