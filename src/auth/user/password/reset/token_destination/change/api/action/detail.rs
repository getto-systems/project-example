use std::sync::Arc;

use crate::auth::user::password::reset::token_destination::change::api::detail::LiveChangeResetTokenDestinationInfra;

use crate::auth::user::password::reset::token_destination::change::action::ChangeResetTokenDestinationAction;

use crate::auth::user::password::reset::token_destination::change::infra::{
    ChangeResetTokenDestinationInfra, ChangeResetTokenDestinationLogger,
};

use crate::{
    auth::user::password::reset::token_destination::change::data::{
        ChangeResetTokenDestinationError, ChangeResetTokenDestinationSuccess,
        ValidateChangeResetTokenDestinationFieldsError,
    },
    common::api::repository::data::RepositoryError,
};

impl<M: ChangeResetTokenDestinationInfra> ChangeResetTokenDestinationAction<M> {
    fn new(infra: M) -> Self {
        Self {
            logger: Arc::new(NoopLogger),
            infra,
        }
    }

    pub fn with_logger(self, logger: Arc<dyn ChangeResetTokenDestinationLogger>) -> Self {
        Self { logger, ..self }
    }
}

impl ChangeResetTokenDestinationAction<LiveChangeResetTokenDestinationInfra> {
    pub fn live(infra: LiveChangeResetTokenDestinationInfra) -> Self {
        Self::new(infra)
    }
}

struct NoopLogger;

impl ChangeResetTokenDestinationLogger for NoopLogger {
    fn try_to_change_destination(&self) {}
    fn invalid_request(
        &self,
        err: ValidateChangeResetTokenDestinationFieldsError,
    ) -> ValidateChangeResetTokenDestinationFieldsError {
        err
    }
    fn failed_to_lookup_destination(&self, err: RepositoryError) -> RepositoryError {
        err
    }
    fn user_not_found(
        &self,
        err: ChangeResetTokenDestinationError,
    ) -> ChangeResetTokenDestinationError {
        err
    }
    fn conflict(&self, err: ChangeResetTokenDestinationError) -> ChangeResetTokenDestinationError {
        err
    }
    fn failed_to_change_destination(&self, err: RepositoryError) -> RepositoryError {
        err
    }
    fn succeed_to_change_destination(
        &self,
        auth: ChangeResetTokenDestinationSuccess,
    ) -> ChangeResetTokenDestinationSuccess {
        auth
    }
}

#[cfg(test)]
mod test {
    use crate::auth::user::password::reset::token_destination::change::api::detail::test::MockChangeResetTokenDestinationInfra;

    use crate::auth::user::password::reset::token_destination::change::action::ChangeResetTokenDestinationAction;

    impl ChangeResetTokenDestinationAction<MockChangeResetTokenDestinationInfra> {
        pub fn mock(infra: MockChangeResetTokenDestinationInfra) -> Self {
            Self::new(infra)
        }
    }
}
