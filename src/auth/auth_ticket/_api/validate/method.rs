use super::super::kernel::part::check_nonce;

use super::super::kernel::infra::AuthTicketRepository;
use super::infra::{AuthTokenHeader, AuthTokenValidator, ValidateAuthTokenInfra};

use super::event::ValidateAuthTokenEvent;

use super::super::kernel::data::{AuthTicket, AuthTokenValue, ValidateAuthRolesError};
use super::data::{DecodeAuthTokenError, ValidateAuthTokenError};
use crate::z_details::_api::request::data::HeaderError;

pub fn validate_auth_token<S>(
    infra: &impl ValidateAuthTokenInfra,
    post: impl Fn(ValidateAuthTokenEvent) -> S,
) -> Result<AuthTicket, S> {
    check_nonce(
        infra.nonce_config(),
        infra.clock(),
        infra.nonce_header(),
        infra.nonce_repository(),
    )
    .map_err(|err| post(ValidateAuthTokenEvent::NonceError(err)))?;

    match validate_token(infra) {
        Err(err) => Err(match err {
            ValidateError::Header(err) => post(ValidateAuthTokenEvent::TokenError(
                ValidateAuthTokenError::HeaderError(err),
            )),
            ValidateError::Permission(err) => post(ValidateAuthTokenEvent::TokenError(
                ValidateAuthTokenError::PermissionError(err),
            )),
            ValidateError::Decode(token, err) => {
                post(ValidateAuthTokenEvent::TokenError(
                    ValidateAuthTokenError::DecodeError(err),
                ));
                disable_token(infra, &token).map_err(|err| post(ValidateAuthTokenEvent::TokenError(err)))?;
                post(ValidateAuthTokenEvent::TicketDisabled)
            }
        }),
        Ok(ticket) => {
            // 呼び出し側を簡単にするため、例外的に State ではなく AuthTicket を返す
            post(ValidateAuthTokenEvent::Success(ticket.clone()));
            Ok(ticket)
        }
    }
}

enum ValidateError {
    Header(HeaderError),
    Permission(ValidateAuthRolesError),
    Decode(AuthTokenValue, DecodeAuthTokenError),
}
fn validate_token(infra: &impl ValidateAuthTokenInfra) -> Result<AuthTicket, ValidateError> {
    let header = infra.token_header();
    let token_validator = infra.token_validator();
    let config = infra.config();

    let token = header.token().map_err(|err| ValidateError::Header(err))?;

    let ticket = token_validator
        .validate(&token)
        .map_err(|err| ValidateError::Decode(token, err))?;

    ticket
        .check_enough_permission(config.require_roles.clone())
        .map_err(|err| ValidateError::Permission(err))
}

fn disable_token(
    infra: &impl ValidateAuthTokenInfra,
    token: &AuthTokenValue,
) -> Result<(), ValidateAuthTokenError> {
    let ticket_repository = infra.ticket_repository();

    ticket_repository
        .disable(token)
        .map_err(ValidateAuthTokenError::RepositoryError)
}
