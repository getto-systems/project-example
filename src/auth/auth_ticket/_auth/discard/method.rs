use getto_application::data::MethodResult;

use crate::auth::auth_ticket::_auth::{
    discard::infra::DiscardAuthTicketInfra,
    kernel::infra::{AuthClock, AuthTicketInfra, AuthTicketRepository},
};

use super::event::DiscardAuthTicketEvent;

use crate::auth::auth_ticket::_auth::kernel::data::AuthTicket;

pub async fn discard_auth_ticket<S>(
    infra: &impl DiscardAuthTicketInfra,
    auth_ticket: AuthTicket,
    post: impl Fn(DiscardAuthTicketEvent) -> S,
) -> MethodResult<S> {
    let ticket_infra = infra.ticket_infra();
    let clock = ticket_infra.clock();
    let ticket_repository = ticket_infra.ticket_repository();

    ticket_repository
        .discard(auth_ticket, clock.now())
        .await
        .map_err(|err| post(DiscardAuthTicketEvent::RepositoryError(err)))?;

    Ok(post(DiscardAuthTicketEvent::Success))
}
