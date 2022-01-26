use crate::auth::ticket::remote::validate_nonce::method::{
    validate_auth_nonce, ValidateAuthNonceEvent, ValidateAuthNonceInfra,
};

use crate::auth::ticket::remote::kernel::infra::{AuthTokenDecoder, AuthTokenMetadata};

use crate::auth::{
    ticket::remote::{
        kernel::data::{AuthTicket, ValidateAuthRolesError},
        validate::data::ValidateAuthTokenError,
    },
    user::remote::kernel::data::RequireAuthRoles,
};

pub enum ValidateAuthTokenEvent {
    ValidateNonce(ValidateAuthNonceEvent),
    Success(AuthTicket),
    TokenError(ValidateAuthTokenError),
    PermissionError(ValidateAuthRolesError),
}

const SUCCESS: &'static str = "validate success";
const ERROR: &'static str = "validate error";

impl std::fmt::Display for ValidateAuthTokenEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ValidateNonce(event) => event.fmt(f),
            Self::Success(ticket) => write!(f, "{}; {}", SUCCESS, ticket),
            Self::TokenError(err) => write!(f, "{}; {}", ERROR, err),
            Self::PermissionError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

pub trait ValidateAuthTokenInfra {
    type ValidateNonce: ValidateAuthNonceInfra;
    type TokenMetadata: AuthTokenMetadata;
    type TokenDecoder: AuthTokenDecoder;

    fn validate_nonce(&self) -> &Self::ValidateNonce;
    fn token_metadata(&self) -> &Self::TokenMetadata;
    fn token_decoder(&self) -> &Self::TokenDecoder;
}

pub async fn validate_auth_token<S>(
    infra: &impl ValidateAuthTokenInfra,
    require_roles: RequireAuthRoles,
    post: impl Fn(ValidateAuthTokenEvent) -> S,
) -> Result<AuthTicket, S> {
    validate_auth_nonce(infra.validate_nonce(), |event| {
        post(ValidateAuthTokenEvent::ValidateNonce(event))
    })
    .await?;

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
        .map(|ticket| ticket.restore())
        .map_err(ValidateAuthTokenError::DecodeError)
}
