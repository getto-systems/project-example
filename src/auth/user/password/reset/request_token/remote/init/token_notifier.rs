use rusoto_ses::{Body, Content, Destination, Message, SendEmailRequest, Ses, SesClient};
use url::{ParseError, Url};

use crate::auth::x_outside_feature::remote::auth::feature::AuthOutsideEmail;

use crate::x_content::mail::{
    notify_password_reset_token::{BODY, SUBJECT},
    SENDER_ADDRESS,
};

use crate::auth::user::password::reset::request_token::remote::infra::ResetTokenNotifier;

use crate::auth::user::password::reset::{
    kernel::data::{ResetTokenDestination, ResetTokenEncoded},
    request_token::remote::data::{NotifyResetTokenError, NotifyResetTokenResponse},
};

pub struct EmailResetTokenNotifier<'a> {
    client: &'a SesClient,
    reset_password_url: &'a str,
}

impl<'a> EmailResetTokenNotifier<'a> {
    pub fn new(email: &'a AuthOutsideEmail) -> Self {
        Self {
            client: &email.ses,
            reset_password_url: &email.reset_password_url,
        }
    }
}

#[async_trait::async_trait]
impl<'a> ResetTokenNotifier for EmailResetTokenNotifier<'a> {
    async fn notify(
        &self,
        destination: ResetTokenDestination,
        token: ResetTokenEncoded,
    ) -> Result<NotifyResetTokenResponse, NotifyResetTokenError> {
        let destination = destination.into();
        let message = build_message(self.reset_password_url, token)
            .map_err(|err| NotifyResetTokenError::InfraError(format!("{}", err)))?;
        let source = SENDER_ADDRESS.into();

        let request = SendEmailRequest {
            destination,
            message,
            source,
            ..Default::default()
        };

        let response = self
            .client
            .send_email(request)
            .await
            .map_err(|err| NotifyResetTokenError::InfraError(format!("{}", err)))?;

        Ok(NotifyResetTokenResponse::new(response.message_id))
    }
}

impl Into<Destination> for ResetTokenDestination {
    fn into(self) -> Destination {
        Destination {
            bcc_addresses: None,
            cc_addresses: None,
            to_addresses: Some(vec![self.into_email()]),
        }
    }
}

fn build_message(
    reset_password_url: &str,
    token: ResetTokenEncoded,
) -> Result<Message, ParseError> {
    let url = build_url(reset_password_url, token)?;

    let subject = SUBJECT.into();
    let body = BODY.replace("{URL}", url.as_str());

    Ok(Message {
        subject: utf8_content(subject),
        body: Body {
            html: None,
            text: Some(utf8_content(body)),
        },
    })
}
fn build_url(reset_password_url: &str, token: ResetTokenEncoded) -> Result<Url, ParseError> {
    // path と query を組み立てる; ホスト名は使用されない
    let mut target = Url::parse("http://localhost/index.html")?;
    target
        .query_pairs_mut()
        .append_pair("-password-reset", "reset")
        .append_pair("-password-reset-token", token.as_str());
    let target = format!("{}?{}", target.path(), target.query().unwrap_or(""));

    let mut url = Url::parse(reset_password_url)?;
    url.query_pairs_mut()
        .append_pair("-application-target", &target);
    Ok(url)
}
fn utf8_content(data: String) -> Content {
    Content {
        charset: Some("UTF-8".into()),
        data,
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::user::password::reset::request_token::remote::infra::ResetTokenNotifier;

    use crate::auth::user::password::reset::{
        kernel::data::{ResetTokenDestination, ResetTokenEncoded},
        request_token::remote::data::{NotifyResetTokenError, NotifyResetTokenResponse},
    };

    pub struct StaticResetTokenNotifier;

    #[async_trait::async_trait]
    impl ResetTokenNotifier for StaticResetTokenNotifier {
        async fn notify(
            &self,
            _destination: ResetTokenDestination,
            _token: ResetTokenEncoded,
        ) -> Result<NotifyResetTokenResponse, NotifyResetTokenError> {
            Ok(NotifyResetTokenResponse::new("message-id".into()))
        }
    }
}