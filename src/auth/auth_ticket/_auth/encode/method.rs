use getto_application::data::MethodResult;

use crate::auth::auth_ticket::_auth::{
    encode::infra::{
        AuthTokenEncoder, CloudfrontTokenEncoder, EncodeAuthTicketConfig, EncodeAuthTicketInfra,
    },
    kernel::infra::{AuthClock, AuthTicketInfra, AuthTicketRepository},
};

use super::event::EncodeAuthTicketEvent;

use crate::auth::auth_ticket::{
    _auth::{encode::data::AuthTokenExpires, kernel::data::AuthTicket},
    _common::{
        encode::data::EncodeAuthTicketResponse,
        kernel::data::{AuthTokenEncoded, ExpansionLimitDateTime},
    },
};

pub async fn encode_auth_ticket<S>(
    infra: impl EncodeAuthTicketInfra,
    ticket: AuthTicket,
    post: impl Fn(EncodeAuthTicketEvent) -> S,
) -> MethodResult<S> {
    let (ticket_infra, ticket_encoder, api_encoder, cloudfront_encoder, config) = infra.extract();
    let (clock, ticket_repository) = ticket_infra.extract();

    let limit = ticket_repository
        .expansion_limit(&ticket)
        .await
        .map_err(|err| post(EncodeAuthTicketEvent::RepositoryError(err)))?
        .ok_or_else(|| post(EncodeAuthTicketEvent::TicketNotFound))?;

    let expires = calc_expires(&clock, &config, limit);
    post(EncodeAuthTicketEvent::TokenExpiresCalculated(
        expires.clone(),
    ));

    let token = AuthTokenEncoded {
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

    let response = EncodeAuthTicketResponse::new(ticket.into_user(), token);
    Ok(post(EncodeAuthTicketEvent::Success(response)))
}
fn calc_expires(
    clock: &impl AuthClock,
    config: &EncodeAuthTicketConfig,
    limit: ExpansionLimitDateTime,
) -> AuthTokenExpires {
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
