use super::data::{AuthUser, AuthUserId};
use crate::z_details::_common::repository::data::RepositoryError;

pub trait AuthUserInfra {
    type UserRepository: AuthUserRepository;

    fn user_repository(&self) -> &Self::UserRepository;
}

#[async_trait::async_trait]
pub trait AuthUserRepository {
    async fn get(&self, id: &AuthUserId) -> Result<Option<AuthUser>, RepositoryError>;
}
