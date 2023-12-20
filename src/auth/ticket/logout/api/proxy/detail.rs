use std::sync::Arc;

use actix_web::web::Data;
use tonic::Request;

use crate::auth::ticket::logout::y_protobuf::service::{
    logout_pb_client::LogoutPbClient, LogoutRequestPb,
};

use crate::{
    common::api::{
        feature::AsInfra,
        logger::detail::StdoutJsonLogger,
        message::helper::encode_protobuf_base64,
        service::detail::{authorizer::GoogleServiceAuthorizer, service::TonicService},
    },
    x_outside_feature::proxy::feature::ProxyAppFeature,
};

use crate::{
    auth::ticket::logout::proxy::infra::{LogoutProxyInfra, LogoutProxyLogger},
    common::{api::service::infra::ServiceAuthorizer, proxy::infra::ProxyCall},
};

use crate::{
    auth::{
        proxy::data::AuthProxyCallError,
        ticket::{
            authenticate::data::CheckAuthenticateTokenSuccess,
            kernel::data::ValidateAuthenticateTokenError,
        },
    },
    common::{
        api::request::data::RequestInfo,
        proxy::data::{ProxyMetadataExtract, ProxyResponseBody},
    },
};

pub struct LiveLogoutProxyInfra {
    proxy_call: TonicLogoutProxyCall<GoogleServiceAuthorizer>,
}

impl AsInfra<LiveLogoutProxyInfra> for Data<ProxyAppFeature> {
    fn as_infra(&self) -> LiveLogoutProxyInfra {
        LiveLogoutProxyInfra {
            proxy_call: TonicLogoutProxyCall::new(
                self.auth.service_url,
                GoogleServiceAuthorizer::new(Arc::clone(&self.auth.google_authorize_store)),
            ),
        }
    }
}

impl LogoutProxyInfra for LiveLogoutProxyInfra {
    type ProxyCall = TonicLogoutProxyCall<GoogleServiceAuthorizer>;

    fn proxy_call(&self) -> &Self::ProxyCall {
        &self.proxy_call
    }
}

impl LogoutProxyLogger for StdoutJsonLogger {
    fn try_to_logout(&self) {
        self.info(format!("try to logout"));
    }
    fn invalid_request(
        &self,
        err: ValidateAuthenticateTokenError,
    ) -> ValidateAuthenticateTokenError {
        self.incident(format!("invalid logout request; {}", err));
        err
    }
    fn proxy_error(&self, err: AuthProxyCallError) -> AuthProxyCallError {
        self.fatal(format!("proxy error; {}", err));
        err
    }
    fn succeed_to_logout(&self, auth: ProxyResponseBody) -> ProxyResponseBody {
        self.info(format!("succeed to logout"));
        auth
    }
}

pub struct TonicLogoutProxyCall<A: ServiceAuthorizer> {
    service_url: &'static str,
    authorizer: A,
}

impl<A: ServiceAuthorizer> TonicLogoutProxyCall<A> {
    pub fn new(service_url: &'static str, authorizer: A) -> Self {
        Self {
            service_url,
            authorizer,
        }
    }
}

#[async_trait::async_trait]
impl<A: ServiceAuthorizer> ProxyCall for TonicLogoutProxyCall<A> {
    type Request = CheckAuthenticateTokenSuccess;
    type Response = ProxyResponseBody;
    type Error = AuthProxyCallError;

    async fn call(
        &self,
        info: RequestInfo,
        metadata: impl ProxyMetadataExtract,
        _request: Self::Request,
    ) -> Result<Self::Response, Self::Error> {
        let mut request = Request::new(LogoutRequestPb {});

        TonicService::set_metadata(
            &mut request,
            self.authorizer.fetch_token(&self.service_url).await?,
            info,
            metadata,
        )?;

        let response = LogoutPbClient::new(TonicService::endpoint(&self.service_url).await?)
            .logout(request)
            .await?;

        Ok(ProxyResponseBody::restore(encode_protobuf_base64(
            response.into_inner(),
        )?))
    }
}
