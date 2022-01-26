use crate::{
    auth::ticket::remote::{
        issue::infra::{AuthTicketIdGenerator, IssueAuthTicketRepository},
        kernel::infra::AuthClock,
    },
    z_lib::remote::repository::data::RepositoryError,
};

use crate::auth::{
    ticket::remote::kernel::data::{AuthTicket, ExpansionLimitDateTime, ExpansionLimitDuration},
    user::remote::kernel::data::AuthUser,
};

pub enum IssueAuthTicketEvent {
    ExpansionLimitCalculated(ExpansionLimitDateTime),
    Success(AuthTicket),
    RepositoryError(RepositoryError),
}

const SUCCESS: &'static str = "issue success";
const ERROR: &'static str = "issue error";

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

pub struct IssueAuthTicketConfig {
    pub ticket_expansion_limit: ExpansionLimitDuration,
}

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
