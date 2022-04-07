use getto_application::data::MethodResult;

use crate::auth::ticket::{
    encode::infra::{AuthTokenEncoder, CloudfrontTokenEncoder, EncodeAuthTicketRepository},
    kernel::infra::AuthClock,
};

use crate::{
    auth::ticket::{
        encode::data::{AuthTicketEncoded, AuthTokenExpires, EncodeAuthTokenError},
        kernel::data::{AuthTicket, EncodedAuthTokens, ExpansionLimitDateTime, ExpireDuration},
    },
    z_lib::repository::data::RepositoryError,
};

pub enum EncodeAuthTicketEvent {
    TokenExpiresCalculated(AuthTokenExpires),
    Success(AuthTicketEncoded),
    TicketNotFound,
    RepositoryError(RepositoryError),
    EncodeError(EncodeAuthTokenError),
}

const SUCCESS: &'static str = "encode success";
const ERROR: &'static str = "encode error";

impl std::fmt::Display for EncodeAuthTicketEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TokenExpiresCalculated(expires) => {
                write!(f, "token expires calculated; {}", expires)
            }
            Self::Success(_) => write!(f, "{}", SUCCESS),
            Self::TicketNotFound => write!(f, "{}; ticket data not found", ERROR),
            Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
            Self::EncodeError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

pub trait EncodeAuthTicketInfra {
    type Clock: AuthClock;
    type TicketRepository: EncodeAuthTicketRepository;
    type TicketEncoder: AuthTokenEncoder;
    type ApiEncoder: AuthTokenEncoder;
    type CloudfrontEncoder: CloudfrontTokenEncoder;

    fn clock(&self) -> &Self::Clock;
    fn ticket_repository(&self) -> &Self::TicketRepository;
    fn ticket_encoder(&self) -> &Self::TicketEncoder;
    fn api_encoder(&self) -> &Self::ApiEncoder;
    fn cloudfront_encoder(&self) -> &Self::CloudfrontEncoder;
    fn config(&self) -> &EncodeAuthTicketConfig;
}

pub struct EncodeAuthTicketConfig {
    pub ticket_expires: ExpireDuration,
    pub api_expires: ExpireDuration,
    pub cloudfront_expires: ExpireDuration,
}

pub async fn encode_auth_ticket<S>(
    infra: &impl EncodeAuthTicketInfra,
    ticket: AuthTicket,
    post: impl Fn(EncodeAuthTicketEvent) -> S,
) -> MethodResult<S> {
    let ticket_encoder = infra.ticket_encoder();
    let api_encoder = infra.api_encoder();
    let cloudfront_encoder = infra.cloudfront_encoder();
    let ticket_repository = infra.ticket_repository();

    let limit = ticket_repository
        .lookup_expansion_limit(&ticket)
        .await
        .map_err(|err| post(EncodeAuthTicketEvent::RepositoryError(err)))?
        .ok_or_else(|| post(EncodeAuthTicketEvent::TicketNotFound))?;

    let expires = calc_expires(infra, limit);
    post(EncodeAuthTicketEvent::TokenExpiresCalculated(
        expires.clone(),
    ));

    let token = EncodedAuthTokens {
        ticket_token: ticket_encoder
            .encode(ticket.clone(), expires.ticket)
            .map_err(|err| post(EncodeAuthTicketEvent::EncodeError(err)))?,

        api_token: api_encoder
            .encode(ticket.clone(), expires.api)
            .map_err(|err| post(EncodeAuthTicketEvent::EncodeError(err)))?,

        cloudfront_tokens: cloudfront_encoder
            .encode(expires.cloudfront)
            .map_err(|err| post(EncodeAuthTicketEvent::EncodeError(err)))?,
    };

    let response = AuthTicketEncoded {
        roles: ticket.into_user().into_granted_roles(),
        token,
    };
    Ok(post(EncodeAuthTicketEvent::Success(response)))
}
fn calc_expires(
    infra: &impl EncodeAuthTicketInfra,
    limit: ExpansionLimitDateTime,
) -> AuthTokenExpires {
    let clock = infra.clock();
    let config = infra.config();

    AuthTokenExpires {
        ticket: clock
            .now()
            .expires_with_limit(&config.ticket_expires, limit.clone()),
        api: clock
            .now()
            .expires_with_limit(&config.api_expires, limit.clone()),
        cloudfront: clock
            .now()
            .expires_with_limit(&config.cloudfront_expires, limit.clone()),
    }
}
