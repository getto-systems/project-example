use getto_application::data::MethodResult;

use super::super::kernel::infra::{AuthClock, AuthTicketRepository, AuthTicketTokens};
use super::infra::{AuthTokenEncoder, EncodeAuthTicketInfra, EncodeMessenger};

use super::event::EncodeAuthTicketEvent;

use super::super::kernel::data::{AuthTicket, AuthToken, ExpansionLimitDateTime};
use super::data::{AuthTicketEncoded, AuthTokenEncoded, EncodeAuthTokenError};
use crate::z_details::_api::{message::data::MessageError, repository::data::RepositoryError};

pub fn encode_auth_ticket<S>(
    infra: &impl EncodeAuthTicketInfra,
    ticket: AuthTicket,
    post: impl Fn(EncodeAuthTicketEvent) -> S,
) -> MethodResult<S> {
    let limit = fetch_expansion_limit(infra, &ticket)
        .map_err(|err| post(EncodeAuthTicketEvent::RepositoryError(err)))?;

    let limit = limit.ok_or_else(|| post(EncodeAuthTicketEvent::TicketNotFound))?;

    let encoded = AuthTicketEncoded {
        message: encode_message(infra, ticket.clone())
            .map_err(|err| post(EncodeAuthTicketEvent::MessageError(err)))?,

        ticket_tokens: encode_ticket_token(infra, ticket.clone(), limit.clone())
            .map_err(|err| post(EncodeAuthTicketEvent::EncodeError(err)))?,

        api_tokens: encode_api_token(infra, ticket.clone(), limit.clone())
            .map_err(|err| post(EncodeAuthTicketEvent::EncodeError(err)))?,

        cdn_tokens: encode_cdn_token(infra, ticket.clone(), limit.clone())
            .map_err(|err| post(EncodeAuthTicketEvent::EncodeError(err)))?,
    };

    register_ticket_tokens(infra, ticket.clone(), encoded.clone())
        .map_err(|err| post(EncodeAuthTicketEvent::RepositoryError(err)))?;

    Ok(post(EncodeAuthTicketEvent::Success(encoded)))
}
fn fetch_expansion_limit(
    infra: &impl EncodeAuthTicketInfra,
    ticket: &AuthTicket,
) -> Result<Option<ExpansionLimitDateTime>, RepositoryError> {
    let ticket_repository = infra.ticket_repository();
    ticket_repository.expansion_limit(&ticket)
}
fn register_ticket_tokens(
    infra: &impl EncodeAuthTicketInfra,
    ticket: AuthTicket,
    encoded: AuthTicketEncoded,
) -> Result<(), RepositoryError> {
    let ticket_repository = infra.ticket_repository();

    let mut tokens = vec![];
    encoded
        .ticket_tokens
        .into_iter()
        .for_each(|token| tokens.push(AuthToken::new(token.token)));
    encoded
        .api_tokens
        .into_iter()
        .for_each(|token| tokens.push(AuthToken::new(token.token)));

    ticket_repository.register_tokens(ticket, AuthTicketTokens::new(tokens))
}
fn encode_message(
    infra: &impl EncodeAuthTicketInfra,
    ticket: AuthTicket,
) -> Result<String, MessageError> {
    infra.messenger().encode(ticket.into_granted_roles())
}
fn encode_ticket_token(
    infra: &impl EncodeAuthTicketInfra,
    ticket: AuthTicket,
    limit: ExpansionLimitDateTime,
) -> Result<Vec<AuthTokenEncoded>, EncodeAuthTokenError> {
    let config = infra.config();
    let clock = infra.clock();
    let ticket_encoder = infra.ticket_encoder();

    let expires = clock
        .now()
        .expires_with_limit(&config.ticket_expires, limit);

    ticket_encoder.encode(ticket.clone(), expires)
}
fn encode_api_token(
    infra: &impl EncodeAuthTicketInfra,
    ticket: AuthTicket,
    limit: ExpansionLimitDateTime,
) -> Result<Vec<AuthTokenEncoded>, EncodeAuthTokenError> {
    let config = infra.config();
    let clock = infra.clock();
    let api_encoder = infra.api_encoder();

    let expires = clock.now().expires_with_limit(&config.api_expires, limit);

    api_encoder.encode(ticket.clone(), expires)
}
fn encode_cdn_token(
    infra: &impl EncodeAuthTicketInfra,
    ticket: AuthTicket,
    limit: ExpansionLimitDateTime,
) -> Result<Vec<AuthTokenEncoded>, EncodeAuthTokenError> {
    let config = infra.config();
    let clock = infra.clock();
    let cdn_encoder = infra.cdn_encoder();

    let expires = clock.now().expires_with_limit(&config.cdn_expires, limit);

    cdn_encoder.encode(ticket.clone(), expires)
}
