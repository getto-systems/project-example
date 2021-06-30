mod user_repository;

use crate::auth::_api::x_outside_feature::feature::AuthOutsideFeature;

use user_repository::MemoryAuthUserRepository;

pub use user_repository::{MemoryAuthUserMap, MemoryAuthUserStore};

use crate::auth::auth_user::_api::kernel::infra::AuthUserInfra;

pub struct AuthUserStruct<'a> {
    user_repository: MemoryAuthUserRepository<'a>,
}

impl<'a> AuthUserStruct<'a> {
    pub fn new(feature: &'a AuthOutsideFeature) -> Self {
        Self {
            user_repository: MemoryAuthUserRepository::new(&feature.store.user),
        }
    }
}

impl<'a> AuthUserInfra for AuthUserStruct<'a> {
    type UserRepository = MemoryAuthUserRepository<'a>;

    fn user_repository(&self) -> &Self::UserRepository {
        &self.user_repository
    }
}

#[cfg(test)]
pub mod test {
    pub use super::user_repository::{
        MemoryAuthUserMap, MemoryAuthUserRepository, MemoryAuthUserStore,
    };

    use crate::auth::auth_user::_api::kernel::infra::AuthUserInfra;

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
