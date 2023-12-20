use std::sync::Arc;

use actix_web::web::Data;
use prost::Message;
use tonic::Request;

use crate::auth::{
    ticket::y_protobuf::service::AuthTokenPb,
    user::password::reset::reset::y_protobuf::service::{
        reset_password_pb_client::ResetPasswordPbClient, ResetPasswordMaskedResponsePb,
        ResetPasswordRequestPb, ResetPasswordResponsePb,
    },
};

use crate::x_outside_feature::proxy::feature::ProxyAppFeature;

use crate::common::api::{
    feature::AsInfra,
    logger::detail::StdoutJsonLogger,
    message::helper::{decode_base64, encode_protobuf_base64},
    service::detail::{authorizer::GoogleServiceAuthorizer, service::TonicService},
};

use crate::{
    auth::user::password::reset::reset::proxy::infra::{
        ResetPasswordProxyInfra, ResetPasswordProxyLogger,
    },
    common::{api::service::infra::ServiceAuthorizer, proxy::infra::ProxyCall},
};

use crate::{
    auth::{
        proxy::data::{AuthProxyCallError, ProxyDomain},
        ticket::{authenticate::proxy::data::ProxyResponseAuthenticated, kernel::data::AuthToken},
    },
    common::{
        api::{message::data::MessageError, request::data::RequestInfo},
        proxy::data::{ProxyMetadataExtract, ProxyResponseBody},
    },
};

pub struct LiveResetPasswordProxyInfra {
    proxy_call: TonicResetPasswordProxyCall<GoogleServiceAuthorizer>,
}

impl AsInfra<LiveResetPasswordProxyInfra> for Data<ProxyAppFeature> {
    fn as_infra(&self) -> LiveResetPasswordProxyInfra {
        LiveResetPasswordProxyInfra {
            proxy_call: TonicResetPasswordProxyCall {
                service_url: self.auth.service_url,
                domain: self.auth.cookie.domain,
                authorizer: GoogleServiceAuthorizer::new(Arc::clone(
                    &self.auth.google_authorize_store,
                )),
            },
        }
    }
}

#[async_trait::async_trait]
impl<'a> ResetPasswordProxyInfra for LiveResetPasswordProxyInfra {
    type ProxyCall = TonicResetPasswordProxyCall<GoogleServiceAuthorizer>;

    fn proxy_call(&self) -> &Self::ProxyCall {
        &self.proxy_call
    }
}

pub struct TonicResetPasswordProxyCall<A: ServiceAuthorizer> {
    service_url: &'static str,
    domain: &'static str,
    authorizer: A,
}

#[async_trait::async_trait]
impl<A: ServiceAuthorizer> ProxyCall for TonicResetPasswordProxyCall<A> {
    type Request = String;
    type Response = ProxyResponseAuthenticated;
    type Error = AuthProxyCallError;

    async fn call(
        &self,
        info: RequestInfo,
        metadata: impl ProxyMetadataExtract,
        request: Self::Request,
    ) -> Result<Self::Response, Self::Error> {
        let mut request = Request::new(
            ResetPasswordRequestPb::decode(decode_base64(request)?)
                .map_err(Into::<MessageError>::into)?,
        );

        TonicService::set_metadata(
            &mut request,
            self.authorizer.fetch_token(&self.service_url).await?,
            info,
            metadata,
        )?;

        let response = ResetPasswordPbClient::new(TonicService::endpoint(&self.service_url).await?)
            .reset(request)
            .await?;

        let (token, message) = extract_response(response.into_inner());
        let token: Option<AuthToken> = token.and_then(|token| token.into());
        let body = encode_protobuf_base64(message).map_err(AuthProxyCallError::MessageError)?;

        Ok((
            ProxyResponseBody::restore(body),
            token.map(|token| (token, ProxyDomain::restore(self.domain.to_owned()))),
        ))
    }
}

fn extract_response(
    response: ResetPasswordResponsePb,
) -> (Option<AuthTokenPb>, ResetPasswordMaskedResponsePb) {
    (
        response.token,
        ResetPasswordMaskedResponsePb {
            success: response.success,
            granted: response.granted,
            err: response.err,
        },
    )
}

impl ResetPasswordProxyLogger for StdoutJsonLogger {
    fn try_to_reset_password(&self) {
        self.info(format!("try to reset password"));
    }
    fn proxy_error(&self, err: AuthProxyCallError) -> AuthProxyCallError {
        self.fatal(format!("failed to proxy call; {}", err));
        err
    }
    fn succeed_to_reset_password(
        &self,
        response: ProxyResponseAuthenticated,
    ) -> ProxyResponseAuthenticated {
        self.info(format!("succeed to reset password"));
        response
    }
}
