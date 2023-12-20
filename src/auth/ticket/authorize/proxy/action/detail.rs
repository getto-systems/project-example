use std::sync::Arc;

use crate::auth::{
    kernel::feature::AsAuthorizedInfra, ticket::authorize::proxy::detail::LiveAuthorizeProxyInfra,
};

use crate::auth::ticket::authorize::proxy::action::AuthorizeProxyAction;

use crate::auth::ticket::authorize::proxy::infra::{AuthorizeProxyInfra, AuthorizeProxyLogger};

use crate::{
    auth::{
        proxy::data::AuthProxyCallError,
        ticket::{
            authorize::{data::AuthorizeSuccess, proxy::data::AuthorizeProxyError},
            kernel::data::{
                AuthPermissionError, AuthorizeTokenExtract, DecodeAuthorizeTokenError,
                ValidateAuthorizeTokenError,
            },
        },
    },
    common::api::request::data::RequestInfo,
};

impl<M: AuthorizeProxyInfra> AuthorizeProxyAction<M> {
    pub async fn pick_authorized_infra<T>(
        &self,
        app: &impl AsAuthorizedInfra<T>,
        info: RequestInfo,
        token: impl AuthorizeTokenExtract,
    ) -> Result<T, AuthorizeProxyError> {
        let auth = self.authorize(info, token, app.required()).await?;
        Ok(app.as_authorized_infra(&auth))
    }

    fn new(infra: M) -> Self {
        Self {
            logger: Arc::new(NoopLogger),
            infra,
        }
    }

    pub fn with_logger(self, logger: Arc<dyn AuthorizeProxyLogger>) -> Self {
        Self { logger, ..self }
    }
}

impl AuthorizeProxyAction<LiveAuthorizeProxyInfra> {
    pub fn live(infra: LiveAuthorizeProxyInfra) -> Self {
        Self::new(infra)
    }
}

struct NoopLogger;

impl AuthorizeProxyLogger for NoopLogger {
    fn try_to_authorize(&self) {}
    fn invalid_request(&self, err: ValidateAuthorizeTokenError) -> ValidateAuthorizeTokenError {
        err
    }
    fn invalid_token(&self, err: DecodeAuthorizeTokenError) -> DecodeAuthorizeTokenError {
        err
    }
    fn forbidden(&self, err: AuthPermissionError) -> AuthPermissionError {
        err
    }
    fn proxy_error(&self, err: AuthProxyCallError) -> AuthProxyCallError {
        err
    }
    fn authorized(&self, auth: AuthorizeSuccess) -> AuthorizeSuccess {
        auth
    }
}

#[cfg(test)]
mod test {
    use crate::auth::ticket::authorize::proxy::detail::test::MockAuthorizeProxyInfra;

    impl super::AuthorizeProxyAction<MockAuthorizeProxyInfra> {
        pub fn mock(infra: MockAuthorizeProxyInfra) -> Self {
            Self::new(infra)
        }
    }
}
