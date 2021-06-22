use actix_web::HttpRequest;

use crate::auth::auth_ticket::_api::kernel::init::CheckAuthNonceStruct;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use super::infra::{messenger::ProtobufAuthenticatePasswordMessenger, AuthenticatePasswordInfra};
use crate::auth::{
    auth_ticket::_api::kernel::infra::clock::ChronoAuthClock,
    auth_user::_api::kernel::infra::user_repository::MemoryAuthUserRepository,
    password::_api::kernel::infra::{
        password_matcher::Argon2PasswordMatcher,
        password_repository::MemoryAuthUserPasswordRepository,
    },
};

pub(in crate::auth::password) struct AuthenticatePasswordStruct<'a> {
    check_nonce_infra: CheckAuthNonceStruct<'a>,
    clock: ChronoAuthClock,
    password_repository: MemoryAuthUserPasswordRepository<'a>,
    user_repository: MemoryAuthUserRepository<'a>,
    messenger: ProtobufAuthenticatePasswordMessenger,
}

impl<'a> AuthenticatePasswordStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature, request: &'a HttpRequest, body: String) -> Self {
        Self {
            check_nonce_infra: CheckAuthNonceStruct::new(feature, request),
            clock: ChronoAuthClock::new(),
            password_repository: MemoryAuthUserPasswordRepository::new(
                &feature.store.user_password,
            ),
            user_repository: MemoryAuthUserRepository::new(&feature.store.user),
            messenger: ProtobufAuthenticatePasswordMessenger::new(body),
        }
    }
}

impl<'a> AuthenticatePasswordInfra for AuthenticatePasswordStruct<'a> {
    type CheckNonceInfra = CheckAuthNonceStruct<'a>;
    type Clock = ChronoAuthClock;
    type PasswordMatcher = Argon2PasswordMatcher;
    type PasswordRepository = MemoryAuthUserPasswordRepository<'a>;
    type UserRepository = MemoryAuthUserRepository<'a>;
    type Messenger = ProtobufAuthenticatePasswordMessenger;

    fn check_nonce_infra(&self) -> &Self::CheckNonceInfra {
        &self.check_nonce_infra
    }
    fn clock(&self) -> &Self::Clock {
        &self.clock
    }
    fn password_repository(&self) -> &Self::PasswordRepository {
        &self.password_repository
    }
    fn user_repository(&self) -> &Self::UserRepository {
        &self.user_repository
    }
    fn messenger(&self) -> &Self::Messenger {
        &self.messenger
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::auth_ticket::_api::kernel::init::test::StaticCheckAuthNonceStruct;

    use super::super::infra::{
        messenger::test::StaticAuthenticatePasswordMessenger, AuthenticatePasswordInfra,
    };
    use crate::auth::{
        auth_ticket::_api::kernel::infra::clock::test::StaticChronoAuthClock,
        auth_user::_api::kernel::infra::user_repository::MemoryAuthUserRepository,
        password::_api::kernel::infra::{
            password_matcher::test::PlainPasswordMatcher,
            password_repository::MemoryAuthUserPasswordRepository,
        },
    };

    pub struct StaticAuthenticatePasswordStruct<'a> {
        pub check_nonce_infra: StaticCheckAuthNonceStruct<'a>,
        pub clock: StaticChronoAuthClock,
        pub password_repository: MemoryAuthUserPasswordRepository<'a>,
        pub user_repository: MemoryAuthUserRepository<'a>,
        pub messenger: StaticAuthenticatePasswordMessenger,
    }

    impl<'a> AuthenticatePasswordInfra for StaticAuthenticatePasswordStruct<'a> {
        type CheckNonceInfra = StaticCheckAuthNonceStruct<'a>;
        type Clock = StaticChronoAuthClock;
        type PasswordMatcher = PlainPasswordMatcher;
        type PasswordRepository = MemoryAuthUserPasswordRepository<'a>;
        type UserRepository = MemoryAuthUserRepository<'a>;
        type Messenger = StaticAuthenticatePasswordMessenger;

        fn check_nonce_infra(&self) -> &Self::CheckNonceInfra {
            &self.check_nonce_infra
        }
        fn clock(&self) -> &Self::Clock {
            &self.clock
        }
        fn password_repository(&self) -> &Self::PasswordRepository {
            &self.password_repository
        }
        fn user_repository(&self) -> &Self::UserRepository {
            &self.user_repository
        }
        fn messenger(&self) -> &Self::Messenger {
            &self.messenger
        }
    }
}
