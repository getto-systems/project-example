use std::sync::Arc;

use crate::auth::user::account::register::api::detail::LiveRegisterAuthUserAccountInfra;

use crate::auth::user::account::register::action::RegisterAuthUserAccountAction;

use crate::auth::user::account::register::infra::{
    RegisterAuthUserAccountInfra, RegisterAuthUserAccountLogger,
};

use crate::{
    auth::user::account::{
        kernel::data::ValidateAuthUserAccountError,
        register::data::{RegisterAuthUserAccountError, RegisterAuthUserAccountSuccess},
    },
    common::api::repository::data::RepositoryError,
};

impl<M: RegisterAuthUserAccountInfra> RegisterAuthUserAccountAction<M> {
    fn new(infra: M) -> Self {
        Self {
            logger: Arc::new(NoopLogger),
            infra,
        }
    }

    pub fn with_logger(self, logger: Arc<dyn RegisterAuthUserAccountLogger>) -> Self {
        Self { logger, ..self }
    }
}

impl RegisterAuthUserAccountAction<LiveRegisterAuthUserAccountInfra> {
    pub fn live(infra: LiveRegisterAuthUserAccountInfra) -> Self {
        Self::new(infra)
    }
}

struct NoopLogger;

impl RegisterAuthUserAccountLogger for NoopLogger {
    fn try_to_register_auth_user_account(&self) {}
    fn invalid_request(&self, err: ValidateAuthUserAccountError) -> ValidateAuthUserAccountError {
        err
    }
    fn failed_to_check_login_id_registered(&self, err: RepositoryError) -> RepositoryError {
        err
    }
    fn login_id_already_registered(
        &self,
        err: RegisterAuthUserAccountError,
    ) -> RegisterAuthUserAccountError {
        err
    }
    fn failed_to_register_user(&self, err: RepositoryError) -> RepositoryError {
        err
    }
    fn succeed_to_register_auth_user_account(
        &self,
        success: RegisterAuthUserAccountSuccess,
    ) -> RegisterAuthUserAccountSuccess {
        success
    }
}

#[cfg(test)]
mod test {
    use crate::auth::user::account::register::api::detail::test::MockRegisterAuthUserAccountInfra;

    use crate::auth::user::account::register::action::RegisterAuthUserAccountAction;

    impl RegisterAuthUserAccountAction<MockRegisterAuthUserAccountInfra> {
        pub fn mock(infra: MockRegisterAuthUserAccountInfra) -> Self {
            Self::new(infra)
        }
    }
}
