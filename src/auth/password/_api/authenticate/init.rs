use actix_web::HttpRequest;

use crate::auth::{
    _api::x_outside_feature::feature::AuthOutsideFeature,
    auth_ticket::_api::kernel::init::CheckAuthNonceStruct,
};

use super::infra::{
    messenger::ProtobufAuthenticateMessenger, password_hash::Argon2PasswordHash,
    password_repository::MemoryAuthUserPasswordRepository, AuthenticatePasswordInfra,
};
use crate::auth::{
    auth_ticket::_api::kernel::infra::clock::ChronoAuthClock,
    auth_user::_api::kernel::infra::user_repository::MemoryAuthUserRepository,
};

pub struct AuthenticatePasswordStruct<'a> {
    check_nonce_infra: CheckAuthNonceStruct<'a>,
    clock: ChronoAuthClock,
    password_hash: Argon2PasswordHash,
    password_repository: MemoryAuthUserPasswordRepository<'a>,
    user_repository: MemoryAuthUserRepository<'a>,
    messenger: ProtobufAuthenticateMessenger,
}

impl<'a> AuthenticatePasswordStruct<'a> {
    pub fn new(request: HttpRequest, body: String, feature: &'a AuthOutsideFeature) -> Self {
        Self {
            check_nonce_infra: CheckAuthNonceStruct::new(request, feature),
            clock: ChronoAuthClock::new(),
            password_hash: Argon2PasswordHash::new(),
            password_repository: MemoryAuthUserPasswordRepository::new(
                &feature.store.user_password,
            ),
            user_repository: MemoryAuthUserRepository::new(&feature.store.user),
            messenger: ProtobufAuthenticateMessenger::new(body),
        }
    }
}

impl<'a> AuthenticatePasswordInfra for AuthenticatePasswordStruct<'a> {
    type CheckNonceInfra = CheckAuthNonceStruct<'a>;
    type Clock = ChronoAuthClock;
    type PasswordHash = Argon2PasswordHash;
    type PasswordRepository = MemoryAuthUserPasswordRepository<'a>;
    type UserRepository = MemoryAuthUserRepository<'a>;
    type Messenger = ProtobufAuthenticateMessenger;

    fn check_nonce_infra(&self) -> &Self::CheckNonceInfra {
        &self.check_nonce_infra
    }
    fn clock(&self) -> &Self::Clock {
        &self.clock
    }
    fn password_hash(&self) -> &Self::PasswordHash {
        &self.password_hash
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
        messenger::test::StaticAuthenticateMessenger, password_hash::test::PlainPasswordHash,
        password_repository::MemoryAuthUserPasswordRepository, AuthenticatePasswordInfra,
    };
    use crate::auth::{
        auth_ticket::_api::kernel::infra::clock::test::StaticChronoAuthClock,
        auth_user::_api::kernel::infra::user_repository::MemoryAuthUserRepository,
    };

    pub struct StaticAuthenticatePasswordStruct<'a> {
        check_nonce_infra: StaticCheckAuthNonceStruct<'a>,
        clock: StaticChronoAuthClock,
        password_hash: PlainPasswordHash,
        password_repository: MemoryAuthUserPasswordRepository<'a>,
        user_repository: MemoryAuthUserRepository<'a>,
        messenger: StaticAuthenticateMessenger,
    }

    pub struct StaticAuthenticatePasswordParam<'a> {
        pub check_nonce_infra: StaticCheckAuthNonceStruct<'a>,
        pub clock: StaticChronoAuthClock,
        pub password_repository: MemoryAuthUserPasswordRepository<'a>,
        pub user_repository: MemoryAuthUserRepository<'a>,
        pub messenger: StaticAuthenticateMessenger,
    }

    impl<'a> StaticAuthenticatePasswordStruct<'a> {
        pub fn new(param: StaticAuthenticatePasswordParam<'a>) -> Self {
            Self {
                check_nonce_infra: param.check_nonce_infra,
                clock: param.clock,
                password_hash: PlainPasswordHash::new(),
                password_repository: param.password_repository,
                user_repository: param.user_repository,
                messenger: param.messenger,
            }
        }
    }

    impl<'a> AuthenticatePasswordInfra for StaticAuthenticatePasswordStruct<'a> {
        type CheckNonceInfra = StaticCheckAuthNonceStruct<'a>;
        type Clock = StaticChronoAuthClock;
        type PasswordHash = PlainPasswordHash;
        type PasswordRepository = MemoryAuthUserPasswordRepository<'a>;
        type UserRepository = MemoryAuthUserRepository<'a>;
        type Messenger = StaticAuthenticateMessenger;

        fn check_nonce_infra(&self) -> &Self::CheckNonceInfra {
            &self.check_nonce_infra
        }
        fn clock(&self) -> &Self::Clock {
            &self.clock
        }
        fn password_hash(&self) -> &Self::PasswordHash {
            &self.password_hash
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
