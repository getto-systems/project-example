use prost::Message;
use tonic::Request;

use crate::auth::user::account::search::y_protobuf::service::{
    search_auth_user_account_pb_client::SearchAuthUserAccountPbClient,
    SearchAuthUserAccountRequestPb,
};

use crate::{
    common::x_outside_feature::feature::CoreProxyOutsideFeature, x_outside_feature::data::RequestId,
};

use crate::common::api::message::helper::{
    decode_base64, encode_protobuf_base64, invalid_protobuf,
};

use crate::common::api::service::init::service::GoogleTonicService;

use crate::common::proxy::infra::ProxyCall;

use crate::{
    common::api::message::data::MessageError,
    common::proxy::data::{CoreProxyError, ProxyMetadataExtract, ProxyResponseBody},
};

pub struct TonicSearchAuthUserAccountProxyCall<'a> {
    service: GoogleTonicService<'a>,
}

impl<'a> TonicSearchAuthUserAccountProxyCall<'a> {
    pub fn new(feature: &'a CoreProxyOutsideFeature, request_id: RequestId) -> Self {
        Self {
            service: GoogleTonicService::new(&feature.service, request_id),
        }
    }
}

#[async_trait::async_trait]
impl<'a> ProxyCall for TonicSearchAuthUserAccountProxyCall<'a> {
    type Request = String;
    type Response = ProxyResponseBody;
    type Error = CoreProxyError;

    async fn call(
        &self,
        metadata: impl ProxyMetadataExtract,
        request: Self::Request,
    ) -> Result<Self::Response, Self::Error> {
        let mut client = SearchAuthUserAccountPbClient::new(
            self.service
                .endpoint()
                .await
                .map_err(CoreProxyError::ServiceConnectError)?,
        );

        let mut request =
            Request::new(decode_request(request).map_err(CoreProxyError::MessageError)?);
        self.service
            .set_metadata(&mut request, metadata)
            .await
            .map_err(CoreProxyError::ServiceMetadataError)?;

        let response = client.search(request).await.map_err(CoreProxyError::from)?;

        let body =
            encode_protobuf_base64(response.into_inner()).map_err(CoreProxyError::MessageError)?;

        Ok(ProxyResponseBody::restore(body))
    }
}

fn decode_request(body: String) -> Result<SearchAuthUserAccountRequestPb, MessageError> {
    SearchAuthUserAccountRequestPb::decode(decode_base64(body)?).map_err(invalid_protobuf)
}
