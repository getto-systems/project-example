mod messenger;

use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::{
    auth_ticket::_api::kernel::init::CheckAuthNonceStruct,
    auth_user::_api::kernel::init::AuthUserStruct,
    password::_api::kernel::init::{Argon2PasswordMatcher, MemoryAuthUserPasswordRepository},
};
use messenger::ProtobufAuthenticatePasswordMessenger;

use super::infra::AuthenticatePasswordInfra;

pub struct AuthenticatePasswordStruct<'a> {
    check_nonce_infra: CheckAuthNonceStruct<'a>,
    user_infra: AuthUserStruct<'a>,
    password_repository: MemoryAuthUserPasswordRepository<'a>,
    messenger: ProtobufAuthenticatePasswordMessenger,
}

impl<'a> AuthenticatePasswordStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature, request: &'a HttpRequest, body: String) -> Self {
        Self {
            check_nonce_infra: CheckAuthNonceStruct::new(feature, request),
            user_infra: AuthUserStruct::new(feature),
            password_repository: MemoryAuthUserPasswordRepository::new(
                &feature.store.user_password,
            ),
            messenger: ProtobufAuthenticatePasswordMessenger::new(body),
        }
    }
}

impl<'a> AuthenticatePasswordInfra for AuthenticatePasswordStruct<'a> {
    type CheckNonceInfra = CheckAuthNonceStruct<'a>;
    type UserInfra = AuthUserStruct<'a>;
    type PasswordMatcher = Argon2PasswordMatcher;
    type PasswordRepository = MemoryAuthUserPasswordRepository<'a>;
    type Messenger = ProtobufAuthenticatePasswordMessenger;

    fn check_nonce_infra(&self) -> &Self::CheckNonceInfra {
        &self.check_nonce_infra
    }
    fn user_infra(&self) -> &Self::UserInfra {
        &self.user_infra
    }
    fn password_repository(&self) -> &Self::PasswordRepository {
        &self.password_repository
    }
    fn messenger(&self) -> &Self::Messenger {
        &self.messenger
    }
}

#[cfg(test)]
pub mod test {
    pub use super::messenger::test::StaticAuthenticatePasswordMessenger;

    use super::super::infra::AuthenticatePasswordInfra;
    use crate::auth::{
        auth_ticket::_api::kernel::init::test::StaticCheckAuthNonceStruct,
        auth_user::_api::kernel::init::test::StaticAuthUserStruct,
        password::_api::kernel::init::test::{
            MemoryAuthUserPasswordRepository, PlainPasswordMatcher,
        },
    };

    pub struct StaticAuthenticatePasswordStruct<'a> {
        pub check_nonce_infra: StaticCheckAuthNonceStruct<'a>,
        pub user_infra: StaticAuthUserStruct<'a>,
        pub password_repository: MemoryAuthUserPasswordRepository<'a>,
        pub messenger: StaticAuthenticatePasswordMessenger,
    }

    impl<'a> AuthenticatePasswordInfra for StaticAuthenticatePasswordStruct<'a> {
        type CheckNonceInfra = StaticCheckAuthNonceStruct<'a>;
        type UserInfra = StaticAuthUserStruct<'a>;
        type PasswordMatcher = PlainPasswordMatcher;
        type PasswordRepository = MemoryAuthUserPasswordRepository<'a>;
        type Messenger = StaticAuthenticatePasswordMessenger;

        fn check_nonce_infra(&self) -> &Self::CheckNonceInfra {
            &self.check_nonce_infra
        }
        fn user_infra(&self) -> &Self::UserInfra {
            &self.user_infra
        }
        fn password_repository(&self) -> &Self::PasswordRepository {
            &self.password_repository
        }
        fn messenger(&self) -> &Self::Messenger {
            &self.messenger
        }
    }
}
