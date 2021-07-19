use getto_application::data::MethodResult;

use crate::auth::auth_ticket::_auth::{
    encode::infra::{AuthTokenEncoder, EncodeAuthTicketInfra},
    kernel::infra::{AuthClock, AuthTicketInfra, AuthTicketRepository},
};

use super::event::EncodeAuthTicketEvent;

use crate::auth::auth_ticket::_auth::{
    encode::data::{AuthTokenEncoded, AuthTokenExpires},
    kernel::data::{AuthTicket, ExpansionLimitDateTime},
};
use crate::z_details::_common::repository::data::RepositoryError;

pub async fn encode_auth_ticket<S>(
    infra: &impl EncodeAuthTicketInfra,
    ticket: AuthTicket,
    post: impl Fn(EncodeAuthTicketEvent) -> S,
) -> MethodResult<S> {
    let limit = fetch_expansion_limit(infra, &ticket)
        .await
        .map_err(|err| post(EncodeAuthTicketEvent::RepositoryError(err)))?
        .ok_or_else(|| post(EncodeAuthTicketEvent::TicketNotFound))?;

    let expires = calc_expires(infra, limit);
    post(EncodeAuthTicketEvent::TokenExpiresCalculated(
        expires.clone(),
    ));

    let encoded = AuthTokenEncoded {
        ticket_tokens: infra
            .ticket_encoder()
            .encode(ticket.clone(), expires.ticket)
            .map_err(|err| post(EncodeAuthTicketEvent::EncodeError(err)))?,

        api_tokens: infra
            .api_encoder()
            .encode(ticket.clone(), expires.api)
            .map_err(|err| post(EncodeAuthTicketEvent::EncodeError(err)))?,

        cloudfront_tokens: infra
            .cloudfront_encoder()
            .encode(ticket.clone(), expires.cloudfront)
            .map_err(|err| post(EncodeAuthTicketEvent::EncodeError(err)))?,

        granted_roles: ticket.into_granted_roles(),
    };

    Ok(post(EncodeAuthTicketEvent::Success(encoded)))
}
async fn fetch_expansion_limit(
    infra: &impl EncodeAuthTicketInfra,
    ticket: &AuthTicket,
) -> Result<Option<ExpansionLimitDateTime>, RepositoryError> {
    let ticket_infra = infra.ticket_infra();
    let ticket_repository = ticket_infra.ticket_repository();
    ticket_repository.expansion_limit(&ticket).await
}
fn calc_expires(
    infra: &impl EncodeAuthTicketInfra,
    limit: ExpansionLimitDateTime,
) -> AuthTokenExpires {
    let ticket_infra = infra.ticket_infra();
    let clock = ticket_infra.clock();
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
