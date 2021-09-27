use crate::{
    auth::user::remote::kernel::data::{AuthUser, AuthUserId},
    z_details::_common::repository::data::RepositoryError,
};

#[async_trait::async_trait]
pub trait AuthUserRepository {
    async fn get(&self, id: &AuthUserId) -> Result<Option<AuthUser>, RepositoryError>;
}
