use actix_web::HttpRequest;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::{
    auth_ticket::_api::kernel::init::{CheckAuthNonceStruct, ChronoAuthClock},
    auth_user::_api::kernel::init::MemoryAuthUserRepository,
};

use crate::auth::password::{
    _api::kernel::infra::{
        password_hasher::Argon2PasswordHasher,
        password_repository::MemoryAuthUserPasswordRepository,
    },
    reset::_api::reset::infra::{
        messenger::ProtobufResetPasswordMessenger, token_decoder::JwtResetTokenDecoder,
        ResetPasswordInfra,
    },
};

pub struct ResetPasswordStruct<'a> {
    check_nonce_infra: CheckAuthNonceStruct<'a>,
    clock: ChronoAuthClock,
    password_repository: MemoryAuthUserPasswordRepository<'a>,
    user_repository: MemoryAuthUserRepository<'a>,
    token_decoder: JwtResetTokenDecoder<'a>,
    messenger: ProtobufResetPasswordMessenger,
}

impl<'a> ResetPasswordStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature, request: &'a HttpRequest, body: String) -> Self {
        Self {
            check_nonce_infra: CheckAuthNonceStruct::new(feature, request),
            clock: ChronoAuthClock::new(),
            password_repository: MemoryAuthUserPasswordRepository::new(
                &feature.store.user_password,
            ),
            user_repository: MemoryAuthUserRepository::new(&feature.store.user),
            token_decoder: JwtResetTokenDecoder::new(&feature.secret.reset_token.decoding_key),
            messenger: ProtobufResetPasswordMessenger::new(body),
        }
    }
}

impl<'a> ResetPasswordInfra for ResetPasswordStruct<'a> {
    type CheckNonceInfra = CheckAuthNonceStruct<'a>;
    type Clock = ChronoAuthClock;
    type PasswordRepository = MemoryAuthUserPasswordRepository<'a>;
    type UserRepository = MemoryAuthUserRepository<'a>;
    type PasswordHasher = Argon2PasswordHasher;
    type TokenDecoder = JwtResetTokenDecoder<'a>;
    type Messenger = ProtobufResetPasswordMessenger;

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
    fn token_decoder(&self) -> &Self::TokenDecoder {
        &self.token_decoder
    }
    fn messenger(&self) -> &Self::Messenger {
        &self.messenger
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::{
        auth_ticket::_api::kernel::init::test::{
            StaticCheckAuthNonceStruct, StaticChronoAuthClock,
        },
        auth_user::_api::kernel::init::test::MemoryAuthUserRepository,
    };

    use crate::auth::password::{
        _api::kernel::infra::{
            password_hasher::test::PlainPasswordHasher,
            password_repository::MemoryAuthUserPasswordRepository,
        },
        reset::_api::reset::infra::{
            messenger::test::StaticResetPasswordMessenger,
            token_decoder::test::StaticResetTokenDecoder, ResetPasswordInfra,
        },
    };

    pub struct StaticResetPasswordStruct<'a> {
        pub check_nonce_infra: StaticCheckAuthNonceStruct<'a>,
        pub clock: StaticChronoAuthClock,
        pub password_repository: MemoryAuthUserPasswordRepository<'a>,
        pub user_repository: MemoryAuthUserRepository<'a>,
        pub token_decoder: StaticResetTokenDecoder,
        pub messenger: StaticResetPasswordMessenger,
    }

    impl<'a> ResetPasswordInfra for StaticResetPasswordStruct<'a> {
        type CheckNonceInfra = StaticCheckAuthNonceStruct<'a>;
        type Clock = StaticChronoAuthClock;
        type PasswordRepository = MemoryAuthUserPasswordRepository<'a>;
        type UserRepository = MemoryAuthUserRepository<'a>;
        type PasswordHasher = PlainPasswordHasher;
        type TokenDecoder = StaticResetTokenDecoder;
        type Messenger = StaticResetPasswordMessenger;

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
        fn token_decoder(&self) -> &Self::TokenDecoder {
            &self.token_decoder
        }
        fn messenger(&self) -> &Self::Messenger {
            &self.messenger
        }
    }
}
