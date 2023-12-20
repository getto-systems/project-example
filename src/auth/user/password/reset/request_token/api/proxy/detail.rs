use std::sync::Arc;

use actix_web::web::Data;
use prost::Message;
use tonic::Request;

use crate::auth::user::password::reset::request_token::y_protobuf::service::{
    request_reset_token_pb_client::RequestResetTokenPbClient, RequestResetTokenRequestPb,
};

use crate::{
    common::api::{
        feature::AsInfra,
        logger::detail::StdoutJsonLogger,
        message::helper::{decode_base64, encode_protobuf_base64},
        service::detail::{authorizer::GoogleServiceAuthorizer, service::TonicService},
    },
    x_outside_feature::proxy::feature::ProxyAppFeature,
};

use crate::{
    auth::user::password::reset::request_token::proxy::infra::{
        RequestResetPasswordTokenProxyInfra, RequestResetPasswordTokenProxyLogger,
    },
    common::{api::service::infra::ServiceAuthorizer, proxy::infra::ProxyCall},
};

use crate::{
    auth::proxy::data::AuthProxyCallError,
    common::{
        api::{message::data::MessageError, request::data::RequestInfo},
        proxy::data::{ProxyMetadataExtract, ProxyResponseBody},
    },
};

pub struct LiveRequestResetPasswordTokenProxyInfra {
    proxy_call: TonicRequestResetPasswordTokenProxyCall<GoogleServiceAuthorizer>,
}

impl AsInfra<LiveRequestResetPasswordTokenProxyInfra> for Data<ProxyAppFeature> {
    fn as_infra(&self) -> LiveRequestResetPasswordTokenProxyInfra {
        LiveRequestResetPasswordTokenProxyInfra {
            proxy_call: TonicRequestResetPasswordTokenProxyCall {
                service_url: self.auth.service_url,
                authorizer: GoogleServiceAuthorizer::new(Arc::clone(
                    &self.auth.google_authorize_store,
                )),
            },
        }
    }
}

#[async_trait::async_trait]
impl RequestResetPasswordTokenProxyInfra for LiveRequestResetPasswordTokenProxyInfra {
    type ProxyCall = TonicRequestResetPasswordTokenProxyCall<GoogleServiceAuthorizer>;

    fn proxy_call(&self) -> &Self::ProxyCall {
        &self.proxy_call
    }
}

impl RequestResetPasswordTokenProxyLogger for StdoutJsonLogger {
    fn try_to_request_reset_token(&self) {
        self.info(format!("try to request reset-password-token"));
    }
    fn proxy_error(&self, err: AuthProxyCallError) -> AuthProxyCallError {
        self.fatal(format!("proxy error; {}", err));
        err
    }
    fn succeed_to_request_reset_token(&self, response: ProxyResponseBody) -> ProxyResponseBody {
        self.info(format!("succeed to request reset-password-token"));
        response
    }
}

pub struct TonicRequestResetPasswordTokenProxyCall<A: ServiceAuthorizer> {
    service_url: &'static str,
    authorizer: A,
}

#[async_trait::async_trait]
impl<A: ServiceAuthorizer> ProxyCall for TonicRequestResetPasswordTokenProxyCall<A> {
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
            RequestResetTokenRequestPb::decode(decode_base64(request)?)
                .map_err(Into::<MessageError>::into)?,
        );

        TonicService::set_metadata(
            &mut request,
            self.authorizer.fetch_token(&self.service_url).await?,
            info,
            metadata,
        )?;

        let response =
            RequestResetTokenPbClient::new(TonicService::endpoint(&self.service_url).await?)
                .request_token(request)
                .await?;

        let body = encode_protobuf_base64(response.into_inner())
            .map_err(AuthProxyCallError::MessageError)?;

        Ok(ProxyResponseBody::restore(body))
    }
}
