use std::sync::Arc;

use actix_web::web::Data;
use prost::Message;
use tonic::Request;

use crate::auth::user::account::search::y_protobuf::service::{
    search_auth_user_account_pb_client::SearchAuthUserAccountPbClient,
    SearchAuthUserAccountRequestPb,
};

use crate::x_outside_feature::proxy::feature::ProxyAppFeature;

use crate::{
    auth::feature::AsCheckedInfra,
    common::api::{
        message::helper::{decode_base64, encode_protobuf_base64},
        service::detail::{authorizer::GoogleServiceAuthorizer, service::TonicService},
    },
};

use crate::auth::user::account::search::action::SearchAuthUserAccountInfo;

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

impl AsCheckedInfra<LiveSearchAuthUserAccountProxyInfra> for Data<ProxyAppFeature> {
    fn required(&self) -> AuthPermissionRequired {
        SearchAuthUserAccountInfo::required()
    }
    fn as_authorized_infra(
        &self,
        _: &CheckAuthorizeTokenSuccess,
    ) -> LiveSearchAuthUserAccountProxyInfra {
        LiveSearchAuthUserAccountProxyInfra {
            proxy_call: TonicSearchAuthUserAccountProxyCall {
                service_url: self.auth.service_url,
                authorizer: GoogleServiceAuthorizer::new(Arc::clone(
                    &self.auth.google_authorize_store,
                )),
            },
        }
    }
}

pub struct LiveSearchAuthUserAccountProxyInfra {
    proxy_call: TonicSearchAuthUserAccountProxyCall<GoogleServiceAuthorizer>,
}

impl ProxyCallInfra for LiveSearchAuthUserAccountProxyInfra {
    type ProxyCall = TonicSearchAuthUserAccountProxyCall<GoogleServiceAuthorizer>;

    fn proxy_call(&self) -> &Self::ProxyCall {
        &self.proxy_call
    }
}

pub struct TonicSearchAuthUserAccountProxyCall<A: ServiceAuthorizer> {
    service_url: &'static str,
    authorizer: A,
}

#[async_trait::async_trait]
impl<A: ServiceAuthorizer> ProxyCall for TonicSearchAuthUserAccountProxyCall<A> {
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
            SearchAuthUserAccountRequestPb::decode(decode_base64(request)?)
                .map_err(Into::<MessageError>::into)?,
        );

        TonicService::set_metadata(
            &mut request,
            self.authorizer.fetch_token(&self.service_url).await?,
            info,
            metadata,
        )?;

        let response =
            SearchAuthUserAccountPbClient::new(TonicService::endpoint(&self.service_url).await?)
                .search(request)
                .await?;

        Ok(ProxyResponseBody::restore(encode_protobuf_base64(
            response.into_inner(),
        )?))
    }
}
