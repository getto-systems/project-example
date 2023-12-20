use crate::{
    auth::ticket::{kernel::data::AuthTicket, logout::data::LogoutSuccess},
    common::api::repository::data::RepositoryError,
};

pub trait LogoutInfra {
    type Repository: LogoutRepository;

    fn repository(&self) -> &Self::Repository;
}

#[async_trait::async_trait]
pub trait LogoutRepository {
    async fn discard(&self, ticket: &AuthTicket) -> Result<(), RepositoryError>;
}

pub trait LogoutLogger: Send + Sync {
    fn try_to_logout(&self);
    fn failed_to_discard_ticket(&self, err: RepositoryError) -> RepositoryError;
    fn succeed_to_logout(&self, auth: LogoutSuccess) -> LogoutSuccess;
}
