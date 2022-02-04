use crate::{
    auth::ticket::kernel::api::data::{
        AuthDateTime, AuthTicket, AuthTicketId, ExpansionLimitDateTime,
    },
    z_lib::api::repository::data::RepositoryError,
};

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
