use super::super::kernel::infra::AuthTicketInfra;

use crate::auth::auth_ticket::_api::kernel::data::{AuthTicketId, ExpansionLimitDuration};

pub trait IssueAuthTicketInfra {
    type TicketInfra: AuthTicketInfra;
    type TicketIdGenerator: AuthTicketIdGenerator;

    fn ticket_infra(&self) -> &Self::TicketInfra;
    fn ticket_id_generator(&self) -> &Self::TicketIdGenerator;
    fn config(&self) -> &IssueAuthTicketConfig;
}

pub struct IssueAuthTicketConfig {
    pub ticket_expansion_limit: ExpansionLimitDuration,
}

pub trait AuthTicketIdGenerator {
    fn generate(&self) -> AuthTicketId;
}
