use std::sync::Arc;

use crate::common::proxy::infra::{ProxyCall, ProxyCallInfra, ProxyCallLogger};

use crate::{
    auth::data::{AuthorizeTokenExtract, ValidateAuthorizeTokenError},
    common::api::request::data::RequestInfo,
};

pub struct ProxyCallAction<M: ProxyCallInfra> {
    logger: Arc<
        dyn ProxyCallLogger<
            <<M as ProxyCallInfra>::ProxyCall as ProxyCall>::Response,
            <<M as ProxyCallInfra>::ProxyCall as ProxyCall>::Error,
        >,
    >,
    infra: M,
}

impl<M: ProxyCallInfra> ProxyCallAction<M> {
    pub fn new(infra: M) -> Self {
        Self {
            logger: Arc::new(NoopLogger),
            infra,
        }
    }

    pub fn with_logger(
        self,
        logger: Arc<
            dyn ProxyCallLogger<
                <<M as ProxyCallInfra>::ProxyCall as ProxyCall>::Response,
                <<M as ProxyCallInfra>::ProxyCall as ProxyCall>::Error,
            >,
        >,
    ) -> Self {
        Self { logger, ..self }
    }
}

impl<M: ProxyCallInfra> ProxyCallAction<M> {
    pub async fn call(
        &self,
        info: RequestInfo,
        token: impl AuthorizeTokenExtract,
        request: <<M as ProxyCallInfra>::ProxyCall as ProxyCall>::Request,
    ) -> Result<
        <<M as ProxyCallInfra>::ProxyCall as ProxyCall>::Response,
        <<M as ProxyCallInfra>::ProxyCall as ProxyCall>::Error,
    > {
        self.logger.try_to_proxy_call();

        let token = token
            .convert()
            .map_err(|err| self.logger.invalid_authorize_token(err))?;

        let response = self
            .infra
            .proxy_call()
            .call(info, token, request)
            .await
            .map_err(|err| self.logger.failed_to_proxy_call(err))?;

        Ok(self.logger.succeed_to_proxy_call(response))
    }
}

struct NoopLogger;

impl<R, E> ProxyCallLogger<R, E> for NoopLogger {
    fn try_to_proxy_call(&self) {}
    fn invalid_authorize_token(
        &self,
        err: ValidateAuthorizeTokenError,
    ) -> ValidateAuthorizeTokenError {
        err
    }
    fn failed_to_proxy_call(&self, err: E) -> E {
        err
    }
    fn succeed_to_proxy_call(&self, response: R) -> R {
        response
    }
}
