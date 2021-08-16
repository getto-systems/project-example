pub(in crate::auth) mod password_hasher;
pub(in crate::auth) mod password_matcher;
pub(in crate::auth) mod password_repository;

use crate::auth::_auth::x_outside_feature::feature::AuthOutsideFeature;

use password_hasher::Argon2PasswordHasher;
use password_repository::MysqlAuthUserPasswordRepository;

use super::infra::AuthUserPasswordHashInfra;

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
    pub use super::password_repository::test::{
        MemoryAuthUserPasswordMap, MemoryAuthUserPasswordRepository, MemoryAuthUserPasswordStore,
    };

    use super::super::infra::AuthUserPasswordHashInfra;

    pub struct StaticAuthUserPasswordStruct<'a> {
        pub password_repository: MemoryAuthUserPasswordRepository<'a>,
    }

    impl<'a> AuthUserPasswordHashInfra for StaticAuthUserPasswordStruct<'a> {
        type PasswordRepository = MemoryAuthUserPasswordRepository<'a>;
        type PasswordHasher = PlainPasswordHasher;

        fn password_repository(&self) -> &Self::PasswordRepository {
            &self.password_repository
        }
    }
}
