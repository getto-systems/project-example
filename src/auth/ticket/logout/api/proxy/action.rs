mod detail;

use std::sync::Arc;

use crate::{
    auth::ticket::logout::proxy::infra::{LogoutProxyInfra, LogoutProxyLogger},
    common::proxy::infra::ProxyCall,
};

use crate::{
    auth::ticket::{
        authenticate::data::CheckAuthenticateTokenSuccess, kernel::data::AuthenticateTokenExtract,
        logout::proxy::data::LogoutProxyError,
    },
    common::{api::request::data::RequestInfo, proxy::data::ProxyResponseBody},
};

pub struct LogoutProxyAction<M: LogoutProxyInfra> {
    logger: Arc<dyn LogoutProxyLogger>,
    infra: M,
}

impl<M: LogoutProxyInfra> LogoutProxyAction<M> {
    pub async fn logout(
        &self,
        info: RequestInfo,
        token: impl AuthenticateTokenExtract,
        auth: CheckAuthenticateTokenSuccess,
    ) -> Result<ProxyResponseBody, LogoutProxyError> {
        self.logger.try_to_logout();

        let token = token
            .convert()
            .map_err(|err| self.logger.invalid_request(err))?;

        let auth = self
            .infra
            .proxy_call()
            .call(info, token, auth)
            .await
            .map_err(|err| self.logger.proxy_error(err))?;

        Ok(self.logger.succeed_to_logout(auth))
    }
}
