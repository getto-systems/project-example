use std::sync::Arc;

use crate::auth::user::account::modify::api::detail::LiveModifyAuthUserAccountInfra;

use crate::auth::user::account::modify::action::ModifyAuthUserAccountAction;

use crate::auth::user::account::modify::infra::{
    ModifyAuthUserAccountInfra, ModifyAuthUserAccountLogger,
};

use crate::{
    auth::user::account::modify::data::{
        ModifyAuthUserAccountError, ModifyAuthUserAccountSuccess,
        ValidateModifyAuthUserAccountFieldsError,
    },
    common::api::repository::data::RepositoryError,
};

impl<M: ModifyAuthUserAccountInfra> ModifyAuthUserAccountAction<M> {
    fn new(infra: M) -> Self {
        Self {
            logger: Arc::new(NoopLogger),
            infra,
        }
    }

    pub fn with_logger(self, logger: Arc<dyn ModifyAuthUserAccountLogger>) -> Self {
        Self { logger, ..self }
    }
}

impl ModifyAuthUserAccountAction<LiveModifyAuthUserAccountInfra> {
    pub fn live(infra: LiveModifyAuthUserAccountInfra) -> Self {
        Self::new(infra)
    }
}

struct NoopLogger;

impl ModifyAuthUserAccountLogger for NoopLogger {
    fn try_to_modify_auth_user_account(&self) {}
    fn invalid_request(
        &self,
        err: ValidateModifyAuthUserAccountFieldsError,
    ) -> ValidateModifyAuthUserAccountFieldsError {
        err
    }
    fn failed_to_lookup_user_id(&self, err: RepositoryError) -> RepositoryError {
        err
    }
    fn user_id_not_found(&self, err: ModifyAuthUserAccountError) -> ModifyAuthUserAccountError {
        err
    }
    fn failed_to_lookup_attrs(&self, err: RepositoryError) -> RepositoryError {
        err
    }
    fn user_attrs_not_found(&self, err: ModifyAuthUserAccountError) -> ModifyAuthUserAccountError {
        err
    }
    fn conflict(&self, err: ModifyAuthUserAccountError) -> ModifyAuthUserAccountError {
        err
    }
    fn failed_to_modify_attrs(&self, err: RepositoryError) -> RepositoryError {
        err
    }
    fn succeed_to_modify_auth_user_account(
        &self,
        success: ModifyAuthUserAccountSuccess,
    ) -> ModifyAuthUserAccountSuccess {
        success
    }
}

#[cfg(test)]
mod test {
    use crate::auth::user::account::modify::api::detail::test::MockModifyAuthUserAccountInfra;

    use crate::auth::user::account::modify::action::ModifyAuthUserAccountAction;

    impl ModifyAuthUserAccountAction<MockModifyAuthUserAccountInfra> {
        pub fn mock(infra: MockModifyAuthUserAccountInfra) -> Self {
            Self::new(infra)
        }
    }
}
