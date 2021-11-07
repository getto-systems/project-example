use crate::auth::ticket::remote::{
    issue::infra::{AuthTicketIdGenerator, IssueAuthTicketInfra},
    kernel::infra::{AuthClock, IssueAuthTicketRepository},
};

use super::event::IssueAuthTicketEvent;

use crate::auth::{
    ticket::remote::kernel::data::AuthTicket, user::remote::kernel::data::AuthUser,
};

pub async fn issue_auth_ticket<S>(
    infra: &impl IssueAuthTicketInfra,
    user: AuthUser,
    post: impl Fn(IssueAuthTicketEvent) -> S,
) -> Result<AuthTicket, S> {
    let ticket_id_generator = infra.ticket_id_generator();
    let config = infra.config();
    let clock = infra.clock();
    let ticket_repository = infra.ticket_repository();

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
        .await
        .map_err(|err| post(IssueAuthTicketEvent::RepositoryError(err)))?;

    // 呼び出し側を簡単にするため、例外的に State ではなく AuthTicket を返す
    post(IssueAuthTicketEvent::Success(ticket.clone()));
    Ok(ticket)
}
