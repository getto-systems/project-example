pub(in crate::auth) mod request_decoder;
pub(in crate::auth) mod token_decoder;

use crate::auth::{
    _auth::x_outside_feature::feature::AuthOutsideFeature,
    auth_ticket::_auth::kernel::init::{clock::ChronoAuthClock, CheckAuthNonceStruct},
    auth_user::_auth::kernel::init::user_repository::MysqlAuthUserRepository,
    password::_auth::kernel::init::{
        password_hasher::Argon2PasswordHasher, password_repository::MysqlAuthUserPasswordRepository,
    },
};
use token_decoder::JwtResetTokenDecoder;
use tonic::metadata::MetadataMap;

use crate::auth::password::reset::_auth::reset::infra::ResetPasswordInfra;

pub struct ResetPasswordStruct<'a> {
    check_nonce_infra: CheckAuthNonceStruct<'a>,
    clock: ChronoAuthClock,
    user_repository: MysqlAuthUserRepository<'a>,
    password_repository: MysqlAuthUserPasswordRepository<'a>,
    token_decoder: JwtResetTokenDecoder<'a>,
}

impl<'a> ResetPasswordStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature, metadata: &'a MetadataMap) -> Self {
        Self {
            check_nonce_infra: CheckAuthNonceStruct::new(feature, metadata),
            clock: ChronoAuthClock::new(),
            user_repository: MysqlAuthUserRepository::new(&feature.store.mysql),
            password_repository: MysqlAuthUserPasswordRepository::new(&feature.store.mysql),
            token_decoder: JwtResetTokenDecoder::new(&feature.key),
        }
    }
}

impl<'a> ResetPasswordInfra for ResetPasswordStruct<'a> {
    type CheckNonceInfra = CheckAuthNonceStruct<'a>;
    type Clock = ChronoAuthClock;
    type UserRepository = MysqlAuthUserRepository<'a>;
    type PasswordRepository = MysqlAuthUserPasswordRepository<'a>;
    type PasswordHasher = Argon2PasswordHasher;
    type TokenDecoder = JwtResetTokenDecoder<'a>;

    fn check_nonce_infra(&self) -> &Self::CheckNonceInfra {
        &self.check_nonce_infra
    }
    fn clock(&self) -> &Self::Clock {
        &self.clock
    }
    fn user_repository(&self) -> &Self::UserRepository {
        &self.user_repository
    }
    fn password_repository(&self) -> &Self::PasswordRepository {
        &self.password_repository
    }
    fn token_decoder(&self) -> &Self::TokenDecoder {
        &self.token_decoder
    }
}

#[cfg(test)]
pub mod test {
    use super::token_decoder::test::StaticResetTokenDecoder;
    use crate::auth::{
        auth_ticket::_auth::kernel::init::{
            clock::test::StaticChronoAuthClock, test::StaticCheckAuthNonceStruct,
        },
        auth_user::_auth::kernel::init::user_repository::test::MemoryAuthUserRepository,
        password::_auth::kernel::init::{
            password_hasher::test::PlainPasswordHasher,
            password_repository::test::MemoryAuthUserPasswordRepository,
        },
    };

    use super::super::infra::ResetPasswordInfra;

    pub struct StaticResetPasswordStruct<'a> {
        pub check_nonce_infra: StaticCheckAuthNonceStruct<'a>,
        pub clock: StaticChronoAuthClock,
        pub user_repository: MemoryAuthUserRepository<'a>,
        pub password_repository: MemoryAuthUserPasswordRepository<'a>,
        pub token_decoder: StaticResetTokenDecoder,
    }

    impl<'a> ResetPasswordInfra for StaticResetPasswordStruct<'a> {
        type CheckNonceInfra = StaticCheckAuthNonceStruct<'a>;
        type Clock = StaticChronoAuthClock;
        type UserRepository = MemoryAuthUserRepository<'a>;
        type PasswordRepository = MemoryAuthUserPasswordRepository<'a>;
        type PasswordHasher = PlainPasswordHasher;
        type TokenDecoder = StaticResetTokenDecoder;

        fn check_nonce_infra(&self) -> &Self::CheckNonceInfra {
            &self.check_nonce_infra
        }
        fn clock(&self) -> &Self::Clock {
            &self.clock
        }
        fn user_repository(&self) -> &Self::UserRepository {
            &self.user_repository
        }
        fn password_repository(&self) -> &Self::PasswordRepository {
            &self.password_repository
        }
        fn token_decoder(&self) -> &Self::TokenDecoder {
            &self.token_decoder
        }
    }
}
