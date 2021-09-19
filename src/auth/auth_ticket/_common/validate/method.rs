use crate::auth::{
    auth_ticket::_common::{
        kernel::infra::AuthServiceMetadata,
        validate::infra::{ValidateApiTokenInfra, ValidateService},
    },
    auth_user::_common::kernel::data::{AuthUserId, RequireAuthRoles},
};

use super::data::ValidateApiTokenError;

pub async fn validate_api_token(
    infra: &impl ValidateApiTokenInfra,
    require_roles: RequireAuthRoles,
) -> Result<AuthUserId, ValidateApiTokenError> {
    let service_metadata = infra.service_metadata();
    let validate_service = infra.validate_service();

    let metadata = service_metadata
        .metadata()
        .map_err(ValidateApiTokenError::MetadataError)?;

    let user_id = validate_service
        .validate(metadata, require_roles)
        .await
        .map_err(ValidateApiTokenError::ServiceError)?;

    Ok(user_id)
}
