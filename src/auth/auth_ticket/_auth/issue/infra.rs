use super::super::kernel::infra::AuthTicketInfra;

use crate::auth::auth_ticket::_auth::kernel::data::{AuthTicketId, ExpansionLimitDuration};

pub trait IssueAuthTicketInfra {
    type TicketInfra: AuthTicketInfra;
    type TicketIdGenerator: AuthTicketIdGenerator;

    fn extract(
        self,
    ) -> (
        Self::TicketInfra,
        Self::TicketIdGenerator,
        IssueAuthTicketConfig,
    );
}

pub trait AuthTicketIdGenerator {
    fn generate(&self) -> AuthTicketId;
}

pub struct IssueAuthTicketConfig {
    pub ticket_expansion_limit: ExpansionLimitDuration,
}
