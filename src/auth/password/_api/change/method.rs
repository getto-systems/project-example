use getto_application::data::MethodResult;

use crate::auth::{
    auth_ticket::_api::{
        kernel::infra::{AuthNonceHeader, AuthTokenHeader},
        validate::method::validate_api_token,
    },
    auth_user::_common::kernel::data::RequireAuthRoles,
    password::{
        _api::change::infra::{
            ChangePasswordInfra, ChangePasswordResponseEncoder, ChangePasswordService,
        },
        _common::change::infra::ChangePasswordFieldsExtract,
    },
};

use super::event::ChangePasswordEvent;

pub async fn change_password<S>(
    infra: &impl ChangePasswordInfra,
    fields: ChangePasswordFieldsExtract,
    post: impl Fn(ChangePasswordEvent) -> S,
) -> MethodResult<S> {
    let nonce_header = infra.nonce_header();
    let token_header = infra.token_header();
    let validate_infra = infra.validate_infra();
    let change_service = infra.change_service();
    let response_encoder = infra.response_encoder();

    let require_roles = RequireAuthRoles::Nothing;

    let nonce = nonce_header
        .nonce()
        .map_err(|err| post(ChangePasswordEvent::HeaderError(err)))?;

    let token = token_header
        .token()
        .map_err(|err| post(ChangePasswordEvent::HeaderError(err)))?;

    let user_id = validate_api_token(validate_infra, require_roles)
        .await
        .map_err(|err| post(ChangePasswordEvent::ValidateError(err)))?;

    let response = change_service
        .change(nonce, token, user_id, fields)
        .await
        .map_err(|err| post(ChangePasswordEvent::ServiceError(err)))?;

    let message = response_encoder
        .encode(response)
        .map_err(|err| post(ChangePasswordEvent::MessageError(err)))?;

    Ok(post(ChangePasswordEvent::Result(message)))
}
