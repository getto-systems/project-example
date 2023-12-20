use std::sync::Arc;

use crate::auth::user::password::reset::reset::proxy::detail::LiveResetPasswordProxyInfra;

use crate::auth::user::password::reset::reset::proxy::action::ResetPasswordProxyAction;

use crate::auth::user::password::reset::reset::proxy::infra::{
    ResetPasswordProxyInfra, ResetPasswordProxyLogger,
};

use crate::auth::{
    proxy::data::AuthProxyCallError, ticket::authenticate::proxy::data::ProxyResponseAuthenticated,
};

impl<M: ResetPasswordProxyInfra> ResetPasswordProxyAction<M> {
    fn new(infra: M) -> Self {
        Self {
            logger: Arc::new(NoopLogger),
            infra,
        }
    }

    pub fn with_logger(self, logger: Arc<dyn ResetPasswordProxyLogger>) -> Self {
        Self { logger, ..self }
    }
}

impl ResetPasswordProxyAction<LiveResetPasswordProxyInfra> {
    pub fn live(infra: LiveResetPasswordProxyInfra) -> Self {
        Self::new(infra)
    }
}

struct NoopLogger;

impl ResetPasswordProxyLogger for NoopLogger {
    fn try_to_reset_password(&self) {}
    fn proxy_error(&self, err: AuthProxyCallError) -> AuthProxyCallError {
        err
    }
    fn succeed_to_reset_password(
        &self,
        response: ProxyResponseAuthenticated,
    ) -> ProxyResponseAuthenticated {
        response
    }
}
