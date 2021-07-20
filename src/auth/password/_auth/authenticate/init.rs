mod request_decoder;

use tonic::metadata::MetadataMap;

use crate::auth::_auth::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::password::_common::y_protobuf::service::AuthenticatePasswordRequestPb;
use crate::auth::{
    auth_ticket::_auth::kernel::init::CheckAuthNonceStruct,
    auth_user::_auth::kernel::init::AuthUserStruct,
    password::_auth::kernel::init::AuthUserPasswordStruct,
};
use request_decoder::TonicAuthenticatePasswordRequestDecoder;

use super::infra::AuthenticatePasswordInfra;

pub struct AuthenticatePasswordStruct<'a> {
    check_nonce_infra: CheckAuthNonceStruct<'a>,
    user_infra: AuthUserStruct<'a>,
    password_infra: AuthUserPasswordStruct<'a>,
    request_decoder: TonicAuthenticatePasswordRequestDecoder,
}

impl<'a> AuthenticatePasswordStruct<'a> {
    pub fn new(
        feature: &'a AuthOutsideFeature,
        metadata: &'a MetadataMap,
        request: AuthenticatePasswordRequestPb,
    ) -> Self {
        Self {
            check_nonce_infra: CheckAuthNonceStruct::new(feature, metadata.clone()),
            user_infra: AuthUserStruct::new(feature),
            password_infra: AuthUserPasswordStruct::new(feature),
            request_decoder: TonicAuthenticatePasswordRequestDecoder::new(request),
        }
    }
}

impl<'a> AuthenticatePasswordInfra for AuthenticatePasswordStruct<'a> {
    type CheckNonceInfra = CheckAuthNonceStruct<'a>;
    type UserInfra = AuthUserStruct<'a>;
    type PasswordInfra = AuthUserPasswordStruct<'a>;
    type RequestDecoder = TonicAuthenticatePasswordRequestDecoder;

    fn extract(
        self,
    ) -> (
        Self::CheckNonceInfra,
        Self::UserInfra,
        Self::PasswordInfra,
        Self::RequestDecoder,
    ) {
        (
            self.check_nonce_infra,
            self.user_infra,
            self.password_infra,
            self.request_decoder,
        )
    }
}

#[cfg(test)]
pub mod test {
    pub use super::request_decoder::test::StaticAuthenticatePasswordRequestDecoder;

    use super::super::infra::AuthenticatePasswordInfra;
    use crate::auth::{
        auth_ticket::_auth::kernel::init::test::StaticCheckAuthNonceStruct,
        auth_user::_auth::kernel::init::test::StaticAuthUserStruct,
        password::_auth::kernel::init::test::StaticAuthUserPasswordStruct,
    };

    pub struct StaticAuthenticatePasswordStruct<'a> {
        pub check_nonce_infra: StaticCheckAuthNonceStruct<'a>,
        pub user_infra: StaticAuthUserStruct<'a>,
        pub password_infra: StaticAuthUserPasswordStruct<'a>,
        pub request_decoder: StaticAuthenticatePasswordRequestDecoder,
    }

    impl<'a> AuthenticatePasswordInfra for StaticAuthenticatePasswordStruct<'a> {
        type CheckNonceInfra = StaticCheckAuthNonceStruct<'a>;
        type UserInfra = StaticAuthUserStruct<'a>;
        type PasswordInfra = StaticAuthUserPasswordStruct<'a>;
        type RequestDecoder = StaticAuthenticatePasswordRequestDecoder;

        fn extract(
            self,
        ) -> (
            Self::CheckNonceInfra,
            Self::UserInfra,
            Self::PasswordInfra,
            Self::RequestDecoder,
        ) {
            (
                self.check_nonce_infra,
                self.user_infra,
                self.password_infra,
                self.request_decoder,
            )
        }
    }
}
