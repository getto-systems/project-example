use crate::{
    auth::ticket::kernel::data::AuthTicket, common::api::repository::data::RepositoryError,
};

#[async_trait::async_trait]
pub trait LogoutAuthTicketRepository {
    async fn discard(&self, ticket: &AuthTicket) -> Result<(), RepositoryError>;
}
