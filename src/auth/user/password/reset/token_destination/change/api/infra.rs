use crate::{
    auth::user::{
        login_id::kernel::data::LoginId,
        password::reset::{
            kernel::data::ResetPasswordTokenDestination,
            token_destination::change::data::ValidateChangeResetTokenDestinationFieldsError,
        },
    },
    common::api::repository::data::RepositoryError,
};

pub struct ChangeResetTokenDestinationFields {
    pub login_id: LoginId,
    pub from: ResetPasswordTokenDestination,
    pub to: ResetPasswordTokenDestination,
}

pub trait ChangeResetTokenDestinationFieldsExtract {
    fn convert(
        self,
    ) -> Result<ChangeResetTokenDestinationFields, ValidateChangeResetTokenDestinationFieldsError>;
}

#[async_trait::async_trait]
pub trait ChangeResetTokenDestinationRepository {
    async fn lookup_destination(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<ResetPasswordTokenDestination>, RepositoryError>;

    async fn change_destination(
        &self,
        login_id: LoginId,
        data: ResetPasswordTokenDestination,
    ) -> Result<(), RepositoryError>;
}
