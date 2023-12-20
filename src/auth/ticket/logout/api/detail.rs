use std::sync::Arc;

use crate::{
    auth::ticket::kernel::detail::repository::dynamodb::ticket::{ConnectionTicket, TableTicket},
    common::api::{feature::AsInfra, logger::detail::StdoutJsonLogger},
    x_outside_feature::auth::feature::AuthAppFeature,
};

use crate::auth::ticket::logout::infra::{LogoutInfra, LogoutLogger, LogoutRepository};

use crate::{
    auth::ticket::{kernel::data::AuthTicket, logout::data::LogoutSuccess},
    common::api::repository::data::RepositoryError,
};

pub struct LiveLogoutInfra {
    repository: LiveLogoutRepository,
}

impl AsInfra<LiveLogoutInfra> for Arc<AuthAppFeature> {
    fn as_infra(&self) -> LiveLogoutInfra {
        LiveLogoutInfra {
            repository: LiveLogoutRepository {
                ticket: self.as_infra(),
            },
        }
    }
}

impl LogoutInfra for LiveLogoutInfra {
    type Repository = LiveLogoutRepository;

    fn repository(&self) -> &Self::Repository {
        &self.repository
    }
}

pub struct LiveLogoutRepository {
    ticket: ConnectionTicket,
}

#[async_trait::async_trait]
impl LogoutRepository for LiveLogoutRepository {
    async fn discard(&self, ticket: &AuthTicket) -> Result<(), RepositoryError> {
        let ticket = ticket.clone();
        TableTicket::delete_ticket(&self.ticket, ticket.ticket_id, ticket.attrs.user_id).await
    }
}

impl LogoutLogger for StdoutJsonLogger {
    fn try_to_logout(&self) {
        self.info(format!("try to logout"));
    }
    fn failed_to_discard_ticket(&self, err: RepositoryError) -> RepositoryError {
        self.fatal(format!("failed to discard ticket; {}", err));
        err
    }
    fn succeed_to_logout(&self, auth: LogoutSuccess) -> LogoutSuccess {
        self.info(format!("succeed to logout; {}", auth));
        auth
    }
}

#[cfg(test)]
pub mod test {
    use std::sync::Arc;

    use crate::{
        auth::ticket::kernel::detail::repository::memory::{ticket::MapTicket, StoreTicket},
        common::api::feature::AsInfra,
    };

    use crate::auth::ticket::logout::infra::{LogoutInfra, LogoutRepository};

    use crate::{
        auth::ticket::kernel::data::AuthTicket, common::api::repository::data::RepositoryError,
    };

    pub struct MockLogoutInfra {
        repository: MockLogoutRepository,
    }

    impl AsInfra<MockLogoutInfra> for Arc<StoreTicket> {
        fn as_infra(&self) -> MockLogoutInfra {
            MockLogoutInfra {
                repository: MockLogoutRepository {
                    ticket: Arc::clone(&self),
                },
            }
        }
    }

    impl LogoutInfra for MockLogoutInfra {
        type Repository = MockLogoutRepository;

        fn repository(&self) -> &Self::Repository {
            &self.repository
        }
    }

    pub struct MockLogoutRepository {
        ticket: Arc<StoreTicket>,
    }

    #[async_trait::async_trait]
    impl LogoutRepository for MockLogoutRepository {
        async fn discard(&self, ticket: &AuthTicket) -> Result<(), RepositoryError> {
            Ok(MapTicket::remove_ticket(&self.ticket, &ticket.ticket_id))
        }
    }
}
