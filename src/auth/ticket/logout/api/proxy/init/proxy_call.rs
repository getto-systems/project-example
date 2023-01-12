use tonic::Request;

use crate::auth::ticket::logout::y_protobuf::service::{
    logout_pb_client::LogoutPbClient, LogoutRequestPb,
};

use crate::{
    auth::x_outside_feature::feature::AuthProxyOutsideFeature, x_outside_feature::data::RequestId,
};

use crate::common::api::message::helper::encode_protobuf_base64;

use crate::common::api::service::init::service::GoogleTonicService;

use crate::common::proxy::infra::ProxyCall;

use crate::{
    auth::proxy::data::AuthProxyError,
    common::proxy::data::{ProxyMetadataExtract, ProxyResponseBody},
};

pub struct TonicLogoutProxyCall<'a> {
    service: GoogleTonicService<'a>,
}

impl<'a> TonicLogoutProxyCall<'a> {
    pub fn new(feature: &'a AuthProxyOutsideFeature, request_id: RequestId) -> Self {
        Self {
            service: GoogleTonicService::new(&feature.service, request_id),
        }
    }
}

#[async_trait::async_trait]
impl<'a> ProxyCall for TonicLogoutProxyCall<'a> {
    type Request = ();
    type Response = ProxyResponseBody;
    type Error = AuthProxyError;

    async fn call(
        &self,
        metadata: impl ProxyMetadataExtract,
        _request: Self::Request,
    ) -> Result<Self::Response, Self::Error> {
        let mut client = LogoutPbClient::new(
            self.service
                .endpoint()
                .await
                .map_err(AuthProxyError::ServiceConnectError)?,
        );

        let mut request = Request::new(LogoutRequestPb {});
        self.service
            .set_metadata(&mut request, metadata)
            .await
            .map_err(AuthProxyError::ServiceMetadataError)?;

        let response = client.logout(request).await.map_err(AuthProxyError::from)?;

        let body =
            encode_protobuf_base64(response.into_inner()).map_err(AuthProxyError::MessageError)?;

        Ok(ProxyResponseBody::restore(body))
    }
}

#[cfg(test)]
pub mod test {
    use crate::common::proxy::infra::ProxyCall;

    use crate::{
        auth::proxy::data::AuthProxyError,
        common::proxy::data::{ProxyMetadataExtract, ProxyResponseBody},
    };

    pub struct StaticLogoutProxyCall;

    #[async_trait::async_trait]
    impl ProxyCall for StaticLogoutProxyCall {
        type Request = ();
        type Response = ProxyResponseBody;
        type Error = AuthProxyError;

        async fn call(
            &self,
            _metadata: impl ProxyMetadataExtract,
            _request: Self::Request,
        ) -> Result<Self::Response, Self::Error> {
            Ok(ProxyResponseBody::restore("RESPONSE".to_owned()))
        }
    }
}
