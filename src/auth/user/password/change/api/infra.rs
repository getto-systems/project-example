use crate::auth::user::password::kernel::infra::{AuthUserPasswordHasher, AuthUserPasswordMatcher};

use crate::auth::user::{
    kernel::data::AuthUserId, password::change::api::data::ChangePasswordRepositoryError,
};

pub trait ChangePasswordRequestDecoder {
    fn decode(self) -> ChangePasswordFieldsExtract;
}

pub struct ChangePasswordFieldsExtract {
    pub current_password: String,
    pub new_password: String,
}

#[async_trait::async_trait]
pub trait ChangePasswordRepository {
    async fn change_password<'a>(
        &self,
        user_id: &'a AuthUserId,
        matcher: impl 'a + AuthUserPasswordMatcher,
        hasher: impl 'a + AuthUserPasswordHasher,
    ) -> Result<(), ChangePasswordRepositoryError>;
}
