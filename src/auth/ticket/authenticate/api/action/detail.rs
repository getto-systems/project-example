use std::sync::Arc;

use crate::auth::ticket::authenticate::api::detail::LiveCheckAuthenticateTokenInfra;

use crate::auth::ticket::authenticate::action::CheckAuthenticateTokenAction;

use crate::auth::ticket::authenticate::infra::{
    CheckAuthenticateTokenInfra, CheckAuthenticateTokenLogger,
};

use crate::auth::ticket::{
    authenticate::data::CheckAuthenticateTokenSuccess,
    kernel::data::{DecodeAuthenticateTokenError, ValidateAuthenticateTokenError},
};

impl<M: CheckAuthenticateTokenInfra> CheckAuthenticateTokenAction<M> {
    fn new(infra: M) -> Self {
        Self {
            logger: Arc::new(NoopLogger),
            infra,
        }
    }

    pub fn with_logger(self, logger: Arc<dyn CheckAuthenticateTokenLogger>) -> Self {
        Self { logger, ..self }
    }
}

impl CheckAuthenticateTokenAction<LiveCheckAuthenticateTokenInfra> {
    pub fn live(infra: LiveCheckAuthenticateTokenInfra) -> Self {
        Self::new(infra)
    }
}

struct NoopLogger;

impl CheckAuthenticateTokenLogger for NoopLogger {
    fn try_to_check_authenticate_token(&self) {}
    fn invalid_request(
        &self,
        err: ValidateAuthenticateTokenError,
    ) -> ValidateAuthenticateTokenError {
        err
    }
    fn invalid_token(&self, err: DecodeAuthenticateTokenError) -> DecodeAuthenticateTokenError {
        err
    }
    fn succeed_to_check_authenticate_token(
        &self,
        auth: CheckAuthenticateTokenSuccess,
    ) -> CheckAuthenticateTokenSuccess {
        auth
    }
}

#[cfg(test)]
mod test {
    use crate::auth::ticket::authenticate::api::detail::test::MockCheckAuthenticateTokenInfra;

    use crate::auth::ticket::authenticate::action::CheckAuthenticateTokenAction;

    impl CheckAuthenticateTokenAction<MockCheckAuthenticateTokenInfra> {
        pub fn mock(infra: MockCheckAuthenticateTokenInfra) -> Self {
            Self::new(infra)
        }
    }
}
