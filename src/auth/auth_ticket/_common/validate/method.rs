use crate::auth::{
    auth_ticket::_common::{
        kernel::infra::{AuthNonceMetadata, AuthTokenMetadata},
        validate::infra::{ValidateApiTokenInfra, ValidateService},
    },
    auth_user::_common::kernel::data::{AuthUserId, RequireAuthRoles},
};

use super::data::ValidateApiTokenError;

pub async fn validate_api_token(
    infra: &impl ValidateApiTokenInfra,
    require_roles: RequireAuthRoles,
) -> Result<AuthUserId, ValidateApiTokenError> {
    let nonce_metadata = infra.nonce_metadata();
    let token_metadata = infra.token_metadata();
    let validate_service = infra.validate_service();

    let nonce = nonce_metadata
        .nonce()
        .map_err(ValidateApiTokenError::MetadataError)?;

    let token = token_metadata
        .token()
        .map_err(ValidateApiTokenError::MetadataError)?;

    let user_id = validate_service
        .validate(nonce, token, require_roles)
        .await
        .map_err(ValidateApiTokenError::ServiceError)?;

    Ok(user_id)
}
