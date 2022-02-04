use crate::{
    auth::user::kernel::data::{AuthUser, AuthUserId},
    z_lib::api::repository::data::RepositoryError,
};

#[async_trait::async_trait]
pub trait AuthUserRepository {
    async fn get(&self, id: &AuthUserId) -> Result<Option<AuthUser>, RepositoryError>;
}
