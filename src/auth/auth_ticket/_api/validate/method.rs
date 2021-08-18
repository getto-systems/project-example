use crate::auth::{
    auth_ticket::_api::{
        kernel::infra::{AuthNonceHeader, AuthTokenHeader},
        validate::infra::{ValidateApiTokenInfra, ValidateService},
    },
    auth_user::_common::kernel::data::{AuthUserId, RequireAuthRoles},
};

use super::event::ValidateApiTokenEvent;

pub async fn validate_api_token<S>(
    infra: &impl ValidateApiTokenInfra,
    require_roles: RequireAuthRoles,
    post: impl Fn(ValidateApiTokenEvent) -> S,
) -> Result<AuthUserId, S> {
    let nonce_header = infra.nonce_header();
    let token_header = infra.token_header();
    let validate_service = infra.validate_service();

    let nonce = nonce_header
        .nonce()
        .map_err(|err| post(ValidateApiTokenEvent::HeaderError(err)))?;

    let token = token_header
        .token()
        .map_err(|err| post(ValidateApiTokenEvent::HeaderError(err)))?;

    let user_id = validate_service
        .validate(nonce, token, require_roles)
        .await
        .map_err(|err| post(ValidateApiTokenEvent::ServiceError(err)))?;

    post(ValidateApiTokenEvent::Success);
    Ok(user_id)
}
