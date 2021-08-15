pub(in crate::auth) mod request_decoder;
pub(in crate::auth) mod token_decoder;

use crate::auth::auth_ticket::_auth::kernel::infra::AuthClockInfra;

use crate::auth::_auth::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_auth::kernel::init::ChronoAuthClockInitializer;

use crate::auth::{
    auth_ticket::_auth::kernel::init::CheckAuthNonceStruct,
    auth_user::_auth::kernel::init::AuthUserStruct,
    password::_auth::kernel::init::AuthUserPasswordStruct,
};
use token_decoder::JwtResetTokenDecoder;
use tonic::metadata::MetadataMap;

use crate::auth::password::reset::_auth::reset::infra::ResetPasswordInfra;

pub struct ResetPasswordStruct<'a> {
    check_nonce_infra: CheckAuthNonceStruct<'a>,
    clock_infra: AuthClockInfra,
    user_infra: AuthUserStruct<'a>,
    password_infra: AuthUserPasswordStruct<'a>,
    token_decoder: JwtResetTokenDecoder<'a>,
}

impl<'a> ResetPasswordStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature, metadata: MetadataMap) -> Self {
        Self {
            check_nonce_infra: CheckAuthNonceStruct::new(feature, metadata),
            clock_infra: AuthClockInfra::new(ChronoAuthClockInitializer),
            user_infra: AuthUserStruct::new(feature),
            password_infra: AuthUserPasswordStruct::new(feature),
            token_decoder: JwtResetTokenDecoder::new(&feature.secret),
        }
    }
}

impl<'a> ResetPasswordInfra for ResetPasswordStruct<'a> {
    type CheckNonceInfra = CheckAuthNonceStruct<'a>;
    type UserInfra = AuthUserStruct<'a>;
    type PasswordInfra = AuthUserPasswordStruct<'a>;
    type TokenDecoder = JwtResetTokenDecoder<'a>;

    fn check_nonce_infra(&self) -> &Self::CheckNonceInfra {
        &self.check_nonce_infra
    }
    fn clock_infra(&self) -> &AuthClockInfra {
        &self.clock_infra
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
}

#[cfg(test)]
pub mod test {
    pub use super::request_decoder::test::StaticResetPasswordRequestDecoder;
    pub use super::token_decoder::test::StaticResetTokenDecoder;
    use crate::auth::{
        auth_ticket::_auth::kernel::init::test::StaticCheckAuthNonceStruct,
        auth_user::_auth::kernel::init::test::StaticAuthUserStruct,
        password::_auth::kernel::init::test::StaticAuthUserPasswordStruct,
    };

    use super::super::infra::ResetPasswordInfra;
    use crate::auth::auth_ticket::_auth::kernel::infra::AuthClockInfra;

    pub struct StaticResetPasswordStruct<'a> {
        pub check_nonce_infra: StaticCheckAuthNonceStruct<'a>,
        pub clock_infra: AuthClockInfra,
        pub user_infra: StaticAuthUserStruct<'a>,
        pub password_infra: StaticAuthUserPasswordStruct<'a>,
        pub token_decoder: StaticResetTokenDecoder,
    }

    impl<'a> ResetPasswordInfra for StaticResetPasswordStruct<'a> {
        type CheckNonceInfra = StaticCheckAuthNonceStruct<'a>;
        type UserInfra = StaticAuthUserStruct<'a>;
        type PasswordInfra = StaticAuthUserPasswordStruct<'a>;
        type TokenDecoder = StaticResetTokenDecoder;

        fn check_nonce_infra(&self) -> &Self::CheckNonceInfra {
            &self.check_nonce_infra
        }
        fn clock_infra(&self) -> &AuthClockInfra {
            &self.clock_infra
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
    }
}
