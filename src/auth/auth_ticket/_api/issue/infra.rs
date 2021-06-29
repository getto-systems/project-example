use super::super::kernel::infra::{AuthClock, AuthTicketIdGenerator, AuthTicketRepository};

use crate::auth::auth_ticket::_api::kernel::data::ExpansionLimitDuration;

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
