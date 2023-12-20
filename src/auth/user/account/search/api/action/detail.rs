use std::sync::Arc;

use crate::auth::user::account::search::api::detail::LiveSearchAuthUserAccountInfra;

use crate::auth::user::account::search::action::SearchAuthUserAccountAction;

use crate::auth::user::account::search::infra::{
    SearchAuthUserAccountInfra, SearchAuthUserAccountLogger,
};

use crate::{
    auth::user::account::search::data::AuthUserAccountSearch,
    common::api::repository::data::RepositoryError,
};

impl<M: SearchAuthUserAccountInfra> SearchAuthUserAccountAction<M> {
    fn new(infra: M) -> Self {
        Self {
            logger: Arc::new(NoopLogger),
            infra,
        }
    }

    pub fn with_logger(self, logger: Arc<dyn SearchAuthUserAccountLogger>) -> Self {
        Self { logger, ..self }
    }
}

impl SearchAuthUserAccountAction<LiveSearchAuthUserAccountInfra> {
    pub fn live(infra: LiveSearchAuthUserAccountInfra) -> Self {
        Self::new(infra)
    }
}

struct NoopLogger;

impl SearchAuthUserAccountLogger for NoopLogger {
    fn try_to_search_auth_user_account(&self) {}
    fn failed_to_search_user_account(&self, err: RepositoryError) -> RepositoryError {
        err
    }
    fn succeed_to_search_auth_user_account(
        &self,
        success: AuthUserAccountSearch,
    ) -> AuthUserAccountSearch {
        success
    }
}

#[cfg(test)]
mod test {
    use crate::auth::user::account::search::api::detail::test::MockSearchAuthUserAccountInfra;

    use crate::auth::user::account::search::action::SearchAuthUserAccountAction;

    impl SearchAuthUserAccountAction<MockSearchAuthUserAccountInfra> {
        pub fn mock(infra: MockSearchAuthUserAccountInfra) -> Self {
            Self::new(infra)
        }
    }
}
