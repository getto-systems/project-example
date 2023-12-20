use std::sync::Arc;

use crate::avail::unexpected_error::notify::api::detail::LiveNotifyUnexpectedErrorInfra;

use crate::avail::unexpected_error::notify::action::NotifyUnexpectedErrorAction;

use crate::avail::unexpected_error::notify::infra::{
    NotifyUnexpectedErrorInfra, NotifyUnexpectedErrorLogger,
};

use crate::avail::unexpected_error::notify::data::NotifyUnexpectedError;

impl<M: NotifyUnexpectedErrorInfra> NotifyUnexpectedErrorAction<M> {
    fn new(infra: M) -> Self {
        Self {
            logger: Arc::new(NoopLogger),
            infra,
        }
    }

    pub fn with_logger(self, logger: Arc<dyn NotifyUnexpectedErrorLogger>) -> Self {
        Self { logger, ..self }
    }
}

impl NotifyUnexpectedErrorAction<LiveNotifyUnexpectedErrorInfra> {
    pub fn live(infra: LiveNotifyUnexpectedErrorInfra) -> Self {
        Self::new(infra)
    }
}

struct NoopLogger;

impl NotifyUnexpectedErrorLogger for NoopLogger {
    fn unexpected_error_occurred(&self, err: NotifyUnexpectedError) -> NotifyUnexpectedError {
        err
    }
}

#[cfg(test)]
mod test {
    use crate::avail::unexpected_error::notify::api::detail::test::MockNotifyUnexpectedErrorInfra;

    use crate::avail::unexpected_error::notify::action::NotifyUnexpectedErrorAction;

    impl NotifyUnexpectedErrorAction<MockNotifyUnexpectedErrorInfra> {
        pub fn mock(infra: MockNotifyUnexpectedErrorInfra) -> Self {
            Self::new(infra)
        }
    }
}
