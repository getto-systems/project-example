use crate::auth::{
    kernel::infra::AuthClock,
    ticket::issue::infra::{
        AuthTicketIdGenerator, IssueAuthTicketConfig, IssueAuthTicketRepository,
    },
};

use crate::{
    auth::{
        kernel::data::ExpansionLimitDateTime,
        ticket::kernel::data::{AuthTicket, AuthTicketAttrs},
    },
    common::api::repository::data::RepositoryError,
};

pub enum IssueAuthTicketEvent {
    ExpansionLimitCalculated(ExpansionLimitDateTime),
    Success(AuthTicket),
    RepositoryError(RepositoryError),
}

const SUCCESS: &'static str = "issue auth-ticket success";
const ERROR: &'static str = "issue auth-ticket error";

impl std::fmt::Display for IssueAuthTicketEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExpansionLimitCalculated(limit) => {
                write!(f, "expansion limit calculated; {}", limit)
            }
            Self::Success(ticket) => write!(f, "{}; {}", SUCCESS, ticket),
            Self::RepositoryError(err) => write!(f, "{}: {}", ERROR, err),
        }
    }
}

pub trait IssueAuthTicketInfra {
    type Clock: AuthClock;
    type TicketRepository: IssueAuthTicketRepository;
    type TicketIdGenerator: AuthTicketIdGenerator;

    fn clock(&self) -> &Self::Clock;
    fn ticket_repository(&self) -> &Self::TicketRepository;
    fn ticket_id_generator(&self) -> &Self::TicketIdGenerator;
    fn config(&self) -> &IssueAuthTicketConfig;
}

pub async fn issue_auth_ticket<S>(
    infra: &impl IssueAuthTicketInfra,
    attrs: AuthTicketAttrs,
    post: impl Fn(IssueAuthTicketEvent) -> S,
) -> Result<AuthTicket, S> {
    let ticket = AuthTicket {
        ticket_id: infra.ticket_id_generator().generate(),
        attrs,
    };

    let issued_at = infra.clock().now();
    let limit = issued_at.expansion_limit(&infra.config().authenticate_expansion_limit);

    post(IssueAuthTicketEvent::ExpansionLimitCalculated(
        limit.clone(),
    ));

    infra
        .ticket_repository()
        .register(ticket.clone(), limit, issued_at)
        .await
        .map_err(|err| post(IssueAuthTicketEvent::RepositoryError(err)))?;

    // 呼び出し側を簡単にするため、例外的に State ではなく AuthTicket を返す
    post(IssueAuthTicketEvent::Success(ticket.clone()));
    Ok(ticket)
}
