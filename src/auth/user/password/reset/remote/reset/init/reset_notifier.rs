use rusoto_ses::{Body, Content, Message, SendEmailRequest, Ses, SesClient};

use crate::auth::remote::x_outside_feature::auth::feature::AuthOutsideEmail;

use crate::auth::user::password::reset::remote::reset::infra::ResetPasswordNotifier;

use crate::auth::user::password::{
    remote::kernel::data::ResetTokenDestination,
    reset::remote::reset::data::{NotifyResetPasswordError, NotifyResetPasswordResponse},
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

const SENDER_ADDRESS: &'static str = "GETTO Example <labo@message.getto.systems>";
const SUBJECT: &'static str = "GETTO Example パスワードリセットの件 [リセット完了のお知らせ]";
const BODY: &'static str = r#####"お世話になっております
GETTO Example システムです

パスワードリセットが完了しました
今後は新しいパスワードを使用してログインしてください

もしパスワードリセットをしていない場合は
アカウント不正使用の可能性がありますので管理者にご連絡ください

よろしくお願いいたします

#################################
GETTO Example
email: labo@message.getto.systems
#################################
"#####;

#[cfg(test)]
pub mod test {
    use crate::auth::user::password::reset::remote::reset::infra::ResetPasswordNotifier;

    use crate::auth::user::password::{
        remote::kernel::data::ResetTokenDestination,
        reset::remote::reset::data::{NotifyResetPasswordError, NotifyResetPasswordResponse},
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
