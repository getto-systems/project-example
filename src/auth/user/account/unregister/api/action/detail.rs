use std::sync::Arc;

use crate::auth::user::account::unregister::api::detail::LiveUnregisterAuthUserAccountInfra;

use crate::auth::user::account::unregister::action::UnregisterAuthUserAccountAction;

use crate::auth::user::account::unregister::infra::{
    UnregisterAuthUserAccountInfra, UnregisterAuthUserAccountLogger,
};

use crate::{
    auth::user::{
        account::unregister::data::{
            UnregisterAuthUserAccountError, UnregisterAuthUserAccountSuccess,
        },
        login_id::kernel::data::ValidateLoginIdError,
    },
    common::api::repository::data::RepositoryError,
};

impl<M: UnregisterAuthUserAccountInfra> UnregisterAuthUserAccountAction<M> {
    fn new(infra: M) -> Self {
        Self {
            logger: Arc::new(NoopLogger),
            infra,
        }
    }

    pub fn with_logger(self, logger: Arc<dyn UnregisterAuthUserAccountLogger>) -> Self {
        Self { logger, ..self }
    }
}

impl UnregisterAuthUserAccountAction<LiveUnregisterAuthUserAccountInfra> {
    pub fn live(infra: LiveUnregisterAuthUserAccountInfra) -> Self {
        Self::new(infra)
    }
}

struct NoopLogger;

impl UnregisterAuthUserAccountLogger for NoopLogger {
    fn try_to_unregister_auth_user_account(&self) {}
    fn invalid_request(&self, err: ValidateLoginIdError) -> ValidateLoginIdError {
        err
    }
    fn failed_to_lookup_user_id(&self, err: RepositoryError) -> RepositoryError {
        err
    }
    fn user_id_not_found(
        &self,
        err: UnregisterAuthUserAccountError,
    ) -> UnregisterAuthUserAccountError {
        err
    }
    fn failed_to_unregister_user(&self, err: RepositoryError) -> RepositoryError {
        err
    }
    fn failed_to_discard_all_ticket(&self, err: RepositoryError) -> RepositoryError {
        err
    }
    fn succeed_to_unregister_auth_user_account(
        &self,
        success: UnregisterAuthUserAccountSuccess,
    ) -> UnregisterAuthUserAccountSuccess {
        success
    }
}

#[cfg(test)]
mod test {
    use crate::auth::user::account::unregister::api::detail::test::MockUnregisterAuthUserAccountInfra;

    use crate::auth::user::account::unregister::action::UnregisterAuthUserAccountAction;

    impl UnregisterAuthUserAccountAction<MockUnregisterAuthUserAccountInfra> {
        pub fn mock(infra: MockUnregisterAuthUserAccountInfra) -> Self {
            Self::new(infra)
        }
    }
}
