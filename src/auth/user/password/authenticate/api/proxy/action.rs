mod detail;

use std::sync::Arc;

use crate::{
    auth::user::password::authenticate::proxy::infra::{
        AuthenticateWithPasswordProxyInfra, AuthenticateWithPasswordProxyLogger,
    },
    common::proxy::infra::ProxyCall,
};

use crate::{
    auth::{
        proxy::data::AuthProxyCallError,
        ticket::authenticate::proxy::data::ProxyResponseAuthenticated,
    },
    common::{api::request::data::RequestInfo, proxy::data::ProxyMetadataExtract},
};

pub struct AuthenticateWithPasswordProxyAction<M: AuthenticateWithPasswordProxyInfra> {
    logger: Arc<dyn AuthenticateWithPasswordProxyLogger>,
    infra: M,
}

impl<M: AuthenticateWithPasswordProxyInfra> AuthenticateWithPasswordProxyAction<M> {
    pub async fn authenticate(
        &self,
        info: RequestInfo,
        metadata: impl ProxyMetadataExtract,
        fields: String,
    ) -> Result<ProxyResponseAuthenticated, AuthProxyCallError> {
        self.logger.try_to_authenticate_with_password();

        let auth = self
            .infra
            .proxy_call()
            .call(info, metadata, fields)
            .await
            .map_err(|err| self.logger.proxy_error(err))?;

        Ok(self.logger.authenticated(auth))
    }
}
