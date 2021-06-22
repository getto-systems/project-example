use crate::z_details::_api::repository::helper::register_attempt;

use super::super::kernel::infra::{AuthClock, AuthTicketRepository};
use super::infra::IssueAuthTicketInfra;
use crate::auth::auth_ticket::_api::kernel::infra::AuthTicketIdGenerator;

use super::event::IssueAuthTicketEvent;

use crate::auth::{
    auth_ticket::_api::kernel::data::{AuthTicket, AuthTicketId},
    auth_user::_api::kernel::data::AuthUser,
};
use crate::z_details::_api::repository::data::RepositoryError;

pub fn issue_auth_ticket<S>(
    infra: &impl IssueAuthTicketInfra,
    user: AuthUser,
    post: impl Fn(IssueAuthTicketEvent) -> S,
) -> Result<AuthTicket, S> {
    let ticket_id = register_ticket_id(infra)
        .map_err(|err| post(IssueAuthTicketEvent::RepositoryError(err)))?;

    let ticket = AuthTicket::new(ticket_id, user);

    // 呼び出し側を簡単にするため、例外的に State ではなく AuthTicket を返す
    post(IssueAuthTicketEvent::Success(ticket.clone()));
    Ok(ticket)
}

fn register_ticket_id(infra: &impl IssueAuthTicketInfra) -> Result<AuthTicketId, RepositoryError> {
    let config = infra.config();
    let clock = infra.clock();
    let ticket_id_generator = infra.ticket_id_generator();
    let ticket_repository = infra.ticket_repository();

    register_attempt(
        || {
            let ticket_id = ticket_id_generator.generate();
            let limit = clock.now().limit(&config.ticket_expansion_limit);
            let registered_at = clock.now();
            Ok(ticket_repository.register(ticket_id, limit, registered_at)?)
        },
        |err| err,
    )
}
