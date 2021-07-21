use crate::auth::auth_ticket::_auth::kernel::method::check_nonce;

use crate::auth::auth_ticket::_auth::validate::infra::{
    AuthTokenDecoder, AuthTokenMetadata, ValidateAuthTokenConfig, ValidateAuthTokenInfra,
};

use super::event::ValidateAuthTokenEvent;

use crate::auth::auth_ticket::_auth::{
    kernel::data::AuthTicket, validate::data::ValidateAuthTokenError,
};

pub async fn validate_auth_token<S>(
    infra: impl ValidateAuthTokenInfra,
    post: impl Fn(ValidateAuthTokenEvent) -> S,
) -> Result<AuthTicket, S> {
    let (check_nonce_infra, token_metadata, token_decoder, config) = infra.extract();

    check_nonce(check_nonce_infra)
        .await
        .map_err(|err| post(ValidateAuthTokenEvent::NonceError(err)))?;

    let ticket = validate_token(token_metadata, token_decoder, config)
        .map_err(|err| post(ValidateAuthTokenEvent::TokenError(err)))?;

    // 呼び出し側を簡単にするため、例外的に State ではなく AuthTicket を返す
    post(ValidateAuthTokenEvent::Success(ticket.clone()));
    Ok(ticket)
}

fn validate_token(
    token_metadata: impl AuthTokenMetadata,
    token_decoder: impl AuthTokenDecoder,
    config: ValidateAuthTokenConfig,
) -> Result<AuthTicket, ValidateAuthTokenError> {
    let token = token_metadata
        .token()
        .map_err(ValidateAuthTokenError::MetadataError)?;

    let ticket = token_decoder
        .decode(&token)
        .map_err(ValidateAuthTokenError::DecodeError)?;

    ticket
        .check_enough_permission(config.require_roles.clone())
        .map_err(ValidateAuthTokenError::PermissionError)
}
