pub mod request_decoder;

use tonic::metadata::MetadataMap;

use crate::auth::remote::x_outside_feature::auth::feature::AuthOutsideFeature;

use crate::auth::{
    ticket::remote::check_nonce::init::CheckAuthNonceStruct,
    user::{
        password::remote::kernel::init::{
            password_matcher::Argon2PasswordMatcher,
            password_repository::MysqlAuthUserPasswordRepository,
        },
        remote::kernel::init::user_repository::MysqlAuthUserRepository,
    },
};

use super::infra::AuthenticatePasswordInfra;

pub struct AuthenticatePasswordStruct<'a> {
    check_nonce_infra: CheckAuthNonceStruct<'a>,
    user_repository: MysqlAuthUserRepository<'a>,
    password_repository: MysqlAuthUserPasswordRepository<'a>,
}

impl<'a> AuthenticatePasswordStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature, metadata: &'a MetadataMap) -> Self {
        Self {
            check_nonce_infra: CheckAuthNonceStruct::new(feature, metadata),
            user_repository: MysqlAuthUserRepository::new(&feature.store.mysql),
            password_repository: MysqlAuthUserPasswordRepository::new(&feature.store.mysql),
        }
    }
}

impl<'a> AuthenticatePasswordInfra for AuthenticatePasswordStruct<'a> {
    type CheckNonceInfra = CheckAuthNonceStruct<'a>;
    type UserRepository = MysqlAuthUserRepository<'a>;
    type PasswordRepository = MysqlAuthUserPasswordRepository<'a>;
    type PasswordMatcher = Argon2PasswordMatcher;

    fn check_nonce_infra(&self) -> &Self::CheckNonceInfra {
        &self.check_nonce_infra
    }
    fn user_repository(&self) -> &Self::UserRepository {
        &self.user_repository
    }
    fn password_repository(&self) -> &Self::PasswordRepository {
        &self.password_repository
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::{
        ticket::remote::check_nonce::init::test::StaticCheckAuthNonceStruct,
        user::{
            password::remote::kernel::init::{
                password_matcher::test::PlainPasswordMatcher,
                password_repository::test::MemoryAuthUserPasswordRepository,
            },
            remote::kernel::init::user_repository::test::MemoryAuthUserRepository,
        },
    };

    use super::super::infra::AuthenticatePasswordInfra;

    pub struct StaticAuthenticatePasswordStruct<'a> {
        pub check_nonce_infra: StaticCheckAuthNonceStruct<'a>,
        pub user_repository: MemoryAuthUserRepository<'a>,
        pub password_repository: MemoryAuthUserPasswordRepository<'a>,
    }

    impl<'a> AuthenticatePasswordInfra for StaticAuthenticatePasswordStruct<'a> {
        type CheckNonceInfra = StaticCheckAuthNonceStruct<'a>;
        type UserRepository = MemoryAuthUserRepository<'a>;
        type PasswordRepository = MemoryAuthUserPasswordRepository<'a>;
        type PasswordMatcher = PlainPasswordMatcher;

        fn check_nonce_infra(&self) -> &Self::CheckNonceInfra {
            &self.check_nonce_infra
        }
        fn user_repository(&self) -> &Self::UserRepository {
            &self.user_repository
        }
        fn password_repository(&self) -> &Self::PasswordRepository {
            &self.password_repository
        }
    }
}
