use getto_application::data::MethodResult;

use crate::auth::auth_ticket::_auth::{
    discard::infra::DiscardAuthTicketInfra,
    kernel::infra::{AuthClock, DiscardAuthTicketRepository},
};

use crate::auth::auth_ticket::_auth::validate::method::validate_ticket_token;

use super::event::DiscardAuthTicketEvent;

pub async fn discard_auth_ticket<S>(
    infra: &impl DiscardAuthTicketInfra,
    post: impl Fn(DiscardAuthTicketEvent) -> S,
) -> MethodResult<S> {
    let ticket = validate_ticket_token(infra.validate_infra(), |event| {
        post(DiscardAuthTicketEvent::Validate(event))
    })
    .await?;

    let clock = infra.clock();
    let ticket_repository = infra.ticket_repository();

    ticket_repository
        .discard(ticket, clock.now())
        .await
        .map_err(|err| post(DiscardAuthTicketEvent::RepositoryError(err)))?;

    Ok(post(DiscardAuthTicketEvent::Success))
}
