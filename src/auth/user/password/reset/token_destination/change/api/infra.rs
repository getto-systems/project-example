use crate::{
    auth::user::{
        login_id::kernel::data::LoginId,
        password::reset::{
            kernel::data::ResetPasswordTokenDestination,
            token_destination::change::data::{
                ChangeResetTokenDestinationError, ChangeResetTokenDestinationSuccess,
                ValidateChangeResetTokenDestinationFieldsError,
            },
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

impl ChangeResetTokenDestinationFieldsExtract for ChangeResetTokenDestinationFields {
    fn convert(
        self,
    ) -> Result<ChangeResetTokenDestinationFields, ValidateChangeResetTokenDestinationFieldsError>
    {
        Ok(self)
    }
}

pub trait ChangeResetTokenDestinationInfra {
    type Repository: ChangeResetTokenDestinationRepository;

    fn repository(&self) -> &Self::Repository;
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

pub trait ChangeResetTokenDestinationLogger: Send + Sync {
    fn try_to_change_destination(&self);
    fn invalid_request(
        &self,
        err: ValidateChangeResetTokenDestinationFieldsError,
    ) -> ValidateChangeResetTokenDestinationFieldsError;
    fn failed_to_lookup_destination(&self, err: RepositoryError) -> RepositoryError;
    fn user_not_found(
        &self,
        err: ChangeResetTokenDestinationError,
    ) -> ChangeResetTokenDestinationError;
    fn conflict(&self, err: ChangeResetTokenDestinationError) -> ChangeResetTokenDestinationError;
    fn failed_to_change_destination(&self, err: RepositoryError) -> RepositoryError;
    fn succeed_to_change_destination(
        &self,
        auth: ChangeResetTokenDestinationSuccess,
    ) -> ChangeResetTokenDestinationSuccess;
}
