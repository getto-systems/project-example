use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use super::infra::{
    messenger::ProtobufAuthenticateMessenger, password_hash::PassthroughPasswordHash,
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
    password_hash: PassthroughPasswordHash,
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
            password_hash: PassthroughPasswordHash::new(),
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
    type PasswordHash = PassthroughPasswordHash;
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
