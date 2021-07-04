use crate::auth::auth_ticket::_api::{
    issue::infra::{AuthTicketIdGenerator, IssueAuthTicketInfra},
    kernel::infra::{AuthClock, AuthTicketInfra, AuthTicketRepository},
};

use super::event::IssueAuthTicketEvent;

use crate::auth::{
    auth_ticket::_api::kernel::data::AuthTicket, auth_user::_api::kernel::data::AuthUser,
};

pub fn issue_auth_ticket<S>(
    infra: &impl IssueAuthTicketInfra,
    user: AuthUser,
    post: impl Fn(IssueAuthTicketEvent) -> S,
) -> Result<AuthTicket, S> {
    let ticket_infra = infra.ticket_infra();
    let clock = ticket_infra.clock();
    let ticket_repository = ticket_infra.ticket_repository();
    let ticket_id_generator = infra.ticket_id_generator();
    let config = infra.config();

    let ticket = AuthTicket::new(ticket_id_generator.generate(), user);

    let issued_at = clock.now();
    let limit = issued_at
        .clone()
        .expansion_limit(&config.ticket_expansion_limit);
    post(IssueAuthTicketEvent::ExpansionLimitCalculated(
        limit.clone(),
    ));

    ticket_repository
        .issue(ticket.clone(), limit, issued_at)
        .map_err(|err| post(IssueAuthTicketEvent::RepositoryError(err)))?;

    // 呼び出し側を簡単にするため、例外的に State ではなく AuthTicket を返す
    post(IssueAuthTicketEvent::Success(ticket.clone()));
    Ok(ticket)
}
