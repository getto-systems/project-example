use async_trait::async_trait;

use super::ResetTokenNotifier;

use crate::auth::password::reset::_api::{
    kernel::data::ResetTokenEncoded,
    request_token::data::{NotifyResetTokenError, NotifyResetTokenResponse, ResetTokenDestination},
};

pub struct EmailResetTokenNotifier;

impl EmailResetTokenNotifier {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ResetTokenNotifier for EmailResetTokenNotifier {
    async fn notify(
        &self,
        destination: ResetTokenDestination,
        token: ResetTokenEncoded,
    ) -> Result<NotifyResetTokenResponse, NotifyResetTokenError> {
        // TODO 多分テンプレートの中に URL も含めてしまって、EmailResetTokenNotifier を初期化するのがいいかな
        println!(
            "email: {}; https://example.com/reset?token={}",
            destination.into_email(),
            token.extract()
        );
        Ok(NotifyResetTokenResponse::new("message-id".into()))
    }
}

#[cfg(test)]
pub mod test {
    use async_trait::async_trait;

    use super::super::ResetTokenNotifier;

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

    #[async_trait]
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
