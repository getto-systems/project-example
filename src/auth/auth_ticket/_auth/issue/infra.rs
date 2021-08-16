use crate::auth::auth_ticket::_auth::kernel::infra::{AuthClock, AuthTicketRepository};

use crate::auth::auth_ticket::_auth::kernel::data::{AuthTicketId, ExpansionLimitDuration};

pub trait IssueAuthTicketInfra {
    type Clock: AuthClock;
    type TicketRepository: AuthTicketRepository;
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
