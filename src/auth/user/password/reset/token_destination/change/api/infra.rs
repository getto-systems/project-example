use crate::{
    auth::user::{
        login_id::kernel::data::LoginId,
        password::reset::{
            kernel::data::{ResetTokenDestination, ResetTokenDestinationExtract},
            token_destination::change::data::{
                ValidateChangeResetTokenDestinationChangesError,
                ValidateChangeResetTokenDestinationFieldsError,
            },
        },
    },
    z_lib::repository::data::RepositoryError,
};

pub trait ChangeResetTokenDestinationRequestDecoder {
    fn decode(self) -> ChangeResetTokenDestinationFieldsExtract;
}

pub struct ChangeResetTokenDestinationFields {
    pub login_id: LoginId,
    pub from: ResetTokenDestination,
    pub to: ResetTokenDestination,
}

pub struct ChangeResetTokenDestinationFieldsExtract {
    pub login_id: String,
    pub from: Option<ResetTokenDestinationExtract>,
    pub to: Option<ResetTokenDestinationExtract>,
}

impl ChangeResetTokenDestinationFields {
    pub fn convert(
        fields: ChangeResetTokenDestinationFieldsExtract,
    ) -> Result<Self, ValidateChangeResetTokenDestinationFieldsError> {
        Ok(Self {
            login_id: LoginId::convert(fields.login_id)
                .map_err(ValidateChangeResetTokenDestinationFieldsError::InvalidLoginId)?,
            from: convert_changes(fields.from)
                .map_err(ValidateChangeResetTokenDestinationFieldsError::InvalidFrom)?,
            to: convert_changes(fields.to)
                .map_err(ValidateChangeResetTokenDestinationFieldsError::InvalidTo)?,
        })
    }
}

fn convert_changes(
    destination: Option<ResetTokenDestinationExtract>,
) -> Result<ResetTokenDestination, ValidateChangeResetTokenDestinationChangesError> {
    match destination {
        None => Err(ValidateChangeResetTokenDestinationChangesError::NotFound),
        Some(destination) => ResetTokenDestination::convert(destination)
            .map_err(ValidateChangeResetTokenDestinationChangesError::InvalidResetTokenDestination),
    }
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
