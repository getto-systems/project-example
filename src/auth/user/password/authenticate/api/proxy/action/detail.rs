use std::sync::Arc;

use crate::auth::user::password::authenticate::proxy::detail::LiveAuthenticateWithPasswordProxyInfra;

use crate::auth::user::password::authenticate::proxy::action::AuthenticateWithPasswordProxyAction;

use crate::auth::user::password::authenticate::proxy::infra::{
    AuthenticateWithPasswordProxyInfra, AuthenticateWithPasswordProxyLogger,
};

use crate::auth::{
    proxy::data::AuthProxyCallError, ticket::authenticate::proxy::data::ProxyResponseAuthenticated,
};

impl<M: AuthenticateWithPasswordProxyInfra> AuthenticateWithPasswordProxyAction<M> {
    fn new(infra: M) -> Self {
        Self {
            logger: Arc::new(NoopLogger),
            infra,
        }
    }

    pub fn with_logger(self, logger: Arc<dyn AuthenticateWithPasswordProxyLogger>) -> Self {
        Self { logger, ..self }
    }
}

impl AuthenticateWithPasswordProxyAction<LiveAuthenticateWithPasswordProxyInfra> {
    pub fn live(infra: LiveAuthenticateWithPasswordProxyInfra) -> Self {
        Self::new(infra)
    }
}

struct NoopLogger;

impl AuthenticateWithPasswordProxyLogger for NoopLogger {
    fn try_to_authenticate_with_password(&self) {}
    fn proxy_error(&self, err: AuthProxyCallError) -> AuthProxyCallError {
        err
    }
    fn authenticated(&self, auth: ProxyResponseAuthenticated) -> ProxyResponseAuthenticated {
        auth
    }
}
