use getto_application::data::MethodResult;

use crate::auth::{
    kernel::infra::AuthClock,
    ticket::encode::infra::{
        AuthenticateTokenEncoder, AuthorizeTokenEncoder, CdnTokenEncoder,
        EncodeAuthTicketRepository, EncodeAuthTokenConfig,
    },
};

use crate::{
    auth::ticket::{
        encode::data::{AuthTokenExpires, EncodeAuthTokenError},
        kernel::data::{AuthPermissionGranted, AuthTicket, AuthToken},
    },
    common::api::repository::data::RepositoryError,
};

pub enum EncodeAuthTokenEvent {
    TokenExpiresCalculated(AuthTokenExpires),
    Success(AuthToken, AuthPermissionGranted),
    TicketNotFound,
    RepositoryError(RepositoryError),
    EncodeError(EncodeAuthTokenError),
}

const SUCCESS: &'static str = "encode auth-token success";
const ERROR: &'static str = "encode auth-token error";

impl std::fmt::Display for EncodeAuthTokenEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TokenExpiresCalculated(expires) => {
                write!(f, "token expires calculated; {}", expires)
            }
            Self::Success(_, _) => write!(f, "{}", SUCCESS),
            Self::TicketNotFound => write!(f, "{}; ticket data not found", ERROR),
            Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
            Self::EncodeError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

pub trait EncodeAuthTokenInfra {
    type Clock: AuthClock;
    type TicketRepository: EncodeAuthTicketRepository;
    type AuthenticateEncoder: AuthenticateTokenEncoder;
    type AuthorizeEncoder: AuthorizeTokenEncoder;
    type CdnEncoder: CdnTokenEncoder;

    fn clock(&self) -> &Self::Clock;
    fn ticket_repository(&self) -> &Self::TicketRepository;
    fn authenticate_encoder(&self) -> &Self::AuthenticateEncoder;
    fn authorize_encoder(&self) -> &Self::AuthorizeEncoder;
    fn cdn_encoder(&self) -> &Self::CdnEncoder;
    fn config(&self) -> &EncodeAuthTokenConfig;
}

pub async fn encode_auth_token<S>(
    infra: &impl EncodeAuthTokenInfra,
    ticket: AuthTicket,
    post: impl Fn(EncodeAuthTokenEvent) -> S,
) -> MethodResult<S> {
    let limit = infra
        .ticket_repository()
        .lookup_expansion_limit(&ticket)
        .await
        .map_err(|err| post(EncodeAuthTokenEvent::RepositoryError(err)))?
        .ok_or_else(|| post(EncodeAuthTokenEvent::TicketNotFound))?;

    let expires = AuthTokenExpires {
        authenticate: infra
            .clock()
            .now()
            .expires_with_limit(&infra.config().authenticate_expires, &limit),

        authorize: infra
            .clock()
            .now()
            .expires_with_limit(&infra.config().authorize_expires, &limit),

        cdn: infra
            .clock()
            .now()
            .expires_with_limit(&infra.config().cdn_expires, &limit),
    };

    post(EncodeAuthTokenEvent::TokenExpiresCalculated(
        expires.clone(),
    ));

    let token = AuthToken {
        authenticate_token: infra
            .authenticate_encoder()
            .encode(ticket.clone(), expires.authenticate)
            .map_err(|err| post(EncodeAuthTokenEvent::EncodeError(err)))?,

        authorize_token: infra
            .authorize_encoder()
            .encode(ticket.clone(), expires.authorize)
            .map_err(|err| post(EncodeAuthTokenEvent::EncodeError(err)))?,

        cdn_token: infra
            .cdn_encoder()
            .encode(expires.cdn)
            .map_err(|err| post(EncodeAuthTokenEvent::EncodeError(err)))?,
    };

    Ok(post(EncodeAuthTokenEvent::Success(
        token,
        ticket.attrs.granted,
    )))
}
