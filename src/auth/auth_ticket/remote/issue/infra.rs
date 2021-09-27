use crate::auth::auth_ticket::remote::kernel::infra::{AuthClock, IssueAuthTicketRepository};

use crate::auth::auth_ticket::remote::kernel::data::{AuthTicketId, ExpansionLimitDuration};

pub trait IssueAuthTicketInfra {
    type Clock: AuthClock;
    type TicketRepository: IssueAuthTicketRepository;
    type TicketIdGenerator: AuthTicketIdGenerator;

    fn clock(&self) -> &Self::Clock;
    fn ticket_repository(&self) -> &Self::TicketRepository;
    fn ticket_id_generator(&self) -> &Self::TicketIdGenerator;
    fn config(&self) -> &IssueAuthTicketConfig;
}

pub trait AuthTicketIdGenerator {
    fn generate(&self) -> AuthTicketId;
}

pub struct IssueAuthTicketConfig {
    pub ticket_expansion_limit: ExpansionLimitDuration,
}
