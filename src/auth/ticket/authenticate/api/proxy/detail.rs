use std::sync::Arc;

use actix_web::web::Data;
use tonic::Request;

use crate::auth::ticket::{
    authenticate::y_protobuf::service::{
        authenticate_with_token_pb_client::AuthenticateWithTokenPbClient,
        AuthenticateWithTokenMaskedResponsePb, AuthenticateWithTokenRequestPb,
        AuthenticateWithTokenResponsePb,
    },
    y_protobuf::service::AuthTokenPb,
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
    auth::ticket::authenticate::proxy::infra::{
        AuthenticateWithTokenProxyInfra, AuthenticateWithTokenProxyLogger,
    },
    common::{api::service::infra::ServiceAuthorizer, proxy::infra::ProxyCall},
};

use crate::{
    auth::{
        proxy::data::{AuthProxyCallError, ProxyDomain},
        ticket::{
            authenticate::{
                data::CheckAuthenticateTokenSuccess, proxy::data::ProxyResponseAuthenticated,
            },
            kernel::data::{AuthToken, ValidateAuthenticateTokenError},
        },
    },
    common::{
        api::request::data::RequestInfo,
        proxy::data::{ProxyMetadataExtract, ProxyResponseBody},
    },
};

pub struct LiveAuthenticateWithTokenProxyInfra {
    proxy_call: TonicAuthenticateWithTokenProxyCall<GoogleServiceAuthorizer>,
}

impl AsInfra<LiveAuthenticateWithTokenProxyInfra> for Data<ProxyAppFeature> {
    fn as_infra(&self) -> LiveAuthenticateWithTokenProxyInfra {
        LiveAuthenticateWithTokenProxyInfra {
            proxy_call: TonicAuthenticateWithTokenProxyCall {
                service_url: self.auth.service_url,
                domain: self.auth.cookie.domain,
                authorizer: GoogleServiceAuthorizer::new(Arc::clone(
                    &self.auth.google_authorize_store,
                )),
            },
        }
    }
}

impl AuthenticateWithTokenProxyInfra for LiveAuthenticateWithTokenProxyInfra {
    type ProxyCall = TonicAuthenticateWithTokenProxyCall<GoogleServiceAuthorizer>;

    fn proxy_call(&self) -> &Self::ProxyCall {
        &self.proxy_call
    }
}

impl AuthenticateWithTokenProxyLogger for StdoutJsonLogger {
    fn try_to_authenticate_with_token(&self) {
        self.info(format!("try to authenticate with token"));
    }
    fn invalid_request(
        &self,
        err: ValidateAuthenticateTokenError,
    ) -> ValidateAuthenticateTokenError {
        self.incident(format!("invalid authenticate request; {}", err));
        err
    }
    fn proxy_error(&self, err: AuthProxyCallError) -> AuthProxyCallError {
        self.fatal(format!("proxy error; {}", err));
        err
    }
    fn authenticated(&self, auth: ProxyResponseAuthenticated) -> ProxyResponseAuthenticated {
        self.info(format!("authenticated with token"));
        auth
    }
}

pub struct TonicAuthenticateWithTokenProxyCall<A: ServiceAuthorizer> {
    service_url: &'static str,
    domain: &'static str,
    authorizer: A,
}

#[async_trait::async_trait]
impl<A: ServiceAuthorizer> ProxyCall for TonicAuthenticateWithTokenProxyCall<A> {
    type Request = CheckAuthenticateTokenSuccess;
    type Response = ProxyResponseAuthenticated;
    type Error = AuthProxyCallError;

    async fn call(
        &self,
        info: RequestInfo,
        metadata: impl ProxyMetadataExtract,
        _request: Self::Request,
    ) -> Result<Self::Response, Self::Error> {
        let mut request = Request::new(AuthenticateWithTokenRequestPb {});

        TonicService::set_metadata(
            &mut request,
            self.authorizer.fetch_token(&self.service_url).await?,
            info,
            metadata,
        )?;

        let response =
            AuthenticateWithTokenPbClient::new(TonicService::endpoint(&self.service_url).await?)
                .call(request)
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
    response: AuthenticateWithTokenResponsePb,
) -> (Option<AuthTokenPb>, AuthenticateWithTokenMaskedResponsePb) {
    (
        response.token,
        AuthenticateWithTokenMaskedResponsePb {
            granted: response.granted,
        },
    )
}
