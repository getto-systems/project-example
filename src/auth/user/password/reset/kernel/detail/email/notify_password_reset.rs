use std::sync::Arc;

use aws_sdk_ses::{
    error::BuildError,
    types::{Body, Content, Destination, Message},
    Client,
};

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::x_content::mail::{
    notify_password_reset::{BODY, SUBJECT},
    SENDER_ADDRESS,
};

use crate::{
    auth::user::password::reset::kernel::data::ResetPasswordTokenDestinationEmail,
    common::api::{feature::AsInfra, notification::data::NotificationError},
};

pub struct EmailNotifyPasswordReset {
    client: Arc<Client>,
}

impl AsInfra<EmailNotifyPasswordReset> for Arc<AuthAppFeature> {
    fn as_infra(&self) -> EmailNotifyPasswordReset {
        EmailNotifyPasswordReset {
            client: Arc::clone(&self.email.ses),
        }
    }
}

impl EmailNotifyPasswordReset {
    pub async fn send(
        &self,
        destination: ResetPasswordTokenDestinationEmail,
    ) -> Result<String, NotificationError> {
        let destination = Destination::builder()
            .to_addresses(destination.extract())
            .build();

        let message = build_message()?;

        let request = self
            .client
            .send_email()
            .source(SENDER_ADDRESS)
            .destination(destination)
            .message(message);

        let response = request
            .send()
            .await
            .map_err(|err| ("failed to send notify-password-reset email", err))?;

        Ok(response.message_id)
    }
}

fn build_message() -> Result<Message, NotificationError> {
    let subject = utf8_content(SUBJECT.into())
        .map_err(|err| ("failed to build notify-password-reset subject", err))?;

    let body = utf8_content(BODY.into())
        .map_err(|err| ("failed to build notify-password-reset body", err))?;

    let message = Message::builder()
        .subject(subject)
        .body(Body::builder().text(body).build())
        .build();

    Ok(message)
}
fn utf8_content(data: String) -> Result<Content, BuildError> {
    Content::builder().charset("UTF-8").data(data).build()
}
