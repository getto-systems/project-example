use std::sync::Arc;

use actix_web::web::Data;
use prost::Message;
use tonic::Request;

use crate::auth::user::password::change::y_protobuf::service::{
    overwrite_password_pb_client::OverwritePasswordPbClient, OverwritePasswordRequestPb,
};

use crate::x_outside_feature::proxy::feature::ProxyAppFeature;

use crate::{
    auth::feature::AsCheckedInfra,
    common::api::{
        message::helper::{decode_base64, encode_protobuf_base64},
        service::detail::{authorizer::GoogleServiceAuthorizer, service::TonicService},
    },
};

use crate::auth::user::password::change::action::OverwritePasswordInfo;

use crate::common::{
    api::service::infra::ServiceAuthorizer,
    proxy::infra::{ProxyCall, ProxyCallInfra},
};

use crate::{
    auth::ticket::{
        authorize::data::CheckAuthorizeTokenSuccess, kernel::data::AuthPermissionRequired,
    },
    common::{
        api::{message::data::MessageError, request::data::RequestInfo},
        proxy::data::{CoreProxyCallError, ProxyMetadataExtract, ProxyResponseBody},
    },
};

impl AsCheckedInfra<LiveOverwritePasswordProxyInfra> for Data<ProxyAppFeature> {
    fn required(&self) -> AuthPermissionRequired {
        OverwritePasswordInfo::required()
    }
    fn as_authorized_infra(
        &self,
        _: &CheckAuthorizeTokenSuccess,
    ) -> LiveOverwritePasswordProxyInfra {
        LiveOverwritePasswordProxyInfra {
            proxy_call: TonicOverwritePasswordProxyCall {
                service_url: self.auth.service_url,
                authorizer: GoogleServiceAuthorizer::new(Arc::clone(
                    &self.auth.google_authorize_store,
                )),
            },
        }
    }
}

pub struct LiveOverwritePasswordProxyInfra {
    proxy_call: TonicOverwritePasswordProxyCall<GoogleServiceAuthorizer>,
}

impl ProxyCallInfra for LiveOverwritePasswordProxyInfra {
    type ProxyCall = TonicOverwritePasswordProxyCall<GoogleServiceAuthorizer>;

    fn proxy_call(&self) -> &Self::ProxyCall {
        &self.proxy_call
    }
}

pub struct TonicOverwritePasswordProxyCall<A: ServiceAuthorizer> {
    service_url: &'static str,
    authorizer: A,
}

#[async_trait::async_trait]
impl<A: ServiceAuthorizer> ProxyCall for TonicOverwritePasswordProxyCall<A> {
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
            OverwritePasswordRequestPb::decode(decode_base64(request)?)
                .map_err(Into::<MessageError>::into)?,
        );

        TonicService::set_metadata(
            &mut request,
            self.authorizer.fetch_token(&self.service_url).await?,
            info,
            metadata,
        )?;

        let response =
            OverwritePasswordPbClient::new(TonicService::endpoint(&self.service_url).await?)
                .overwrite_password(request)
                .await?;

        Ok(ProxyResponseBody::restore(encode_protobuf_base64(
            response.into_inner(),
        )?))
    }
}
