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
    auth::x_outside_feature::feature::AuthProxyOutsideFeature, x_outside_feature::data::RequestId,
};

use crate::common::api::message::helper::encode_protobuf_base64;

use crate::common::api::service::init::service::GoogleTonicService;

use crate::common::proxy::infra::ProxyCall;

use crate::{
    auth::{
        proxy::data::{AuthProxyError, ProxyDomain},
        ticket::{authenticate::proxy::data::ProxyResponseAuthenticated, kernel::data::AuthToken},
    },
    common::proxy::data::{ProxyMetadataExtract, ProxyResponseBody},
};

pub struct TonicAuthenticateWithTokenProxyCall<'a> {
    service: GoogleTonicService<'a>,
    domain: &'a str,
}

impl<'a> TonicAuthenticateWithTokenProxyCall<'a> {
    pub fn new(feature: &'a AuthProxyOutsideFeature, request_id: RequestId) -> Self {
        Self {
            service: GoogleTonicService::new(&feature.service, request_id),
            domain: feature.cookie.domain,
        }
    }
}

#[async_trait::async_trait]
impl<'a> ProxyCall for TonicAuthenticateWithTokenProxyCall<'a> {
    type Request = ();
    type Response = ProxyResponseAuthenticated;
    type Error = AuthProxyError;

    async fn call(
        &self,
        metadata: impl ProxyMetadataExtract,
        _request: Self::Request,
    ) -> Result<Self::Response, Self::Error> {
        let mut client = AuthenticateWithTokenPbClient::new(
            self.service
                .endpoint()
                .await
                .map_err(AuthProxyError::ServiceConnectError)?,
        );

        let mut request = Request::new(AuthenticateWithTokenRequestPb {});
        self.service
            .set_metadata(&mut request, metadata)
            .await
            .map_err(AuthProxyError::ServiceMetadataError)?;

        let response = client.call(request).await.map_err(AuthProxyError::from)?;

        let (token, message) = extract_response(response.into_inner());
        let token: Option<AuthToken> = token.and_then(|token| token.into());
        let body = encode_protobuf_base64(message).map_err(AuthProxyError::MessageError)?;

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

#[cfg(test)]
pub mod test {
    use crate::common::proxy::infra::ProxyCall;

    use crate::{
        auth::{
            proxy::data::AuthProxyError,
            ticket::authenticate::proxy::data::ProxyResponseAuthenticated,
        },
        common::proxy::data::{ProxyMetadataExtract, ProxyResponseBody},
    };

    pub struct StaticAuthenticateWithTokenProxyCall;

    #[async_trait::async_trait]
    impl ProxyCall for StaticAuthenticateWithTokenProxyCall {
        type Request = ();
        type Response = ProxyResponseAuthenticated;
        type Error = AuthProxyError;

        async fn call(
            &self,
            _metadata: impl ProxyMetadataExtract,
            _request: Self::Request,
        ) -> Result<Self::Response, Self::Error> {
            Ok((ProxyResponseBody::restore("RESPONSE".to_owned()), None))
        }
    }
}
