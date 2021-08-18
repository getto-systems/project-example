use getto_application::data::MethodResult;

use crate::auth::auth_ticket::_auth::{
    discard::infra::DiscardAuthTicketInfra,
    kernel::infra::{AuthClock, DiscardAuthTicketRepository},
};

use super::event::DiscardAuthTicketEvent;

use crate::auth::auth_ticket::_auth::kernel::data::AuthTicket;

pub async fn discard_auth_ticket<S>(
    infra: &impl DiscardAuthTicketInfra,
    ticket: AuthTicket,
    post: impl Fn(DiscardAuthTicketEvent) -> S,
) -> MethodResult<S> {
    let clock = infra.clock();
    let ticket_repository = infra.ticket_repository();

    ticket_repository
        .discard(ticket, clock.now())
        .await
        .map_err(|err| post(DiscardAuthTicketEvent::RepositoryError(err)))?;

    Ok(post(DiscardAuthTicketEvent::Success))
}
