use crate::auth::auth_ticket::_auth::kernel::method::check_nonce;

use crate::auth::auth_ticket::_auth::validate::infra::{
    AuthTokenDecoder, AuthTokenMetadata, ValidateAuthTokenInfra,
};

use super::event::ValidateAuthTokenEvent;

use crate::auth::{
    auth_ticket::_auth::{kernel::data::AuthTicket, validate::data::ValidateAuthTokenError},
    auth_user::_common::kernel::data::RequireAuthRoles,
};

pub async fn validate_auth_token<S>(
    infra: &impl ValidateAuthTokenInfra,
    require_roles: RequireAuthRoles,
    post: impl Fn(ValidateAuthTokenEvent) -> S,
) -> Result<AuthTicket, S> {
    check_nonce(infra.check_nonce_infra())
        .await
        .map_err(|err| post(ValidateAuthTokenEvent::NonceError(err)))?;

    let ticket =
        decode_ticket(infra).map_err(|err| post(ValidateAuthTokenEvent::TokenError(err)))?;

    let ticket = ticket
        .check_enough_permission(require_roles)
        .map_err(|err| post(ValidateAuthTokenEvent::PermissionError(err)))?;

    // 呼び出し側を簡単にするため、例外的に State ではなく AuthTicket を返す
    post(ValidateAuthTokenEvent::Success(ticket.clone()));
    Ok(ticket)
}

fn decode_ticket(
    infra: &impl ValidateAuthTokenInfra,
) -> Result<AuthTicket, ValidateAuthTokenError> {
    let token_metadata = infra.token_metadata();
    let token_decoder = infra.token_decoder();

    let token = token_metadata
        .token()
        .map_err(ValidateAuthTokenError::MetadataError)?
        .ok_or(ValidateAuthTokenError::TokenNotSent)?;

    token_decoder
        .decode(&token)
        .map_err(ValidateAuthTokenError::DecodeError)
}