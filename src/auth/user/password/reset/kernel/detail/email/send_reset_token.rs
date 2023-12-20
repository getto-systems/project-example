use std::sync::Arc;

use aws_sdk_ses::{
    error::BuildError,
    types::{Body, Content, Destination, Message},
    Client,
};
use url::{ParseError, Url};

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::{
    common::api::feature::AsInfra,
    x_content::mail::{
        send_reset_token::{BODY, SUBJECT},
        SENDER_ADDRESS,
    },
};

use crate::{
    auth::user::password::reset::kernel::data::{
        ResetPasswordToken, ResetPasswordTokenDestinationEmail,
    },
    common::api::notification::data::NotificationError,
};

pub struct EmailSendResetToken {
    client: Arc<Client>,
    reset_password_url: &'static str,
}

impl AsInfra<EmailSendResetToken> for Arc<AuthAppFeature> {
    fn as_infra(&self) -> EmailSendResetToken {
        EmailSendResetToken {
            client: Arc::clone(&self.email.ses),
            reset_password_url: &self.email.reset_password_url,
        }
    }
}

impl EmailSendResetToken {
    pub async fn send(
        &self,
        destination: ResetPasswordTokenDestinationEmail,
        token: ResetPasswordToken,
    ) -> Result<String, NotificationError> {
        let destination = Destination::builder()
            .to_addresses(destination.extract())
            .build();

        let message = build_message(self.reset_password_url, token)?;

        let request = self
            .client
            .send_email()
            .source(SENDER_ADDRESS)
            .destination(destination)
            .message(message);

        let response = request
            .send()
            .await
            .map_err(|err| ("failed to send reset-password email", err))?;

        Ok(response.message_id)
    }
}

fn build_message(
    reset_password_url: &str,
    token: ResetPasswordToken,
) -> Result<Message, NotificationError> {
    let url = build_url(reset_password_url, token)
        .map_err(|err| ("failed to build reset-password-url", err))?;

    let subject = utf8_content(SUBJECT.into())
        .map_err(|err| ("failed to build reset-password subject", err))?;

    let body = utf8_content(BODY.replace("{URL}", url.as_str()))
        .map_err(|err| ("failed to build reset-password body", err))?;

    let message = Message::builder()
        .subject(subject)
        .body(Body::builder().text(body).build())
        .build();

    Ok(message)
}
fn build_url(reset_password_url: &str, token: ResetPasswordToken) -> Result<Url, ParseError> {
    // path と query を組み立てる; ホスト名は使用されない
    let mut target = Url::parse("http://localhost/index.html")?;
    target
        .query_pairs_mut()
        .append_pair("-password-reset", "reset")
        .append_pair("-password-reset-token", token.extract().as_str());
    let target = format!("{}?{}", target.path(), target.query().unwrap_or(""));

    let mut url = Url::parse(reset_password_url)?;
    url.query_pairs_mut()
        .append_pair("-application-target", &target);
    Ok(url)
}
fn utf8_content(data: String) -> Result<Content, BuildError> {
    Content::builder().charset("UTF-8").data(data).build()
}
