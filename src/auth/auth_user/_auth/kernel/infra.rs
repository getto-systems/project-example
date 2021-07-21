use crate::{
    auth::auth_user::_common::kernel::data::{AuthUser, AuthUserId},
    z_details::_common::repository::data::RepositoryError,
};

pub trait AuthUserInfra {
    type UserRepository: AuthUserRepository;

    fn extract(self) -> Self::UserRepository;
}

#[async_trait::async_trait]
pub trait AuthUserRepository {
    async fn get(&self, id: &AuthUserId) -> Result<Option<AuthUser>, RepositoryError>;
}
