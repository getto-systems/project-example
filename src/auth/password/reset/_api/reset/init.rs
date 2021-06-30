mod messenger;
mod token_decoder;

use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::{
    auth_ticket::_api::kernel::init::CheckAuthNonceStruct,
    auth_user::_api::kernel::init::AuthUserStruct,
    password::_api::kernel::init::AuthUserPasswordStruct,
};
use messenger::ProtobufResetPasswordMessenger;
use token_decoder::JwtResetTokenDecoder;

use crate::auth::password::reset::_api::reset::infra::ResetPasswordInfra;

pub struct ResetPasswordStruct<'a> {
    check_nonce_infra: CheckAuthNonceStruct<'a>,
    user_infra: AuthUserStruct<'a>,
    password_infra: AuthUserPasswordStruct<'a>,
    token_decoder: JwtResetTokenDecoder<'a>,
    messenger: ProtobufResetPasswordMessenger,
}

impl<'a> ResetPasswordStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature, request: &'a HttpRequest, body: String) -> Self {
        Self {
            check_nonce_infra: CheckAuthNonceStruct::new(feature, request),
            user_infra: AuthUserStruct::new(feature),
            password_infra: AuthUserPasswordStruct::new(feature),
            token_decoder: JwtResetTokenDecoder::new(&feature.secret.reset_token.decoding_key),
            messenger: ProtobufResetPasswordMessenger::new(body),
        }
    }
}

impl<'a> ResetPasswordInfra for ResetPasswordStruct<'a> {
    type CheckNonceInfra = CheckAuthNonceStruct<'a>;
    type UserInfra = AuthUserStruct<'a>;
    type PasswordInfra = AuthUserPasswordStruct<'a>;
    type TokenDecoder = JwtResetTokenDecoder<'a>;
    type Messenger = ProtobufResetPasswordMessenger;

    fn check_nonce_infra(&self) -> &Self::CheckNonceInfra {
        &self.check_nonce_infra
    }
    fn user_infra(&self) -> &Self::UserInfra {
        &self.user_infra
    }
    fn password_infra(&self) -> &Self::PasswordInfra {
        &self.password_infra
    }
    fn token_decoder(&self) -> &Self::TokenDecoder {
        &self.token_decoder
    }
    fn messenger(&self) -> &Self::Messenger {
        &self.messenger
    }
}

#[cfg(test)]
pub mod test {
    pub use super::messenger::test::StaticResetPasswordMessenger;
    pub use super::token_decoder::test::StaticResetTokenDecoder;
    use crate::auth::{
        auth_ticket::_api::kernel::init::test::StaticCheckAuthNonceStruct,
        auth_user::_api::kernel::init::test::StaticAuthUserStruct,
        password::_api::kernel::init::test::StaticAuthUserPasswordStruct,
    };

    use crate::auth::password::reset::_api::reset::infra::ResetPasswordInfra;

    pub struct StaticResetPasswordStruct<'a> {
        pub check_nonce_infra: StaticCheckAuthNonceStruct<'a>,
        pub user_infra: StaticAuthUserStruct<'a>,
        pub password_infra: StaticAuthUserPasswordStruct<'a>,
        pub token_decoder: StaticResetTokenDecoder,
        pub messenger: StaticResetPasswordMessenger,
    }

    impl<'a> ResetPasswordInfra for StaticResetPasswordStruct<'a> {
        type CheckNonceInfra = StaticCheckAuthNonceStruct<'a>;
        type UserInfra = StaticAuthUserStruct<'a>;
        type PasswordInfra = StaticAuthUserPasswordStruct<'a>;
        type TokenDecoder = StaticResetTokenDecoder;
        type Messenger = StaticResetPasswordMessenger;

        fn check_nonce_infra(&self) -> &Self::CheckNonceInfra {
            &self.check_nonce_infra
        }
        fn user_infra(&self) -> &Self::UserInfra {
            &self.user_infra
        }
        fn password_infra(&self) -> &Self::PasswordInfra {
            &self.password_infra
        }
        fn token_decoder(&self) -> &Self::TokenDecoder {
            &self.token_decoder
        }
        fn messenger(&self) -> &Self::Messenger {
            &self.messenger
        }
    }
}
