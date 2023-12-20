mod detail;

use std::sync::Arc;

use crate::{
    auth::ticket::authenticate::proxy::infra::{
        AuthenticateWithTokenProxyInfra, AuthenticateWithTokenProxyLogger,
    },
    common::proxy::infra::ProxyCall,
};

use crate::{
    auth::ticket::{
        authenticate::{
            data::CheckAuthenticateTokenSuccess,
            proxy::data::{AuthenticateWithTokenProxyError, ProxyResponseAuthenticated},
        },
        kernel::data::AuthenticateTokenExtract,
    },
    common::api::request::data::RequestInfo,
};

pub struct AuthenticateWithTokenProxyAction<M: AuthenticateWithTokenProxyInfra> {
    logger: Arc<dyn AuthenticateWithTokenProxyLogger>,
    infra: M,
}

impl<M: AuthenticateWithTokenProxyInfra> AuthenticateWithTokenProxyAction<M> {
    pub async fn authenticate(
        &self,
        info: RequestInfo,
        token: impl AuthenticateTokenExtract,
        auth: CheckAuthenticateTokenSuccess,
    ) -> Result<ProxyResponseAuthenticated, AuthenticateWithTokenProxyError> {
        self.logger.try_to_authenticate_with_token();

        let token = token
            .convert()
            .map_err(|err| self.logger.invalid_request(err))?;

        let auth = self
            .infra
            .proxy_call()
            .call(info, token, auth)
            .await
            .map_err(|err| self.logger.proxy_error(err))?;

        Ok(self.logger.authenticated(auth))
    }
}
