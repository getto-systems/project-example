use rusoto_ses::{Body, Content, Message, SendEmailRequest, Ses, SesClient};

use crate::auth::x_outside_feature::auth::feature::AuthOutsideEmail;

use crate::x_content::mail::{
    notify_password_reset::{BODY, SUBJECT},
    SENDER_ADDRESS,
};

use crate::auth::user::password::reset::reset::infra::ResetPasswordNotifier;

use crate::auth::user::password::reset::{
    kernel::data::ResetTokenDestination,
    reset::data::{NotifyResetPasswordError, NotifyResetPasswordResponse},
};

pub struct EmailResetPasswordNotifier<'a> {
    client: &'a SesClient,
}

impl<'a> EmailResetPasswordNotifier<'a> {
    pub fn new(email: &'a AuthOutsideEmail) -> Self {
        Self { client: &email.ses }
    }
}

#[async_trait::async_trait]
impl<'a> ResetPasswordNotifier for EmailResetPasswordNotifier<'a> {
    async fn notify(
        &self,
        destination: ResetTokenDestination,
    ) -> Result<NotifyResetPasswordResponse, NotifyResetPasswordError> {
        let destination = destination.into();
        let message = build_message();
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
            .map_err(|err| NotifyResetPasswordError::InfraError(format!("{}", err)))?;

        Ok(NotifyResetPasswordResponse::new(response.message_id))
    }
}

fn build_message() -> Message {
    let subject = SUBJECT.into();
    let body = BODY.into();

    Message {
        subject: utf8_content(subject),
        body: Body {
            html: None,
            text: Some(utf8_content(body)),
        },
    }
}
fn utf8_content(data: String) -> Content {
    Content {
        charset: Some("UTF-8".into()),
        data,
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::user::password::reset::reset::infra::ResetPasswordNotifier;

    use crate::auth::user::password::reset::{
        kernel::data::ResetTokenDestination,
        reset::data::{NotifyResetPasswordError, NotifyResetPasswordResponse},
    };

    pub struct StaticResetPasswordNotifier;

    #[async_trait::async_trait]
    impl ResetPasswordNotifier for StaticResetPasswordNotifier {
        async fn notify(
            &self,
            _destination: ResetTokenDestination,
        ) -> Result<NotifyResetPasswordResponse, NotifyResetPasswordError> {
            Ok(NotifyResetPasswordResponse::new("message-id".into()))
        }
    }
}
