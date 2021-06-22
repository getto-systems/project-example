use super::super::kernel::method::check_nonce;

use super::infra::{AuthTokenHeader, AuthTokenDecoder, ValidateAuthTokenInfra};

use super::event::ValidateAuthTokenEvent;

use super::super::kernel::data::AuthTicket;
use super::data::ValidateAuthTokenError;

pub fn validate_auth_token<S>(
    infra: &impl ValidateAuthTokenInfra,
    post: impl Fn(ValidateAuthTokenEvent) -> S,
) -> Result<AuthTicket, S> {
    check_nonce(infra.check_nonce_infra())
        .map_err(|err| post(ValidateAuthTokenEvent::NonceError(err)))?;

    let ticket =
        validate_token(infra).map_err(|err| post(ValidateAuthTokenEvent::TokenError(err)))?;

    // 呼び出し側を簡単にするため、例外的に State ではなく AuthTicket を返す
    post(ValidateAuthTokenEvent::Success(ticket.clone()));
    Ok(ticket)
}

fn validate_token(
    infra: &impl ValidateAuthTokenInfra,
) -> Result<AuthTicket, ValidateAuthTokenError> {
    let header = infra.token_header();
    let token_validator = infra.token_validator();
    let config = infra.config();

    let token = header
        .token()
        .map_err(|err| ValidateAuthTokenError::HeaderError(err))?;

    let ticket = token_validator
        .decode(&token)
        .map_err(|err| ValidateAuthTokenError::DecodeError(err))?;

    ticket
        .check_enough_permission(config.require_roles.clone())
        .map_err(|err| ValidateAuthTokenError::PermissionError(err))
}
