use std::sync::Arc;

use crate::auth::ticket::logout::api::detail::LiveLogoutInfra;

use crate::auth::ticket::logout::action::LogoutAction;

use crate::auth::ticket::logout::infra::{LogoutInfra, LogoutLogger};

use crate::{
    auth::ticket::logout::data::LogoutSuccess, common::api::repository::data::RepositoryError,
};

impl<M: LogoutInfra> LogoutAction<M> {
    fn new(infra: M) -> Self {
        Self {
            logger: Arc::new(NoopLogger),
            infra,
        }
    }

    pub fn with_logger(self, logger: Arc<dyn LogoutLogger>) -> Self {
        Self { logger, ..self }
    }
}

impl LogoutAction<LiveLogoutInfra> {
    pub fn live(infra: LiveLogoutInfra) -> Self {
        Self::new(infra)
    }
}

struct NoopLogger;

impl LogoutLogger for NoopLogger {
    fn try_to_logout(&self) {}
    fn failed_to_discard_ticket(&self, err: RepositoryError) -> RepositoryError {
        err
    }
    fn succeed_to_logout(&self, auth: LogoutSuccess) -> LogoutSuccess {
        auth
    }
}

#[cfg(test)]
mod test {
    use crate::auth::ticket::logout::api::detail::test::MockLogoutInfra;

    use crate::auth::ticket::logout::action::LogoutAction;

    impl LogoutAction<MockLogoutInfra> {
        pub fn mock(infra: MockLogoutInfra) -> Self {
            Self::new(infra)
        }
    }
}
