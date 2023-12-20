use std::sync::Arc;

use crate::auth::ticket::encode::detail::LiveEncodeAuthTokenInfra;

use crate::auth::ticket::encode::action::EncodeAuthTokenAction;

use crate::auth::ticket::encode::infra::{EncodeAuthTokenInfra, EncodeAuthTokenLogger};

use crate::{
    auth::ticket::encode::data::{
        AuthTokenExpires, EncodeAuthTokenError, EncodeAuthTokenSuccess, EncodeTokenError,
    },
    common::api::repository::data::RepositoryError,
};

impl<M: EncodeAuthTokenInfra> EncodeAuthTokenAction<M> {
    fn new(infra: M) -> Self {
        Self {
            logger: Arc::new(NoopLogger),
            infra,
        }
    }

    pub fn with_logger(self, logger: Arc<dyn EncodeAuthTokenLogger>) -> Self {
        Self { logger, ..self }
    }
}

impl EncodeAuthTokenAction<LiveEncodeAuthTokenInfra> {
    pub fn live(infra: LiveEncodeAuthTokenInfra) -> Self {
        Self::new(infra)
    }
}

struct NoopLogger;

impl EncodeAuthTokenLogger for NoopLogger {
    fn try_to_encode_auth_token(&self) {}
    fn calculate_token_expires(&self, expires: AuthTokenExpires) -> AuthTokenExpires {
        expires
    }
    fn failed_to_lookup_expansion_limit(&self, err: RepositoryError) -> RepositoryError {
        err
    }
    fn expansion_limit_not_found(&self, err: EncodeAuthTokenError) -> EncodeAuthTokenError {
        err
    }
    fn failed_to_encode_token(&self, err: EncodeTokenError) -> EncodeTokenError {
        err
    }
    fn succeed_to_encode_auth_token(&self, auth: EncodeAuthTokenSuccess) -> EncodeAuthTokenSuccess {
        auth
    }
}

#[cfg(test)]
mod test {
    use crate::auth::ticket::encode::detail::test::MockEncodeAuthTokenInfra;

    use crate::auth::ticket::encode::action::EncodeAuthTokenAction;

    impl EncodeAuthTokenAction<MockEncodeAuthTokenInfra> {
        pub fn mock(infra: MockEncodeAuthTokenInfra) -> Self {
            Self::new(infra)
        }
    }
}
