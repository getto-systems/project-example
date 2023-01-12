use crate::auth::ticket::authorize::infra::{AuthorizeFieldsExtract, AuthorizeTokenDecoder};

use crate::auth::ticket::{
    authorize::data::ValidateAuthorizeFieldsError,
    kernel::data::{
        AuthPermissionError, AuthPermissionRequired, AuthTicket, AuthorizeToken,
        DecodeAuthorizeTokenError,
    },
};

pub enum AuthorizeWithTokenEvent {
    Success,
    Invalid(ValidateAuthorizeFieldsError),
    DecodeError(DecodeAuthorizeTokenError),
    PermissionError(AuthPermissionError),
}

const SUCCESS: &'static str = "authorize with token success";
const ERROR: &'static str = "authorize with token error";

impl std::fmt::Display for AuthorizeWithTokenEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => write!(f, "{}", SUCCESS),
            Self::Invalid(err) => write!(f, "{}; {}", ERROR, err),
            Self::DecodeError(err) => write!(f, "{}; {}", ERROR, err),
            Self::PermissionError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

pub trait AuthorizeWithTokenInfra {
    type TokenDecoder: AuthorizeTokenDecoder;

    fn token_decoder(&self) -> &Self::TokenDecoder;
}

pub async fn authorize_with_token<S>(
    infra: &impl AuthorizeWithTokenInfra,
    fields: impl AuthorizeFieldsExtract,
    post: impl Fn(AuthorizeWithTokenEvent) -> S,
) -> Result<(AuthTicket, AuthorizeToken, AuthPermissionRequired), S> {
    let fields = fields
        .convert()
        .map_err(|err| post(AuthorizeWithTokenEvent::Invalid(err)))?;

    let ticket = infra
        .token_decoder()
        .decode(fields.token.clone())
        .map_err(|err| post(AuthorizeWithTokenEvent::DecodeError(err)))?;

    ticket
        .attrs
        .granted
        .has_enough_permission(&fields.required)
        .map_err(|err| post(AuthorizeWithTokenEvent::PermissionError(err)))?;

    post(AuthorizeWithTokenEvent::Success);

    Ok((ticket, fields.token, fields.required))
}
