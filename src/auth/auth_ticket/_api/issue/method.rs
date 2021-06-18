use super::super::kernel::infra::{AuthClock, AuthTicketRepository};
use super::infra::IssueAuthTicketInfra;

use super::event::IssueAuthTicketEvent;

use crate::auth::auth_ticket::_api::kernel::data::AuthTicket;
use crate::auth::auth_user::_api::kernel::data::AuthUser;

pub fn issue_auth_ticket<S>(
    infra: &impl IssueAuthTicketInfra,
    user: AuthUser,
    post: impl Fn(IssueAuthTicketEvent) -> S,
) -> Result<AuthTicket, S> {
    let config = infra.config();
    let clock = infra.clock();
    let ticket_repository = infra.ticket_repository();
    let ticket_id_generator = infra.ticket_id_generator();

    let id = ticket_repository
        .register(
            ticket_id_generator,
            clock.now().limit(&config.ticket_expansion_limit),
            clock.now(),
        )
        .map_err(|err| post(IssueAuthTicketEvent::RepositoryError(err)))?;

    let ticket = AuthTicket::new(id, user);

    // 呼び出し側を簡単にするため、例外的に State ではなく AuthTicket を返す
    post(IssueAuthTicketEvent::Success(ticket.clone()));
    Ok(ticket)
}
