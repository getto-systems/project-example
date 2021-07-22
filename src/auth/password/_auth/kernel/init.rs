mod password_hasher;
mod password_matcher;
mod password_repository;

use crate::auth::_auth::x_outside_feature::feature::AuthOutsideFeature;
use crate::auth::password::_auth::kernel::infra::AuthUserPasswordHashInfra;

use password_hasher::Argon2PasswordHasher;
use password_matcher::Argon2PasswordMatcher;
use password_repository::MysqlAuthUserPasswordRepository;

use crate::auth::password::_auth::kernel::infra::{
    AuthUserPasswordHasher, AuthUserPasswordInfra, AuthUserPasswordMatchInfra,
    AuthUserPasswordMatcher, PlainPassword,
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

    fn extract(self) -> Self::PasswordRepository {
        self.password_repository
    }
}

impl<'a> AuthUserPasswordMatchInfra for AuthUserPasswordStruct<'a> {
    type PasswordRepository = MysqlAuthUserPasswordRepository<'a>;
    type PasswordMatcher = Argon2PasswordMatcher;

    fn extract(
        self,
        plain_password: PlainPassword,
    ) -> (Self::PasswordRepository, Self::PasswordMatcher) {
        (
            self.password_repository,
            Self::PasswordMatcher::new(plain_password),
        )
    }
}

impl<'a> AuthUserPasswordHashInfra for AuthUserPasswordStruct<'a> {
    type PasswordRepository = MysqlAuthUserPasswordRepository<'a>;
    type PasswordHasher = Argon2PasswordHasher;

    fn extract(
        self,
        plain_password: PlainPassword,
    ) -> (Self::PasswordRepository, Self::PasswordHasher) {
        (
            self.password_repository,
            Self::PasswordHasher::new(plain_password),
        )
    }
}

#[cfg(test)]
pub mod test {
    use super::password_hasher::test::PlainPasswordHasher;
    use super::password_matcher::test::PlainPasswordMatcher;
    pub use super::password_repository::test::{
        MemoryAuthUserPasswordMap, MemoryAuthUserPasswordRepository, MemoryAuthUserPasswordStore,
    };

    use crate::auth::password::_auth::kernel::infra::{
        AuthUserPasswordHashInfra, AuthUserPasswordHasher, AuthUserPasswordInfra,
        AuthUserPasswordMatchInfra, AuthUserPasswordMatcher, PlainPassword,
    };

    pub struct StaticAuthUserPasswordStruct<'a> {
        pub password_repository: MemoryAuthUserPasswordRepository<'a>,
    }

    impl<'a> AuthUserPasswordInfra for StaticAuthUserPasswordStruct<'a> {
        type PasswordRepository = MemoryAuthUserPasswordRepository<'a>;

        fn extract(self) -> Self::PasswordRepository {
            self.password_repository
        }
    }

    impl<'a> AuthUserPasswordMatchInfra for StaticAuthUserPasswordStruct<'a> {
        type PasswordRepository = MemoryAuthUserPasswordRepository<'a>;
        type PasswordMatcher = PlainPasswordMatcher;

        fn extract(
            self,
            plain_password: PlainPassword,
        ) -> (Self::PasswordRepository, Self::PasswordMatcher) {
            (
                self.password_repository,
                Self::PasswordMatcher::new(plain_password),
            )
        }
    }

    impl<'a> AuthUserPasswordHashInfra for StaticAuthUserPasswordStruct<'a> {
        type PasswordRepository = MemoryAuthUserPasswordRepository<'a>;
        type PasswordHasher = PlainPasswordHasher;

        fn extract(
            self,
            plain_password: PlainPassword,
        ) -> (Self::PasswordRepository, Self::PasswordHasher) {
            (
                self.password_repository,
                Self::PasswordHasher::new(plain_password),
            )
        }
    }
}
