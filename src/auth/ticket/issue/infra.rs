use crate::{
    auth::ticket::remote::kernel::data::{
        AuthDateTime, AuthTicket, AuthTicketId, ExpansionLimitDateTime,
    },
    z_lib::remote::repository::data::RepositoryError,
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
