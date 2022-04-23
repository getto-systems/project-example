use crate::{
    auth::user::{
        account::unregister::data::ValidateUnregisterAuthUserAccountFieldsError,
        login_id::kernel::data::LoginId,
    },
    z_lib::repository::data::RepositoryError,
};

// TODO Extract で受け取って validate は action でやる、だったはず
pub trait UnregisterAuthUserAccountRequestDecoder {
    fn decode(self) -> Result<LoginId, ValidateUnregisterAuthUserAccountFieldsError>;
}

#[async_trait::async_trait]
pub trait UnregisterAuthUserAccountRepository {
    async fn unregister_user(&self, login_id: &LoginId) -> Result<(), RepositoryError>;
}
