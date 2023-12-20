mod detail;

use std::sync::Arc;

use crate::{
    auth::ticket::authorize::{
        infra::AuthorizeTokenDecoder,
        proxy::infra::{AuthorizeProxyInfra, AuthorizeProxyLogger},
    },
    common::proxy::infra::ProxyCall,
};

use crate::{
    auth::ticket::{
        authorize::{data::AuthorizeSuccess, proxy::data::AuthorizeProxyError},
        kernel::data::{AuthPermissionRequired, AuthorizeTokenExtract},
    },
    common::api::request::data::RequestInfo,
};

pub struct AuthorizeProxyAction<M: AuthorizeProxyInfra> {
    logger: Arc<dyn AuthorizeProxyLogger>,
    infra: M,
}

impl<M: AuthorizeProxyInfra> AuthorizeProxyAction<M> {
    pub async fn authorize(
        &self,
        info: RequestInfo,
        token: impl AuthorizeTokenExtract,
        required: AuthPermissionRequired,
    ) -> Result<AuthorizeSuccess, AuthorizeProxyError> {
        self.logger.try_to_authorize();

        let token = token
            .convert()
            .map_err(|err| self.logger.invalid_request(err))?;

        let ticket = self
            .infra
            .token_decoder()
            .decode(token.clone())
            .map_err(|err| self.logger.invalid_token(err))?;

        ticket
            .attrs
            .granted
            .has_enough_permission(&required)
            .map_err(|err| self.logger.forbidden(err))?;

        let auth = self
            .infra
            .proxy_call()
            .call(info, token, required)
            .await
            .map_err(|err| self.logger.proxy_error(err))?;

        Ok(self.logger.authorized(auth))
    }
}
