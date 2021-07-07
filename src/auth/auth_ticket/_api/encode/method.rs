use getto_application::data::MethodResult;

use super::super::kernel::infra::{AuthClock, AuthTicketRepository};
use super::infra::{AuthTokenEncoder, EncodeAuthTicketInfra, EncodeMessenger};

use super::event::EncodeAuthTicketEvent;

use super::super::kernel::data::{AuthTicket, ExpansionLimitDateTime};
use super::data::{AuthTokenEncoded, AuthTokenExpires};
use crate::auth::auth_ticket::_api::kernel::infra::AuthTicketInfra;
use crate::z_details::_api::repository::data::RepositoryError;

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
        message: infra
            .messenger()
            .encode(ticket.clone().into_granted_roles())
            .map_err(|err| post(EncodeAuthTicketEvent::MessageError(err)))?,

        ticket_tokens: infra
            .ticket_encoder()
            .encode(ticket.clone(), expires.ticket)
            .map_err(|err| post(EncodeAuthTicketEvent::EncodeError(err)))?,

        api_tokens: infra
            .api_encoder()
            .encode(ticket.clone(), expires.api)
            .map_err(|err| post(EncodeAuthTicketEvent::EncodeError(err)))?,

        cdn_tokens: infra
            .cdn_encoder()
            .encode(ticket.clone(), expires.cdn)
            .map_err(|err| post(EncodeAuthTicketEvent::EncodeError(err)))?,
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
        cdn: clock
            .now()
            .expires_with_limit(&config.cdn_expires, limit.clone()),
    }
}
