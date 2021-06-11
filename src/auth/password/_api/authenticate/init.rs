use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use super::infra::{
    messenger::ProtobufAuthenticateMessenger, password_hash::Argon2PasswordHash,
    password_repository::MemoryAuthUserPasswordRepository, AuthenticatePasswordInfra,
};
use crate::auth::{
    auth_ticket::_api::kernel::infra::{
        clock::ChronoAuthClock, nonce_header::ActixWebAuthNonceHeader,
        nonce_repository::MemoryAuthNonceRepository, AuthNonceConfig,
    },
    auth_user::_api::kernel::infra::user_repository::MemoryAuthUserRepository,
};

pub struct AuthenticatePasswordStruct<'a> {
    nonce_config: AuthNonceConfig,
    clock: ChronoAuthClock,
    nonce_header: ActixWebAuthNonceHeader,
    nonce_repository: MemoryAuthNonceRepository<'a>,
    password_hash: Argon2PasswordHash,
    password_repository: MemoryAuthUserPasswordRepository<'a>,
    user_repository: MemoryAuthUserRepository<'a>,
    messenger: ProtobufAuthenticateMessenger,
}

impl<'a> AuthenticatePasswordStruct<'a> {
    pub fn new(request: HttpRequest, body: String, feature: &'a AuthOutsideFeature) -> Self {
        Self {
            nonce_config: AuthNonceConfig {
                nonce_expires: feature.config.ticket_expires,
            },
            clock: ChronoAuthClock::new(),
            nonce_header: ActixWebAuthNonceHeader::new(request),
            nonce_repository: MemoryAuthNonceRepository::new(&feature.store.nonce),
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
    type Clock = ChronoAuthClock;
    type NonceHeader = ActixWebAuthNonceHeader;
    type NonceRepository = MemoryAuthNonceRepository<'a>;
    type PasswordHash = Argon2PasswordHash;
    type PasswordRepository = MemoryAuthUserPasswordRepository<'a>;
    type UserRepository = MemoryAuthUserRepository<'a>;
    type Messenger = ProtobufAuthenticateMessenger;

    fn nonce_config(&self) -> &AuthNonceConfig {
        &self.nonce_config
    }
    fn clock(&self) -> &Self::Clock {
        &self.clock
    }
    fn nonce_header(&self) -> &Self::NonceHeader {
        &self.nonce_header
    }
    fn nonce_repository(&self) -> &Self::NonceRepository {
        &self.nonce_repository
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
    use super::super::infra::{
        messenger::test::StaticAuthenticateMessenger, password_hash::test::PlainPasswordHash,
        password_repository::MemoryAuthUserPasswordRepository, AuthenticatePasswordInfra,
    };
    use crate::auth::{
        auth_ticket::_api::kernel::infra::{
            clock::test::StaticChronoAuthClock, nonce_header::test::StaticAuthNonceHeader,
            nonce_repository::MemoryAuthNonceRepository, AuthNonceConfig,
        },
        auth_user::_api::kernel::infra::user_repository::MemoryAuthUserRepository,
    };

    pub struct StaticAuthenticatePasswordStruct<'a> {
        nonce_config: AuthNonceConfig,
        clock: StaticChronoAuthClock,
        nonce_header: StaticAuthNonceHeader,
        nonce_repository: MemoryAuthNonceRepository<'a>,
        password_hash: PlainPasswordHash,
        password_repository: MemoryAuthUserPasswordRepository<'a>,
        user_repository: MemoryAuthUserRepository<'a>,
        messenger: StaticAuthenticateMessenger,
    }

    pub struct StaticAuthenticatePasswordParam<'a> {
        pub nonce_config: AuthNonceConfig,
        pub clock: StaticChronoAuthClock,
        pub nonce_header: StaticAuthNonceHeader,
        pub nonce_repository: MemoryAuthNonceRepository<'a>,
        pub password_repository: MemoryAuthUserPasswordRepository<'a>,
        pub user_repository: MemoryAuthUserRepository<'a>,
        pub messenger: StaticAuthenticateMessenger,
    }

    impl<'a> StaticAuthenticatePasswordStruct<'a> {
        pub fn new(param: StaticAuthenticatePasswordParam<'a>) -> Self {
            Self {
                nonce_config: param.nonce_config,
                clock: param.clock,
                nonce_header: param.nonce_header,
                nonce_repository: param.nonce_repository,
                password_hash: PlainPasswordHash::new(),
                password_repository: param.password_repository,
                user_repository: param.user_repository,
                messenger: param.messenger,
            }
        }
    }

    impl<'a> AuthenticatePasswordInfra for StaticAuthenticatePasswordStruct<'a> {
        type Clock = StaticChronoAuthClock;
        type NonceHeader = StaticAuthNonceHeader;
        type NonceRepository = MemoryAuthNonceRepository<'a>;
        type PasswordHash = PlainPasswordHash;
        type PasswordRepository = MemoryAuthUserPasswordRepository<'a>;
        type UserRepository = MemoryAuthUserRepository<'a>;
        type Messenger = StaticAuthenticateMessenger;

        fn nonce_config(&self) -> &AuthNonceConfig {
            &self.nonce_config
        }
        fn clock(&self) -> &Self::Clock {
            &self.clock
        }
        fn nonce_header(&self) -> &Self::NonceHeader {
            &self.nonce_header
        }
        fn nonce_repository(&self) -> &Self::NonceRepository {
            &self.nonce_repository
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
