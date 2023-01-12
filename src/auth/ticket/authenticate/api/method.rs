use crate::auth::ticket::authenticate::infra::AuthenticateTokenDecoder;

use crate::auth::ticket::kernel::data::{
    AuthTicket, AuthenticateToken, AuthenticateTokenExtract, DecodeAuthenticateTokenError,
    ValidateAuthenticateTokenError,
};

pub trait AuthenticateWithTokenInfra {
    type TokenDecoder: AuthenticateTokenDecoder;

    fn token_decoder(&self) -> &Self::TokenDecoder;
}

pub enum AuthenticateWithTokenEvent {
    Success(AuthTicket),
    Invalid(ValidateAuthenticateTokenError),
    DecodeError(DecodeAuthenticateTokenError),
}

const SUCCESS: &'static str = "authenticate with token success";
const ERROR: &'static str = "authenticate with token error";

impl std::fmt::Display for AuthenticateWithTokenEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success(ticket) => write!(f, "{}; {}", SUCCESS, ticket),
            Self::Invalid(err) => write!(f, "{}; {}", ERROR, err),
            Self::DecodeError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

pub async fn authenticate_with_token<S>(
    infra: &impl AuthenticateWithTokenInfra,
    token: impl AuthenticateTokenExtract,
    post: impl Fn(AuthenticateWithTokenEvent) -> S,
) -> Result<(AuthTicket, AuthenticateToken), S> {
    let token = token
        .convert()
        .map_err(|err| post(AuthenticateWithTokenEvent::Invalid(err)))?;

    let ticket = infra
        .token_decoder()
        .decode(token.clone())
        .map_err(|err| post(AuthenticateWithTokenEvent::DecodeError(err)))?;

    post(AuthenticateWithTokenEvent::Success(ticket.clone()));

    Ok((ticket, token))
}
