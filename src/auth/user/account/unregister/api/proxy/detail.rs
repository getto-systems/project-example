use std::sync::Arc;

use actix_web::web::Data;
use prost::Message;
use tonic::Request;

use crate::auth::user::account::unregister::y_protobuf::service::{
    unregister_auth_user_account_pb_client::UnregisterAuthUserAccountPbClient,
    UnregisterAuthUserAccountRequestPb,
};

use crate::x_outside_feature::proxy::feature::ProxyAppFeature;

use crate::{
    auth::feature::AsCheckedInfra,
    common::api::{
        message::helper::{decode_base64, encode_protobuf_base64},
        service::detail::{authorizer::GoogleServiceAuthorizer, service::TonicService},
    },
};

use crate::auth::user::account::unregister::action::UnregisterAuthUserAccountInfo;

use crate::common::{
    api::service::infra::ServiceAuthorizer,
    proxy::infra::{ProxyCall, ProxyCallInfra},
};

use crate::{
    auth::{
        proxy::data::AuthProxyCallError,
        ticket::{
            authorize::data::CheckAuthorizeTokenSuccess, kernel::data::AuthPermissionRequired,
        },
    },
    common::{
        api::{message::data::MessageError, request::data::RequestInfo},
        proxy::data::{ProxyMetadataExtract, ProxyResponseBody},
    },
};

impl AsCheckedInfra<LiveUnregisterAuthUserAccountProxyInfra> for Data<ProxyAppFeature> {
    fn required(&self) -> AuthPermissionRequired {
        UnregisterAuthUserAccountInfo::required()
    }
    fn as_authorized_infra(
        &self,
        _: &CheckAuthorizeTokenSuccess,
    ) -> LiveUnregisterAuthUserAccountProxyInfra {
        LiveUnregisterAuthUserAccountProxyInfra {
            proxy_call: TonicUnregisterAuthUserAccountProxyCall {
                service_url: self.auth.service_url,
                authorizer: GoogleServiceAuthorizer::new(Arc::clone(
                    &self.auth.google_authorize_store,
                )),
            },
        }
    }
}

pub struct LiveUnregisterAuthUserAccountProxyInfra {
    proxy_call: TonicUnregisterAuthUserAccountProxyCall<GoogleServiceAuthorizer>,
}

impl ProxyCallInfra for LiveUnregisterAuthUserAccountProxyInfra {
    type ProxyCall = TonicUnregisterAuthUserAccountProxyCall<GoogleServiceAuthorizer>;

    fn proxy_call(&self) -> &Self::ProxyCall {
        &self.proxy_call
    }
}

pub struct TonicUnregisterAuthUserAccountProxyCall<A: ServiceAuthorizer> {
    service_url: &'static str,
    authorizer: A,
}

#[async_trait::async_trait]
impl<A: ServiceAuthorizer> ProxyCall for TonicUnregisterAuthUserAccountProxyCall<A> {
    type Request = String;
    type Response = ProxyResponseBody;
    type Error = AuthProxyCallError;

    async fn call(
        &self,
        info: RequestInfo,
        metadata: impl ProxyMetadataExtract,
        request: Self::Request,
    ) -> Result<Self::Response, Self::Error> {
        let mut request = Request::new(
            UnregisterAuthUserAccountRequestPb::decode(decode_base64(request)?)
                .map_err(Into::<MessageError>::into)?,
        );

        TonicService::set_metadata(
            &mut request,
            self.authorizer.fetch_token(&self.service_url).await?,
            info,
            metadata,
        )?;

        let response = UnregisterAuthUserAccountPbClient::new(
            TonicService::endpoint(&self.service_url).await?,
        )
        .unregister_user(request)
        .await?;

        Ok(ProxyResponseBody::restore(encode_protobuf_base64(
            response.into_inner(),
        )?))
    }
}
