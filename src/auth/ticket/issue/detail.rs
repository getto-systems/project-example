use std::sync::Arc;

use uuid::Uuid;

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::{
    auth::{
        kernel::detail::ChronoAuthClock,
        ticket::kernel::detail::repository::dynamodb::ticket::{ConnectionTicket, TableTicket},
    },
    common::api::{feature::AsInfra, logger::detail::StdoutJsonLogger},
};

use crate::auth::ticket::issue::infra::{
    AuthTicketIdGenerator, IssueAuthTicketConfig, IssueAuthTicketInfra, IssueAuthTicketLogger,
    IssueAuthTicketRepository,
};

use crate::{
    auth::{
        kernel::data::{AuthDateTime, ExpansionLimitDateTime},
        ticket::kernel::data::{AuthTicket, AuthTicketId},
    },
    common::api::repository::data::RepositoryError,
};

pub struct LiveIssueAuthTicketInfra {
    clock: ChronoAuthClock,
    repository: LiveIssueAuthTicketRepository,
    ticket_id_generator: UuidAuthTicketIdGenerator,
    config: IssueAuthTicketConfig,
}

impl AsInfra<LiveIssueAuthTicketInfra> for Arc<AuthAppFeature> {
    fn as_infra(&self) -> LiveIssueAuthTicketInfra {
        LiveIssueAuthTicketInfra {
            clock: ChronoAuthClock,
            repository: LiveIssueAuthTicketRepository {
                conn: self.as_infra(),
            },
            ticket_id_generator: UuidAuthTicketIdGenerator,
            config: IssueAuthTicketConfig {
                authenticate_expansion_limit: self.config.authenticate_expansion_limit,
            },
        }
    }
}

impl IssueAuthTicketInfra for LiveIssueAuthTicketInfra {
    type Clock = ChronoAuthClock;
    type Repository = LiveIssueAuthTicketRepository;
    type TicketIdGenerator = UuidAuthTicketIdGenerator;

    fn clock(&self) -> &Self::Clock {
        &self.clock
    }
    fn repository(&self) -> &Self::Repository {
        &self.repository
    }
    fn ticket_id_generator(&self) -> &Self::TicketIdGenerator {
        &self.ticket_id_generator
    }
    fn config(&self) -> &IssueAuthTicketConfig {
        &self.config
    }
}

pub struct LiveIssueAuthTicketRepository {
    conn: ConnectionTicket,
}

#[async_trait::async_trait]
impl IssueAuthTicketRepository for LiveIssueAuthTicketRepository {
    async fn register(
        &self,
        ticket: AuthTicket,
        limit: ExpansionLimitDateTime,
        issued_at: AuthDateTime,
    ) -> Result<(), RepositoryError> {
        TableTicket::put_ticket(&self.conn, ticket, limit, issued_at).await
    }
}

pub struct UuidAuthTicketIdGenerator;

impl AuthTicketIdGenerator for UuidAuthTicketIdGenerator {
    fn generate(&self) -> AuthTicketId {
        AuthTicketId::restore(Uuid::new_v4().to_string())
    }
}

impl IssueAuthTicketLogger for StdoutJsonLogger {
    fn try_to_issue_ticket(&self) {
        self.info(format!("try to issue auth-ticket"));
    }
    fn calculate_expansion_limit(&self, limit: ExpansionLimitDateTime) -> ExpansionLimitDateTime {
        self.info(format!("auth-ticket expansion-limit calculated; {}", limit));
        limit
    }
    fn failed_to_register_ticket(&self, err: RepositoryError) -> RepositoryError {
        self.fatal(format!("failed to register auth-ticket; {}", err));
        err
    }
    fn succeed_to_issue_ticket(&self, ticket: AuthTicket) -> AuthTicket {
        self.info(format!("succeed to issue auth-ticket; {}", ticket));
        ticket
    }
}

#[cfg(test)]
pub mod test {
    use std::sync::Arc;

    use crate::{
        auth::{
            kernel::detail::test::MockChronoAuthClock,
            ticket::kernel::detail::repository::memory::{ticket::MapTicket, StoreTicket},
        },
        common::api::feature::AsInfra,
    };

    use crate::auth::ticket::issue::infra::{
        AuthTicketIdGenerator, IssueAuthTicketConfig, IssueAuthTicketInfra,
        IssueAuthTicketRepository,
    };

    use crate::{
        auth::{
            kernel::data::{AuthDateTime, ExpansionLimitDateTime},
            ticket::kernel::data::{AuthTicket, AuthTicketId},
        },
        common::api::repository::data::RepositoryError,
    };

    pub struct MockIssueAuthTicketInfra {
        clock: MockChronoAuthClock,
        repository: MockIssueAuthTicketRepository,
        ticket_id_generator: MockAuthTicketIdGenerator,
        config: IssueAuthTicketConfig,
    }

    impl AsInfra<MockIssueAuthTicketInfra>
        for (
            MockChronoAuthClock,
            Arc<StoreTicket>,
            MockAuthTicketIdGenerator,
            IssueAuthTicketConfig,
        )
    {
        fn as_infra(&self) -> MockIssueAuthTicketInfra {
            MockIssueAuthTicketInfra {
                clock: self.0.clone(),
                repository: MockIssueAuthTicketRepository {
                    ticket: Arc::clone(&self.1),
                },
                ticket_id_generator: self.2.clone(),
                config: self.3.clone(),
            }
        }
    }

    impl IssueAuthTicketInfra for MockIssueAuthTicketInfra {
        type Clock = MockChronoAuthClock;
        type Repository = MockIssueAuthTicketRepository;
        type TicketIdGenerator = MockAuthTicketIdGenerator;

        fn clock(&self) -> &Self::Clock {
            &self.clock
        }
        fn repository(&self) -> &Self::Repository {
            &self.repository
        }
        fn ticket_id_generator(&self) -> &Self::TicketIdGenerator {
            &self.ticket_id_generator
        }
        fn config(&self) -> &IssueAuthTicketConfig {
            &self.config
        }
    }

    pub struct MockIssueAuthTicketRepository {
        ticket: Arc<StoreTicket>,
    }

    #[async_trait::async_trait]
    impl IssueAuthTicketRepository for MockIssueAuthTicketRepository {
        async fn register(
            &self,
            ticket: AuthTicket,
            limit: ExpansionLimitDateTime,
            issued_at: AuthDateTime,
        ) -> Result<(), RepositoryError> {
            Ok(MapTicket::insert_ticket(
                &self.ticket,
                ticket,
                limit,
                issued_at,
            ))
        }
    }

    #[derive(Clone)]
    pub struct MockAuthTicketIdGenerator {
        ticket_id: AuthTicketId,
    }

    impl MockAuthTicketIdGenerator {
        pub const fn new(ticket_id: AuthTicketId) -> Self {
            Self { ticket_id }
        }
    }

    impl AuthTicketIdGenerator for MockAuthTicketIdGenerator {
        fn generate(&self) -> AuthTicketId {
            self.ticket_id.clone()
        }
    }
}
