use std::sync::Arc;

use crate::auth::ticket::authenticate::proxy::detail::LiveAuthenticateWithTokenProxyInfra;

use crate::auth::ticket::authenticate::proxy::action::AuthenticateWithTokenProxyAction;

use crate::auth::ticket::authenticate::proxy::infra::{
    AuthenticateWithTokenProxyInfra, AuthenticateWithTokenProxyLogger,
};

use crate::auth::{
    proxy::data::AuthProxyCallError,
    ticket::{
        authenticate::proxy::data::ProxyResponseAuthenticated,
        kernel::data::ValidateAuthenticateTokenError,
    },
};

impl<M: AuthenticateWithTokenProxyInfra> AuthenticateWithTokenProxyAction<M> {
    fn new(infra: M) -> Self {
        Self {
            logger: Arc::new(NoopLogger),
            infra,
        }
    }

    pub fn with_logger(self, logger: Arc<dyn AuthenticateWithTokenProxyLogger>) -> Self {
        Self { logger, ..self }
    }
}

impl AuthenticateWithTokenProxyAction<LiveAuthenticateWithTokenProxyInfra> {
    pub fn live(infra: LiveAuthenticateWithTokenProxyInfra) -> Self {
        Self::new(infra)
    }
}

struct NoopLogger;

impl AuthenticateWithTokenProxyLogger for NoopLogger {
    fn try_to_authenticate_with_token(&self) {}
    fn invalid_request(
        &self,
        err: ValidateAuthenticateTokenError,
    ) -> ValidateAuthenticateTokenError {
        err
    }
    fn proxy_error(&self, err: AuthProxyCallError) -> AuthProxyCallError {
        err
    }
    fn authenticated(&self, auth: ProxyResponseAuthenticated) -> ProxyResponseAuthenticated {
        auth
    }
}
