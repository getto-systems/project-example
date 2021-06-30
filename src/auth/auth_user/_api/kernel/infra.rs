use super::data::{AuthUser, AuthUserId};
use crate::z_details::_api::repository::data::RepositoryError;

pub trait AuthUserInfra {
    type UserRepository: AuthUserRepository;

    fn user_repository(&self) -> &Self::UserRepository;
}

pub trait AuthUserRepository {
    fn get(&self, id: &AuthUserId) -> Result<Option<AuthUser>, RepositoryError>;
}
