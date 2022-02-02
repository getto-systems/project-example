use crate::{
    auth::ticket::kernel::remote::data::{AuthDateTime, AuthTicket},
    z_lib::remote::repository::data::RepositoryError,
};

#[async_trait::async_trait]
pub trait LogoutAuthTicketRepository {
    async fn discard(
        &self,
        auth_ticket: AuthTicket,
        discard_at: AuthDateTime,
    ) -> Result<(), RepositoryError>;
}
