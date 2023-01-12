use prost::Message;
use tonic::Request;

use crate::auth::user::password::reset::request_token::y_protobuf::service::{
    request_reset_token_pb_client::RequestResetTokenPbClient, RequestResetTokenRequestPb,
};

use crate::{
    auth::x_outside_feature::feature::AuthProxyOutsideFeature, x_outside_feature::data::RequestId,
};

use crate::common::api::message::helper::{
    decode_base64, encode_protobuf_base64, invalid_protobuf,
};

use crate::common::api::service::init::service::GoogleTonicService;

use crate::common::proxy::infra::ProxyCall;

use crate::{
    auth::proxy::data::AuthProxyError,
    common::api::message::data::MessageError,
    common::proxy::data::{ProxyMetadataExtract, ProxyResponseBody},
};

pub struct TonicRequestResetTokenProxyCall<'a> {
    service: GoogleTonicService<'a>,
}

impl<'a> TonicRequestResetTokenProxyCall<'a> {
    pub fn new(feature: &'a AuthProxyOutsideFeature, request_id: RequestId) -> Self {
        Self {
            service: GoogleTonicService::new(&feature.service, request_id),
        }
    }
}

#[async_trait::async_trait]
impl<'a> ProxyCall for TonicRequestResetTokenProxyCall<'a> {
    type Request = String;
    type Response = ProxyResponseBody;
    type Error = AuthProxyError;

    async fn call(
        &self,
        metadata: impl ProxyMetadataExtract,
        request: Self::Request,
    ) -> Result<Self::Response, Self::Error> {
        let mut client = RequestResetTokenPbClient::new(
            self.service
                .endpoint()
                .await
                .map_err(AuthProxyError::ServiceConnectError)?,
        );

        let mut request =
            Request::new(decode_request(request).map_err(AuthProxyError::MessageError)?);
        self.service
            .set_metadata(&mut request, metadata)
            .await
            .map_err(AuthProxyError::ServiceMetadataError)?;

        let response = client
            .request_token(request)
            .await
            .map_err(AuthProxyError::from)?;

        let body =
            encode_protobuf_base64(response.into_inner()).map_err(AuthProxyError::MessageError)?;

        Ok(ProxyResponseBody::restore(body))
    }
}

fn decode_request(request: String) -> Result<RequestResetTokenRequestPb, MessageError> {
    RequestResetTokenRequestPb::decode(decode_base64(request)?).map_err(invalid_protobuf)
}

#[cfg(test)]
pub mod test {
    use crate::common::proxy::infra::ProxyCall;

    use crate::{
        auth::proxy::data::AuthProxyError,
        common::proxy::data::{ProxyMetadataExtract, ProxyResponseBody},
    };

    pub struct StaticRequestResetTokenProxyCall;

    #[async_trait::async_trait]
    impl ProxyCall for StaticRequestResetTokenProxyCall {
        type Request = String;
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
