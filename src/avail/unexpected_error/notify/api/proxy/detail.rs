use std::sync::Arc;

use actix_web::web::Data;
use prost::Message;
use tonic::Request;

use crate::avail::unexpected_error::notify::y_protobuf::service::{
    notify_pb_client::NotifyPbClient, NotifyRequestPb,
};

use crate::x_outside_feature::proxy::feature::ProxyAppFeature;

use crate::{
    auth::feature::AsCheckedInfra,
    common::api::{
        message::helper::{decode_base64, encode_protobuf_base64},
        service::detail::{authorizer::GoogleServiceAuthorizer, service::TonicService},
    },
};

use crate::avail::unexpected_error::notify::action::NotifyUnexpectedErrorInfo;

use crate::common::{
    api::service::infra::ServiceAuthorizer,
    proxy::infra::{ProxyCall, ProxyCallInfra},
};

use crate::{
    auth::data::{AuthPermissionRequired, CheckAuthorizeTokenSuccess},
    common::{
        api::{message::data::MessageError, request::data::RequestInfo},
        proxy::data::{CoreProxyCallError, ProxyMetadataExtract, ProxyResponseBody},
    },
};

impl AsCheckedInfra<LiveNotifyUnexpectedErrorProxyInfra> for Data<ProxyAppFeature> {
    fn required(&self) -> AuthPermissionRequired {
        NotifyUnexpectedErrorInfo::required()
    }
    fn as_authorized_infra(
        &self,
        _: &CheckAuthorizeTokenSuccess,
    ) -> LiveNotifyUnexpectedErrorProxyInfra {
        LiveNotifyUnexpectedErrorProxyInfra {
            proxy_call: TonicNotifyUnexpectedErrorProxyCall {
                service_url: self.core.service_url,
                authorizer: GoogleServiceAuthorizer::new(Arc::clone(
                    &self.core.google_authorize_store,
                )),
            },
        }
    }
}

pub struct LiveNotifyUnexpectedErrorProxyInfra {
    proxy_call: TonicNotifyUnexpectedErrorProxyCall<GoogleServiceAuthorizer>,
}

impl ProxyCallInfra for LiveNotifyUnexpectedErrorProxyInfra {
    type ProxyCall = TonicNotifyUnexpectedErrorProxyCall<GoogleServiceAuthorizer>;

    fn proxy_call(&self) -> &Self::ProxyCall {
        &self.proxy_call
    }
}

pub struct TonicNotifyUnexpectedErrorProxyCall<A: ServiceAuthorizer> {
    service_url: &'static str,
    authorizer: A,
}

#[async_trait::async_trait]
impl<A: ServiceAuthorizer> ProxyCall for TonicNotifyUnexpectedErrorProxyCall<A> {
    type Request = String;
    type Response = ProxyResponseBody;
    type Error = CoreProxyCallError;

    async fn call(
        &self,
        info: RequestInfo,
        metadata: impl ProxyMetadataExtract,
        request: Self::Request,
    ) -> Result<Self::Response, Self::Error> {
        let mut request = Request::new(
            NotifyRequestPb::decode(decode_base64(request)?).map_err(Into::<MessageError>::into)?,
        );

        TonicService::set_metadata(
            &mut request,
            self.authorizer.fetch_token(&self.service_url).await?,
            info,
            metadata,
        )?;

        let response = NotifyPbClient::new(TonicService::endpoint(&self.service_url).await?)
            .notify(request)
            .await?;

        Ok(ProxyResponseBody::restore(encode_protobuf_base64(
            response.into_inner(),
        )?))
    }
}
