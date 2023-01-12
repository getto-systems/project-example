use crate::{
    auth::{
        kernel::data::{AuthDateTime, ExpansionLimitDateTime, ExpansionLimitDuration},
        ticket::kernel::data::{AuthTicket, AuthTicketId},
    },
    common::api::repository::data::RepositoryError,
};

#[async_trait::async_trait]
pub trait IssueAuthTicketRepository {
    async fn register(
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
    pub authenticate_expansion_limit: ExpansionLimitDuration,
}
