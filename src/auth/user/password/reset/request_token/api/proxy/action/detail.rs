use std::sync::Arc;

use crate::{
    auth::{
        proxy::data::AuthProxyCallError,
        user::password::reset::request_token::proxy::{
            action::RequestResetPasswordTokenProxyAction,
            detail::LiveRequestResetPasswordTokenProxyInfra,
            infra::{RequestResetPasswordTokenProxyInfra, RequestResetPasswordTokenProxyLogger},
        },
    },
    common::proxy::data::ProxyResponseBody,
};

impl<M: RequestResetPasswordTokenProxyInfra> RequestResetPasswordTokenProxyAction<M> {
    fn new(infra: M) -> Self {
        Self {
            logger: Arc::new(NoopLogger),
            infra,
        }
    }

    pub fn with_logger(self, logger: Arc<dyn RequestResetPasswordTokenProxyLogger>) -> Self {
        Self { logger, ..self }
    }
}

impl RequestResetPasswordTokenProxyAction<LiveRequestResetPasswordTokenProxyInfra> {
    pub fn live(infra: LiveRequestResetPasswordTokenProxyInfra) -> Self {
        Self::new(infra)
    }
}

struct NoopLogger;

impl RequestResetPasswordTokenProxyLogger for NoopLogger {
    fn try_to_request_reset_token(&self) {}
    fn proxy_error(&self, err: AuthProxyCallError) -> AuthProxyCallError {
        err
    }
    fn succeed_to_request_reset_token(&self, response: ProxyResponseBody) -> ProxyResponseBody {
        response
    }
}
