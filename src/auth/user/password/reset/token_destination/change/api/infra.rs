use crate::{
    auth::user::{
        login_id::kernel::data::LoginId,
        password::reset::{
            kernel::data::ResetTokenDestination,
            token_destination::change::data::ValidateChangeResetTokenDestinationFieldsError,
        },
    },
    z_lib::repository::data::RepositoryError,
};

// TODO Extract で受け取って validate は action でやる、だったはず
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
    ) -> Result<Option<ResetTokenDestination>, RepositoryError>;

    async fn change_destination(
        &self,
        login_id: LoginId,
        data: ResetTokenDestination,
    ) -> Result<(), RepositoryError>;
}