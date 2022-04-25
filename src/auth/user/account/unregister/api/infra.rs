use crate::{
    auth::user::{
        account::unregister::data::ValidateUnregisterAuthUserAccountFieldsError,
        kernel::data::AuthUserId, login_id::kernel::data::LoginId,
    },
    z_lib::repository::data::RepositoryError,
};

pub trait UnregisterAuthUserAccountRequestDecoder {
    fn decode(self) -> UnregisterAuthUserAccountFieldsExtract;
}

pub struct UnregisterAuthUserAccountFields {
    pub login_id: LoginId,
}

pub struct UnregisterAuthUserAccountFieldsExtract {
    pub login_id: String,
}

impl UnregisterAuthUserAccountFields {
    pub fn convert(
        fields: UnregisterAuthUserAccountFieldsExtract,
    ) -> Result<Self, ValidateUnregisterAuthUserAccountFieldsError> {
        Ok(Self {
            login_id: LoginId::convert(fields.login_id)
                .map_err(ValidateUnregisterAuthUserAccountFieldsError::InvalidLoginId)?,
        })
    }
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
