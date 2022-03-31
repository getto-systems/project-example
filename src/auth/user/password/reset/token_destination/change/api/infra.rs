use crate::{
    auth::user::{
        kernel::data::AuthUserId,
        login_id::kernel::data::LoginId,
        password::reset::{
            kernel::data::ResetTokenDestination,
            token_destination::change::data::ValidateChangeResetTokenDestinationFieldsError,
        },
    },
    z_lib::repository::data::RepositoryError,
};

pub trait ChangeResetTokenDestinationRequestDecoder {
    fn decode(
        self,
    ) -> Result<ChangeResetTokenDestinationFields, ValidateChangeResetTokenDestinationFieldsError>;
}

pub struct ChangeResetTokenDestinationFields {
    pub login_id: LoginId,
    pub from: ResetTokenDestination,
    pub to: ResetTokenDestination,
}

#[async_trait::async_trait]
pub trait ChangeResetTokenDestinationRepository {
    async fn lookup_destination(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<(AuthUserId, ResetTokenDestination)>, RepositoryError>;

    async fn change_destination(
        &self,
        login_id: &LoginId,
        data: ResetTokenDestination,
    ) -> Result<(), RepositoryError>;

    async fn get_updated_destination(
        &self,
        login_id: &LoginId,
    ) -> Result<ResetTokenDestination, RepositoryError>;
}
