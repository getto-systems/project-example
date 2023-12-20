use std::sync::Arc;

use crate::auth::ticket::logout::proxy::detail::LiveLogoutProxyInfra;

use crate::auth::ticket::logout::proxy::action::LogoutProxyAction;

use crate::auth::ticket::logout::proxy::infra::{LogoutProxyInfra, LogoutProxyLogger};

use crate::{
    auth::{proxy::data::AuthProxyCallError, ticket::kernel::data::ValidateAuthenticateTokenError},
    common::proxy::data::ProxyResponseBody,
};

impl<M: LogoutProxyInfra> LogoutProxyAction<M> {
    fn new(infra: M) -> Self {
        Self {
            logger: Arc::new(NoopLogger),
            infra,
        }
    }

    pub fn with_logger(self, logger: Arc<dyn LogoutProxyLogger>) -> Self {
        Self { logger, ..self }
    }
}

impl LogoutProxyAction<LiveLogoutProxyInfra> {
    pub fn live(infra: LiveLogoutProxyInfra) -> Self {
        Self::new(infra)
    }
}

struct NoopLogger;

impl LogoutProxyLogger for NoopLogger {
    fn try_to_logout(&self) {}
    fn invalid_request(
        &self,
        err: ValidateAuthenticateTokenError,
    ) -> ValidateAuthenticateTokenError {
        err
    }
    fn proxy_error(&self, err: AuthProxyCallError) -> AuthProxyCallError {
        err
    }
    fn succeed_to_logout(&self, auth: ProxyResponseBody) -> ProxyResponseBody {
        auth
    }
}
