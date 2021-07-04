use rusoto_ses::{Body, Content, Destination, Message, SendEmailRequest, Ses, SesClient};
use url::{ParseError, Url};

use crate::auth::_api::x_outside_feature::feature::AuthOutsideEmail;

use crate::auth::password::reset::_api::request_token::infra::ResetTokenNotifier;

use crate::auth::password::reset::_api::{
    kernel::data::ResetTokenEncoded,
    request_token::data::{NotifyResetTokenError, NotifyResetTokenResponse, ResetTokenDestination},
};

pub struct EmailResetTokenNotifier<'a> {
    client: &'a SesClient,
    ui_host: &'a str,
}

impl<'a> EmailResetTokenNotifier<'a> {
    pub fn new(email: &'a AuthOutsideEmail) -> Self {
        Self {
            client: &email.ses,
            ui_host: &email.ui_host,
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
        let message = build_message(self.ui_host, token)
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

fn build_message(ui_host: &str, token: ResetTokenEncoded) -> Result<Message, ParseError> {
    let url = build_url(ui_host, token)?;

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
fn build_url(ui_host: &str, token: ResetTokenEncoded) -> Result<Url, ParseError> {
    let mut url = Url::parse(format!("https://{}", ui_host).as_str())?;
    url.query_pairs_mut()
        .append_pair("-password-reset", "reset")
        .append_pair("-password-reset-token", token.as_str());
    Ok(url)
}
fn utf8_content(data: String) -> Content {
    Content {
        charset: Some("UTF-8".into()),
        data,
    }
}

const SENDER_ADDRESS: &'static str = "GETTO Example <labo@message.getto.systems>";
const SUBJECT: &'static str = "GETTO Example パスワードリセットの件 [URL のご案内]";
const BODY: &'static str = r#####"お世話になっております
GETTO Example システムです

下記 URL より、パスワードのリセットができます

{URL}

よろしくお願いいたします

#################################
GETTO Example
email: labo@message.getto.systems
---------------------------------
"#####;

#[cfg(test)]
pub mod test {
    use crate::auth::password::reset::_api::request_token::infra::ResetTokenNotifier;

    use crate::auth::password::reset::_api::{
        kernel::data::ResetTokenEncoded,
        request_token::data::{
            NotifyResetTokenError, NotifyResetTokenResponse, ResetTokenDestination,
        },
    };

    pub struct StaticResetTokenNotifier;

    impl StaticResetTokenNotifier {
        pub fn new() -> Self {
            Self
        }
    }

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
