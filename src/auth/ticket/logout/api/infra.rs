use crate::{auth::ticket::kernel::data::AuthTicket, z_lib::repository::data::RepositoryError};

#[async_trait::async_trait]
pub trait LogoutAuthTicketRepository {
    async fn discard(&self, ticket: &AuthTicket) -> Result<(), RepositoryError>;
}
