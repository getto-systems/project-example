use std::sync::Arc;

use crate::auth::user::login_id::change::api::detail::LiveOverwriteLoginIdInfra;

use crate::auth::user::login_id::change::action::OverwriteLoginIdAction;

use crate::auth::user::login_id::change::infra::{OverwriteLoginIdInfra, OverwriteLoginIdLogger};

use crate::{
    auth::user::login_id::change::data::{
        OverwriteLoginIdError, OverwriteLoginIdSuccess, ValidateOverwriteLoginIdFieldsError,
    },
    common::api::repository::data::RepositoryError,
};

impl<M: OverwriteLoginIdInfra> OverwriteLoginIdAction<M> {
    fn new(infra: M) -> Self {
        Self {
            logger: Arc::new(NoopLogger),
            infra,
        }
    }

    pub fn with_logger(self, logger: Arc<dyn OverwriteLoginIdLogger>) -> Self {
        Self { logger, ..self }
    }
}

impl OverwriteLoginIdAction<LiveOverwriteLoginIdInfra> {
    pub fn live(infra: LiveOverwriteLoginIdInfra) -> Self {
        Self::new(infra)
    }
}

struct NoopLogger;

impl OverwriteLoginIdLogger for NoopLogger {
    fn try_to_overwrite_login_id(&self) {}
    fn invalid_request(
        &self,
        err: ValidateOverwriteLoginIdFieldsError,
    ) -> ValidateOverwriteLoginIdFieldsError {
        err
    }
    fn failed_to_check_login_id_registered(&self, err: RepositoryError) -> RepositoryError {
        err
    }
    fn login_id_already_registered(&self, err: OverwriteLoginIdError) -> OverwriteLoginIdError {
        err
    }
    fn failed_to_drop_login_id_entry(&self, err: RepositoryError) -> RepositoryError {
        err
    }
    fn login_id_entry_not_found(&self, err: OverwriteLoginIdError) -> OverwriteLoginIdError {
        err
    }
    fn failed_to_insert_login_id_entry(&self, err: RepositoryError) -> RepositoryError {
        err
    }
    fn failed_to_update_login_id(&self, err: RepositoryError) -> RepositoryError {
        err
    }
    fn succeed_to_overwrite_login_id(
        &self,
        auth: OverwriteLoginIdSuccess,
    ) -> OverwriteLoginIdSuccess {
        auth
    }
}

#[cfg(test)]
mod test {
    use crate::auth::user::login_id::change::api::detail::test::MockOverwriteLoginIdInfra;

    use crate::auth::user::login_id::change::action::OverwriteLoginIdAction;

    impl OverwriteLoginIdAction<MockOverwriteLoginIdInfra> {
        pub fn mock(infra: MockOverwriteLoginIdInfra) -> Self {
            Self::new(infra)
        }
    }
}
