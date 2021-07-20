use getto_application::data::MethodResult;

use crate::auth::auth_ticket::_auth::{
    encode::infra::{AuthTokenEncoder, CloudfrontTokenEncoder, EncodeAuthTicketInfra},
    kernel::infra::{AuthClock, AuthTicketInfra, AuthTicketRepository},
};

use super::event::EncodeAuthTicketEvent;

use crate::{
    auth::auth_ticket::{
        _auth::{
            encode::data::AuthTokenExpires,
            kernel::data::{AuthTicket, ExpansionLimitDateTime},
        },
        _common::{encode::data::EncodeAuthTicketResponse, kernel::data::AuthTokenEncoded},
    },
    z_details::_common::repository::data::RepositoryError,
};

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

    let token = AuthTokenEncoded {
        ticket_token: infra
            .ticket_encoder()
            .encode(ticket.clone(), expires.ticket)
            .map_err(|err| post(EncodeAuthTicketEvent::EncodeError(err)))?,

        api_token: infra
            .api_encoder()
            .encode(ticket.clone(), expires.api)
            .map_err(|err| post(EncodeAuthTicketEvent::EncodeError(err)))?,

        cloudfront_tokens: infra
            .cloudfront_encoder()
            .encode(expires.cloudfront)
            .map_err(|err| post(EncodeAuthTicketEvent::EncodeError(err)))?,
    };

    let response = EncodeAuthTicketResponse::new(ticket.into_user(), token);
    Ok(post(EncodeAuthTicketEvent::Success(response)))
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
