mod request_decoder;
mod token_decoder;

use crate::auth::_auth::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::auth_ticket::_auth::kernel::init::AuthClockStruct;
use crate::auth::password::reset::_common::y_protobuf::service::ResetPasswordRequestPb;

use crate::auth::{
    auth_ticket::_auth::kernel::init::CheckAuthNonceStruct,
    auth_user::_auth::kernel::init::AuthUserStruct,
    password::_auth::kernel::init::AuthUserPasswordStruct,
};
use request_decoder::PbResetPasswordRequestDecoder;
use token_decoder::JwtResetTokenDecoder;
use tonic::metadata::MetadataMap;

use crate::auth::password::reset::_auth::reset::infra::ResetPasswordInfra;

pub struct ResetPasswordStruct<'a> {
    check_nonce_infra: CheckAuthNonceStruct<'a>,
    clock_infra: AuthClockStruct,
    user_infra: AuthUserStruct<'a>,
    password_infra: AuthUserPasswordStruct<'a>,
    token_decoder: JwtResetTokenDecoder<'a>,
    request_decoder: PbResetPasswordRequestDecoder,
}

impl<'a> ResetPasswordStruct<'a> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        metadata: MetadataMap,
        request: ResetPasswordRequestPb,
    ) -> Self {
        Self {
            check_nonce_infra: CheckAuthNonceStruct::new(feature, metadata),
            clock_infra: AuthClockStruct::new(),
            user_infra: AuthUserStruct::new(feature),
            password_infra: AuthUserPasswordStruct::new(feature),
            token_decoder: JwtResetTokenDecoder::new(&feature.secret),
            request_decoder: PbResetPasswordRequestDecoder::new(request),
        }
    }
}

impl<'a> ResetPasswordInfra for ResetPasswordStruct<'a> {
    type CheckNonceInfra = CheckAuthNonceStruct<'a>;
    type ClockInfra = AuthClockStruct;
    type UserInfra = AuthUserStruct<'a>;
    type PasswordInfra = AuthUserPasswordStruct<'a>;
    type TokenDecoder = JwtResetTokenDecoder<'a>;
    type RequestDecoder = PbResetPasswordRequestDecoder;

    fn extract(
        self,
    ) -> (
        Self::CheckNonceInfra,
        Self::ClockInfra,
        Self::UserInfra,
        Self::PasswordInfra,
        Self::RequestDecoder,
        Self::TokenDecoder,
    ) {
        (
            self.check_nonce_infra,
            self.clock_infra,
            self.user_infra,
            self.password_infra,
            self.request_decoder,
            self.token_decoder,
        )
    }
}

#[cfg(test)]
pub mod test {
    pub use super::request_decoder::test::StaticResetPasswordRequestDecoder;
    pub use super::token_decoder::test::StaticResetTokenDecoder;
    use crate::auth::{
        auth_ticket::_auth::kernel::init::test::{
            StaticAuthClockStruct, StaticCheckAuthNonceStruct,
        },
        auth_user::_auth::kernel::init::test::StaticAuthUserStruct,
        password::_auth::kernel::init::test::StaticAuthUserPasswordStruct,
    };

    use crate::auth::password::reset::_auth::reset::infra::ResetPasswordInfra;

    pub struct StaticResetPasswordStruct<'a> {
        pub check_nonce_infra: StaticCheckAuthNonceStruct<'a>,
        pub clock_infra: StaticAuthClockStruct,
        pub user_infra: StaticAuthUserStruct<'a>,
        pub password_infra: StaticAuthUserPasswordStruct<'a>,
        pub request_decoder: StaticResetPasswordRequestDecoder,
        pub token_decoder: StaticResetTokenDecoder,
    }

    impl<'a> ResetPasswordInfra for StaticResetPasswordStruct<'a> {
        type CheckNonceInfra = StaticCheckAuthNonceStruct<'a>;
        type ClockInfra = StaticAuthClockStruct;
        type UserInfra = StaticAuthUserStruct<'a>;
        type PasswordInfra = StaticAuthUserPasswordStruct<'a>;
        type TokenDecoder = StaticResetTokenDecoder;
        type RequestDecoder = StaticResetPasswordRequestDecoder;

        fn extract(
            self,
        ) -> (
            Self::CheckNonceInfra,
            Self::ClockInfra,
            Self::UserInfra,
            Self::PasswordInfra,
            Self::RequestDecoder,
            Self::TokenDecoder,
        ) {
            (
                self.check_nonce_infra,
                self.clock_infra,
                self.user_infra,
                self.password_infra,
                self.request_decoder,
                self.token_decoder,
            )
        }
    }
}
