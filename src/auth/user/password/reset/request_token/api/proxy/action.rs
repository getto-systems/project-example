mod detail;

use std::sync::Arc;

use crate::{
    auth::user::password::reset::request_token::proxy::infra::{
        RequestResetPasswordTokenProxyInfra, RequestResetPasswordTokenProxyLogger,
    },
    common::proxy::infra::ProxyCall,
};

use crate::{
    auth::proxy::data::AuthProxyCallError,
    common::{
        api::request::data::RequestInfo,
        proxy::data::{ProxyMetadataExtract, ProxyResponseBody},
    },
};

pub struct RequestResetPasswordTokenProxyAction<M: RequestResetPasswordTokenProxyInfra> {
    logger: Arc<dyn RequestResetPasswordTokenProxyLogger>,
    infra: M,
}

impl<M: RequestResetPasswordTokenProxyInfra> RequestResetPasswordTokenProxyAction<M> {
    pub async fn request(
        &self,
        info: RequestInfo,
        metadata: impl ProxyMetadataExtract,
        fields: String,
    ) -> Result<ProxyResponseBody, AuthProxyCallError> {
        self.logger.try_to_request_reset_token();

        let auth = self
            .infra
            .proxy_call()
            .call(info, metadata, fields)
            .await
            .map_err(|err| self.logger.proxy_error(err))?;

        Ok(self.logger.succeed_to_request_reset_token(auth))
    }
}
