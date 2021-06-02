pub mod id_generator;

use super::super::kernel::infra::{AuthClock, AuthTicketRepository};

use crate::auth::auth_ticket::_api::kernel::data::{AuthTicketId, ExpansionLimitDuration};

pub trait IssueAuthTicketInfra {
    type Clock: AuthClock;
    type TicketRepository: AuthTicketRepository;
    type TicketIdGenerator: AuthTicketIdGenerator;

    fn config(&self) -> &IssueAuthTicketConfig;
    fn clock(&self) -> &Self::Clock;
    fn ticket_repository(&self) -> &Self::TicketRepository;
    fn ticket_id_generator(&self) -> &Self::TicketIdGenerator;
}

pub struct IssueAuthTicketConfig {
    pub ticket_expansion_limit: ExpansionLimitDuration,
}

pub trait AuthTicketIdGenerator {
    fn generate(&self) -> AuthTicketId;
}
