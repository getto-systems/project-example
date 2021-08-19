use crate::auth::{
    auth_ticket::_api::{
        kernel::infra::{AuthNonceHeader, AuthTokenHeader},
        validate::infra::{ValidateApiTokenInfra, ValidateService},
    },
    auth_user::_common::kernel::data::{AuthUserId, RequireAuthRoles},
};

use super::data::ValidateApiTokenError;

pub async fn validate_api_token(
    infra: &impl ValidateApiTokenInfra,
    require_roles: RequireAuthRoles,
) -> Result<AuthUserId, ValidateApiTokenError> {
    let nonce_header = infra.nonce_header();
    let token_header = infra.token_header();
    let validate_service = infra.validate_service();

    let nonce = nonce_header
        .nonce()
        .map_err(ValidateApiTokenError::HeaderError)?;

    let token = token_header
        .token()
        .map_err(ValidateApiTokenError::HeaderError)?;

    let user_id = validate_service
        .validate(nonce, token, require_roles)
        .await
        .map_err(ValidateApiTokenError::ServiceError)?;

    Ok(user_id)
}
