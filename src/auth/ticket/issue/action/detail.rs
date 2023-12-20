use std::sync::Arc;

use crate::auth::ticket::issue::detail::LiveIssueAuthTicketInfra;

use crate::auth::ticket::issue::action::IssueAuthTicketAction;

use crate::auth::ticket::issue::infra::{IssueAuthTicketInfra, IssueAuthTicketLogger};

use crate::{
    auth::{kernel::data::ExpansionLimitDateTime, ticket::kernel::data::AuthTicket},
    common::api::repository::data::RepositoryError,
};

impl<M: IssueAuthTicketInfra> IssueAuthTicketAction<M> {
    fn new(infra: M) -> Self {
        Self {
            logger: Arc::new(NoopLogger),
            infra,
        }
    }

    pub fn with_logger(self, logger: Arc<dyn IssueAuthTicketLogger>) -> Self {
        Self { logger, ..self }
    }
}

impl IssueAuthTicketAction<LiveIssueAuthTicketInfra> {
    pub fn live(infra: LiveIssueAuthTicketInfra) -> Self {
        Self::new(infra)
    }
}

struct NoopLogger;

impl IssueAuthTicketLogger for NoopLogger {
    fn try_to_issue_ticket(&self) {}
    fn calculate_expansion_limit(&self, limit: ExpansionLimitDateTime) -> ExpansionLimitDateTime {
        limit
    }
    fn failed_to_register_ticket(&self, err: RepositoryError) -> RepositoryError {
        err
    }
    fn succeed_to_issue_ticket(&self, ticket: AuthTicket) -> AuthTicket {
        ticket
    }
}

#[cfg(test)]
mod test {
    use crate::auth::ticket::issue::detail::test::MockIssueAuthTicketInfra;

    use crate::auth::ticket::issue::action::IssueAuthTicketAction;

    impl IssueAuthTicketAction<MockIssueAuthTicketInfra> {
        pub fn mock(infra: MockIssueAuthTicketInfra) -> Self {
            Self::new(infra)
        }
    }
}
