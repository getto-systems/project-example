use crate::{
    auth::ticket::kernel::api::data::{AuthDateTime, AuthTicket},
    z_lib::api::repository::data::RepositoryError,
};

#[async_trait::async_trait]
pub trait LogoutAuthTicketRepository {
    async fn discard(
        &self,
        auth_ticket: AuthTicket,
        discard_at: AuthDateTime,
    ) -> Result<(), RepositoryError>;
}
