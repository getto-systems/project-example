use crate::auth::ticket::remote::kernel::infra::AuthClock;

use crate::{
    auth::ticket::remote::kernel::data::{
        AuthDateTime, AuthTicket, AuthTicketId, ExpansionLimitDateTime, ExpansionLimitDuration,
    },
    z_lib::remote::repository::data::RepositoryError,
};

pub trait IssueAuthTicketInfra {
    type Clock: AuthClock;
    type TicketRepository: IssueAuthTicketRepository;
    type TicketIdGenerator: AuthTicketIdGenerator;

    fn clock(&self) -> &Self::Clock;
    fn ticket_repository(&self) -> &Self::TicketRepository;
    fn ticket_id_generator(&self) -> &Self::TicketIdGenerator;
    fn config(&self) -> &IssueAuthTicketConfig;
}

#[async_trait::async_trait]
pub trait IssueAuthTicketRepository {
    async fn issue(
        &self,
        ticket: AuthTicket,
        limit: ExpansionLimitDateTime,
        issued_at: AuthDateTime,
    ) -> Result<(), RepositoryError>;
}

pub trait AuthTicketIdGenerator {
    fn generate(&self) -> AuthTicketId;
}

pub struct IssueAuthTicketConfig {
    pub ticket_expansion_limit: ExpansionLimitDuration,
}
