pub(in crate::auth) mod user_repository;

use crate::auth::_auth::x_outside_feature::feature::AuthOutsideFeature;

use user_repository::MysqlAuthUserRepository;

use crate::auth::auth_user::_auth::kernel::infra::AuthUserInfra;

pub struct AuthUserStruct<'a> {
    user_repository: MysqlAuthUserRepository<'a>,
}

impl<'a> AuthUserStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature) -> Self {
        Self {
            user_repository: MysqlAuthUserRepository::new(&feature.store.mysql),
        }
    }
}

impl<'a> AuthUserInfra for AuthUserStruct<'a> {
    type UserRepository = MysqlAuthUserRepository<'a>;

    fn user_repository(&self) -> &Self::UserRepository {
        &self.user_repository
    }
}

#[cfg(test)]
pub mod test {
    pub use super::user_repository::test::{
        MemoryAuthUserMap, MemoryAuthUserRepository, MemoryAuthUserStore,
    };

    use crate::auth::auth_user::_auth::kernel::infra::AuthUserInfra;

    pub struct StaticAuthUserStruct<'a> {
        pub user_repository: MemoryAuthUserRepository<'a>,
    }

    impl<'a> AuthUserInfra for StaticAuthUserStruct<'a> {
        type UserRepository = MemoryAuthUserRepository<'a>;

        fn user_repository(&self) -> &Self::UserRepository {
            &self.user_repository
        }
    }
}
