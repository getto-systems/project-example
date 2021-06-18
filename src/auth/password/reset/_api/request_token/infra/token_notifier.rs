use super::ResetTokenNotifier;

use crate::auth::password::reset::_api::{
    kernel::data::ResetTokenEncoded,
    request_token::data::{NotifyResetTokenError, ResetTokenDestination},
};

pub struct EmailResetTokenNotifier;

impl EmailResetTokenNotifier {
    pub fn new() -> Self {
        Self
    }
}

impl ResetTokenNotifier for EmailResetTokenNotifier {
    fn notify(
        &self,
        destination: ResetTokenDestination,
        token: ResetTokenEncoded,
    ) -> Result<(), NotifyResetTokenError> {
        // TODO 多分テンプレートの中に URL も含めてしまって、EmailResetTokenNotifier を初期化するのがいいかな
        println!(
            "email: {}; https://example.com/reset?token={}",
            destination.into_email(),
            token.extract()
        );
        Ok(())
    }
}

#[cfg(test)]
pub mod test {
    use super::super::ResetTokenNotifier;

    use crate::auth::password::reset::_api::{
        kernel::data::ResetTokenEncoded,
        request_token::data::{NotifyResetTokenError, ResetTokenDestination},
    };

    pub struct StaticResetTokenNotifier;

    impl StaticResetTokenNotifier {
        pub fn new() -> Self {
            Self
        }
    }

    impl ResetTokenNotifier for StaticResetTokenNotifier {
        fn notify(
            &self,
            _destination: ResetTokenDestination,
            _token: ResetTokenEncoded,
        ) -> Result<(), NotifyResetTokenError> {
            Ok(())
        }
    }
}
