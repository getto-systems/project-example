pub(in crate::auth) mod request_decoder;

use crate::auth::_auth::x_outside_feature::feature::AuthOutsideFeature;

use crate::auth::password::_auth::kernel::init::{
    password_hasher::Argon2PasswordHasher, password_matcher::Argon2PasswordMatcher,
    password_repository::MysqlAuthUserPasswordRepository,
};

use super::infra::ChangePasswordInfra;

pub struct ChangePasswordStruct<'a> {
    password_repository: MysqlAuthUserPasswordRepository<'a>,
}

impl<'a> ChangePasswordStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature) -> Self {
        Self {
            password_repository: MysqlAuthUserPasswordRepository::new(&feature.store.mysql),
        }
    }
}

impl<'a> ChangePasswordInfra for ChangePasswordStruct<'a> {
    type PasswordRepository = MysqlAuthUserPasswordRepository<'a>;
    type PasswordMatcher = Argon2PasswordMatcher;
    type PasswordHasher = Argon2PasswordHasher;

    fn password_repository(&self) -> &Self::PasswordRepository {
        &self.password_repository
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::password::_auth::kernel::init::{
        password_hasher::test::PlainPasswordHasher, password_matcher::test::PlainPasswordMatcher,
        password_repository::test::MemoryAuthUserPasswordRepository,
    };

    use super::super::infra::ChangePasswordInfra;

    pub struct StaticChangePasswordStruct<'a> {
        pub password_repository: MemoryAuthUserPasswordRepository<'a>,
    }

    impl<'a> ChangePasswordInfra for StaticChangePasswordStruct<'a> {
        type PasswordRepository = MemoryAuthUserPasswordRepository<'a>;
        type PasswordMatcher = PlainPasswordMatcher;
        type PasswordHasher = PlainPasswordHasher;

        fn password_repository(&self) -> &Self::PasswordRepository {
            &self.password_repository
        }
    }
}
