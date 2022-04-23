use crate::{
    auth::user::{
        account::unregister::data::ValidateUnregisterAuthUserAccountFieldsError,
        kernel::data::AuthUserId, login_id::kernel::data::LoginId,
    },
    z_lib::repository::data::RepositoryError,
};

// TODO Extract で受け取って validate は action でやる、だったはず
pub trait UnregisterAuthUserAccountRequestDecoder {
    fn decode(self) -> Result<LoginId, ValidateUnregisterAuthUserAccountFieldsError>;
}

#[async_trait::async_trait]
pub trait UnregisterAuthUserAccountRepository {
    async fn lookup_user_id(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<AuthUserId>, RepositoryError>;

    async fn unregister_user(
        &self,
        user_id: &AuthUserId,
        login_id: &LoginId,
    ) -> Result<(), RepositoryError>;
}

#[async_trait::async_trait]
pub trait DiscardAuthTicketRepository {
    async fn discard_all(&self, user_id: &AuthUserId) -> Result<(), RepositoryError>;
}
