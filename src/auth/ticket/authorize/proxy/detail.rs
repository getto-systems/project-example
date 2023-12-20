use std::sync::Arc;

use tonic::Request;

use crate::auth::ticket::authorize::y_protobuf::service::{
    authorize_pb_client::AuthorizePbClient, AuthorizeRequestPb, AuthorizeResponsePb,
};

use crate::{
    auth::ticket::kernel::detail::token::authorize::decoder::JwtAuthorizeTokenDecoder,
    common::api::{
        feature::AsInfra,
        logger::detail::StdoutJsonLogger,
        service::detail::{authorizer::GoogleServiceAuthorizer, service::TonicService},
    },
    x_outside_feature::core::feature::CoreAppFeature,
};

use crate::{
    auth::ticket::authorize::proxy::infra::{AuthorizeProxyInfra, AuthorizeProxyLogger},
    common::{api::service::infra::ServiceAuthorizer, proxy::infra::ProxyCall},
};

use crate::{
    auth::{
        proxy::data::AuthProxyCallError,
        ticket::{
            authorize::data::AuthorizeSuccess,
            kernel::data::{
                AuthPermissionError, AuthPermissionRequired, AuthTicketAttrs,
                DecodeAuthorizeTokenError, ValidateAuthorizeTokenError,
            },
        },
        user::kernel::data::AuthUserId,
    },
    common::{api::request::data::RequestInfo, proxy::data::ProxyMetadataExtract},
};

pub struct LiveAuthorizeProxyInfra {
    token_decoder: JwtAuthorizeTokenDecoder,
    proxy_call: TonicAuthorizeProxyCall<GoogleServiceAuthorizer>,
}

impl AsInfra<LiveAuthorizeProxyInfra> for Arc<CoreAppFeature> {
    fn as_infra(&self) -> LiveAuthorizeProxyInfra {
        LiveAuthorizeProxyInfra {
            token_decoder: self.as_infra(),
            proxy_call: TonicAuthorizeProxyCall {
                service_url: self.auth.service_url,
                authorizer: GoogleServiceAuthorizer::new(Arc::clone(
                    &self.auth.google_authorize_store,
                )),
            },
        }
    }
}

impl AuthorizeProxyInfra for LiveAuthorizeProxyInfra {
    type TokenDecoder = JwtAuthorizeTokenDecoder;
    type ProxyCall = TonicAuthorizeProxyCall<GoogleServiceAuthorizer>;

    fn token_decoder(&self) -> &Self::TokenDecoder {
        &self.token_decoder
    }
    fn proxy_call(&self) -> &Self::ProxyCall {
        &self.proxy_call
    }
}

impl AuthorizeProxyLogger for StdoutJsonLogger {
    fn try_to_authorize(&self) {
        self.debug(format!("try to authorize"));
    }
    fn invalid_request(&self, err: ValidateAuthorizeTokenError) -> ValidateAuthorizeTokenError {
        self.incident(format!("invalid authorize request; {}", err));
        err
    }
    fn invalid_token(&self, err: DecodeAuthorizeTokenError) -> DecodeAuthorizeTokenError {
        match err {
            DecodeAuthorizeTokenError::Expired => {
                self.debug(format!("authorize-token expired; {}", err))
            }
            DecodeAuthorizeTokenError::Invalid(_) => {
                self.incident(format!("invalid authorize-token; {}", err))
            }
        }
        err
    }
    fn forbidden(&self, err: AuthPermissionError) -> AuthPermissionError {
        self.incident(format!("forbidden; {}", err));
        err
    }
    fn proxy_error(&self, err: AuthProxyCallError) -> AuthProxyCallError {
        self.incident(format!("proxy error; {}", err));
        err
    }
    fn authorized(&self, auth: AuthorizeSuccess) -> AuthorizeSuccess {
        self.debug(format!("authorized; {}", auth));
        auth
    }
}

pub struct TonicAuthorizeProxyCall<A: ServiceAuthorizer> {
    service_url: &'static str,
    authorizer: A,
}

#[async_trait::async_trait]
impl<A: ServiceAuthorizer> ProxyCall for TonicAuthorizeProxyCall<A> {
    type Request = AuthPermissionRequired;
    type Response = AuthorizeSuccess;
    type Error = AuthProxyCallError;

    async fn call(
        &self,
        info: RequestInfo,
        metadata: impl ProxyMetadataExtract,
        request: Self::Request,
    ) -> Result<Self::Response, Self::Error> {
        let mut request = Request::new(match request {
            AuthPermissionRequired::Nothing => AuthorizeRequestPb {
                require_nothing: true,
                ..Default::default()
            },
            AuthPermissionRequired::HasSome(permissions) => AuthorizeRequestPb {
                require_nothing: false,
                require_permissions: permissions
                    .into_iter()
                    .map(|permission| permission.extract())
                    .collect(),
            },
        });

        TonicService::set_metadata(
            &mut request,
            self.authorizer.fetch_token(&self.service_url).await?,
            info,
            metadata,
        )?;

        let response = AuthorizePbClient::new(TonicService::endpoint(&self.service_url).await?)
            .authorize(request)
            .await?;

        Ok(AuthorizeSuccess::new(response.into_inner().into()))
    }
}

impl Into<AuthTicketAttrs> for AuthorizeResponsePb {
    fn into(self) -> AuthTicketAttrs {
        AuthTicketAttrs {
            user_id: AuthUserId::restore(self.user_id),
            granted: self.granted.unwrap_or_default().into(),
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::auth::ticket::kernel::detail::token::authorize::decoder::test::MockAuthorizeTokenDecoder;

    use crate::common::api::feature::AsInfra;

    use crate::{
        auth::ticket::authorize::proxy::infra::AuthorizeProxyInfra, common::proxy::infra::ProxyCall,
    };

    use crate::{
        auth::{
            proxy::data::AuthProxyCallError,
            ticket::{authorize::data::AuthorizeSuccess, kernel::data::AuthPermissionRequired},
        },
        common::{api::request::data::RequestInfo, proxy::data::ProxyMetadataExtract},
    };

    pub struct MockAuthorizeProxyInfra {
        token_decoder: MockAuthorizeTokenDecoder,
        proxy_call: MockAuthorizeProxyCall,
    }

    impl AsInfra<MockAuthorizeProxyInfra> for (MockAuthorizeTokenDecoder, AuthorizeSuccess) {
        fn as_infra(&self) -> MockAuthorizeProxyInfra {
            MockAuthorizeProxyInfra {
                token_decoder: self.0.clone(),
                proxy_call: MockAuthorizeProxyCall(self.1.clone()),
            }
        }
    }

    impl AuthorizeProxyInfra for MockAuthorizeProxyInfra {
        type TokenDecoder = MockAuthorizeTokenDecoder;
        type ProxyCall = MockAuthorizeProxyCall;

        fn token_decoder(&self) -> &Self::TokenDecoder {
            &self.token_decoder
        }
        fn proxy_call(&self) -> &Self::ProxyCall {
            &self.proxy_call
        }
    }

    pub struct MockAuthorizeProxyCall(AuthorizeSuccess);

    #[async_trait::async_trait]
    impl ProxyCall for MockAuthorizeProxyCall {
        type Request = AuthPermissionRequired;
        type Response = AuthorizeSuccess;
        type Error = AuthProxyCallError;

        async fn call(
            &self,
            _info: RequestInfo,
            _metadata: impl ProxyMetadataExtract,
            _request: Self::Request,
        ) -> Result<Self::Response, Self::Error> {
            Ok(self.0.clone())
        }
    }
}
