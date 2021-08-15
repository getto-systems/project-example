mod password_hasher;
mod password_matcher;
mod password_repository;

use crate::auth::_auth::x_outside_feature::feature::AuthOutsideFeature;

use password_hasher::Argon2PasswordHasher;
use password_matcher::Argon2PasswordMatcher;
use password_repository::MysqlAuthUserPasswordRepository;

use super::infra::{
    AuthUserPasswordHashInfra, AuthUserPasswordInfra, AuthUserPasswordMatchInfra,
};

pub struct AuthUserPasswordStruct<'a> {
    password_repository: MysqlAuthUserPasswordRepository<'a>,
}

impl<'a> AuthUserPasswordStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature) -> Self {
        Self {
            password_repository: MysqlAuthUserPasswordRepository::new(&feature.store.mysql),
        }
    }
}

impl<'a> AuthUserPasswordInfra for AuthUserPasswordStruct<'a> {
    type PasswordRepository = MysqlAuthUserPasswordRepository<'a>;

    fn password_repository(&self) -> &Self::PasswordRepository {
        &self.password_repository
    }
}

impl<'a> AuthUserPasswordMatchInfra for AuthUserPasswordStruct<'a> {
    type PasswordRepository = MysqlAuthUserPasswordRepository<'a>;
    type PasswordMatcher = Argon2PasswordMatcher;

    fn password_repository(&self) -> &Self::PasswordRepository {
        &self.password_repository
    }
}

impl<'a> AuthUserPasswordHashInfra for AuthUserPasswordStruct<'a> {
    type PasswordRepository = MysqlAuthUserPasswordRepository<'a>;
    type PasswordHasher = Argon2PasswordHasher;

    fn password_repository(&self) -> &Self::PasswordRepository {
        &self.password_repository
    }
}

#[cfg(test)]
pub mod test {
    use super::password_hasher::test::PlainPasswordHasher;
    use super::password_matcher::test::PlainPasswordMatcher;
    pub use super::password_repository::test::{
        MemoryAuthUserPasswordMap, MemoryAuthUserPasswordRepository, MemoryAuthUserPasswordStore,
    };

    use super::super::infra::{
        AuthUserPasswordHashInfra, AuthUserPasswordInfra, AuthUserPasswordMatchInfra,
    };

    pub struct StaticAuthUserPasswordStruct<'a> {
        pub password_repository: MemoryAuthUserPasswordRepository<'a>,
    }

    impl<'a> AuthUserPasswordInfra for StaticAuthUserPasswordStruct<'a> {
        type PasswordRepository = MemoryAuthUserPasswordRepository<'a>;

        fn password_repository(&self) -> &Self::PasswordRepository {
            &self.password_repository
        }
    }

    impl<'a> AuthUserPasswordMatchInfra for StaticAuthUserPasswordStruct<'a> {
        type PasswordRepository = MemoryAuthUserPasswordRepository<'a>;
        type PasswordMatcher = PlainPasswordMatcher;

        fn password_repository(&self) -> &Self::PasswordRepository {
            &self.password_repository
        }
    }

    impl<'a> AuthUserPasswordHashInfra for StaticAuthUserPasswordStruct<'a> {
        type PasswordRepository = MemoryAuthUserPasswordRepository<'a>;
        type PasswordHasher = PlainPasswordHasher;

        fn password_repository(&self) -> &Self::PasswordRepository {
            &self.password_repository
        }
    }
}
