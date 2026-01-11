mod errors;

use async_graphql::async_trait::async_trait;
use aws_sdk_sesv2::types::{Body, Content, Destination, EmailContent, Message};
use aws_sdk_sesv2::Client;

use crate::adapter::Mail;
use crate::domain::types::email::Email;
use crate::AppResult;

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

        let subject_content = Content::builder().data(subject).charset("UTF-8").build()?;
        let body_content = Content::builder().data(text).charset("UTF-8").build()?;
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
            .await?;

        Ok(())
    }
}
