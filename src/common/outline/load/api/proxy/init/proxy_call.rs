use tonic::Request;

use crate::common::outline::load::y_protobuf::service::{
    load_menu_badge_pb_client::LoadMenuBadgePbClient, LoadMenuBadgeRequestPb,
};

use crate::{
    common::x_outside_feature::feature::CoreProxyOutsideFeature, x_outside_feature::data::RequestId,
};

use crate::common::api::message::helper::encode_protobuf_base64;

use crate::common::api::service::init::service::GoogleTonicService;

use crate::common::proxy::infra::ProxyCall;

use crate::common::proxy::data::{CoreProxyError, ProxyMetadataExtract, ProxyResponseBody};

pub struct TonicGetOutlineMenuBadgeProxyCall<'a> {
    service: GoogleTonicService<'a>,
}

impl<'a> TonicGetOutlineMenuBadgeProxyCall<'a> {
    pub fn new(feature: &'a CoreProxyOutsideFeature, request_id: RequestId) -> Self {
        Self {
            service: GoogleTonicService::new(&feature.service, request_id),
        }
    }
}

#[async_trait::async_trait]
impl<'a> ProxyCall for TonicGetOutlineMenuBadgeProxyCall<'a> {
    type Request = ();
    type Response = ProxyResponseBody;
    type Error = CoreProxyError;

    async fn call(
        &self,
        metadata: impl ProxyMetadataExtract,
        _request: Self::Request,
    ) -> Result<Self::Response, Self::Error> {
        let mut client = LoadMenuBadgePbClient::new(
            self.service
                .endpoint()
                .await
                .map_err(CoreProxyError::ServiceConnectError)?,
        );

        let mut request = Request::new(LoadMenuBadgeRequestPb {});
        self.service
            .set_metadata(&mut request, metadata)
            .await
            .map_err(CoreProxyError::ServiceMetadataError)?;

        let response = client
            .load_menu_badge(request)
            .await
            .map_err(CoreProxyError::from)?;

        let body =
            encode_protobuf_base64(response.into_inner()).map_err(CoreProxyError::MessageError)?;

        Ok(ProxyResponseBody::restore(body))
    }
}
