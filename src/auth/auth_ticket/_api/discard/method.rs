use getto_application::data::MethodResult;

use super::super::kernel::infra::{AuthClock, AuthTicketRepository};
use super::infra::DiscardAuthTicketInfra;

use super::event::DiscardAuthTicketEvent;

use crate::auth::auth_ticket::_api::kernel::data::AuthTicket;

pub fn discard_auth_ticket<S>(
    infra: &impl DiscardAuthTicketInfra,
    auth_ticket: AuthTicket,
    post: impl Fn(DiscardAuthTicketEvent) -> S,
) -> MethodResult<S> {
    let clock = infra.clock();
    let ticket_repository = infra.ticket_repository();

    ticket_repository
        .discard(auth_ticket, clock.now())
        .map_err(|err| post(DiscardAuthTicketEvent::RepositoryError(err)))?;

    Ok(post(DiscardAuthTicketEvent::Success))
}
