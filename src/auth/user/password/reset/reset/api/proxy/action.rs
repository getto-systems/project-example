mod detail;

use std::sync::Arc;

use crate::{
    auth::user::password::reset::reset::proxy::infra::{
        ResetPasswordProxyInfra, ResetPasswordProxyLogger,
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

pub struct ResetPasswordProxyAction<M: ResetPasswordProxyInfra> {
    logger: Arc<dyn ResetPasswordProxyLogger>,
    infra: M,
}

impl<M: ResetPasswordProxyInfra> ResetPasswordProxyAction<M> {
    pub async fn request(
        &self,
        info: RequestInfo,
        metadata: impl ProxyMetadataExtract,
        fields: String,
    ) -> Result<ProxyResponseAuthenticated, AuthProxyCallError> {
        self.logger.try_to_reset_password();

        let auth = self
            .infra
            .proxy_call()
            .call(info, metadata, fields)
            .await
            .map_err(|err| self.logger.proxy_error(err))?;

        Ok(self.logger.succeed_to_reset_password(auth))
    }
}
