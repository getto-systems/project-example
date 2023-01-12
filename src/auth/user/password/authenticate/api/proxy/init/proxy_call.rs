use prost::Message;
use tonic::Request;

use crate::auth::{
    ticket::y_protobuf::service::AuthTokenPb,
    user::password::authenticate::y_protobuf::service::{
        authenticate_with_password_pb_client::AuthenticateWithPasswordPbClient,
        AuthenticateWithPasswordMaskedResponsePb, AuthenticateWithPasswordRequestPb,
        AuthenticateWithPasswordResponsePb,
    },
};

use crate::{
    auth::x_outside_feature::feature::AuthProxyOutsideFeature, x_outside_feature::data::RequestId,
};

use crate::common::api::message::helper::{
    decode_base64, encode_protobuf_base64, invalid_protobuf,
};

use crate::common::api::service::init::service::GoogleTonicService;

use crate::common::proxy::infra::ProxyCall;

use crate::{
    auth::{
        proxy::data::{AuthProxyError, ProxyDomain},
        ticket::{authenticate::proxy::data::ProxyResponseAuthenticated, kernel::data::AuthToken},
    },
    common::api::message::data::MessageError,
    common::proxy::data::{ProxyMetadataExtract, ProxyResponseBody},
};

pub struct TonicAuthenticateWithPasswordProxyCall<'a> {
    service: GoogleTonicService<'a>,
    domain: &'a str,
}

impl<'a> TonicAuthenticateWithPasswordProxyCall<'a> {
    pub fn new(feature: &'a AuthProxyOutsideFeature, request_id: RequestId) -> Self {
        Self {
            service: GoogleTonicService::new(&feature.service, request_id),
            domain: feature.cookie.domain,
        }
    }
}

#[async_trait::async_trait]
impl<'a> ProxyCall for TonicAuthenticateWithPasswordProxyCall<'a> {
    type Request = String;
    type Response = ProxyResponseAuthenticated;
    type Error = AuthProxyError;

    async fn call(
        &self,
        metadata: impl ProxyMetadataExtract,
        request: Self::Request,
    ) -> Result<Self::Response, Self::Error> {
        let mut client = AuthenticateWithPasswordPbClient::new(
            self.service
                .endpoint()
                .await
                .map_err(AuthProxyError::ServiceConnectError)?,
        );

        let mut request =
            Request::new(decode_request(request).map_err(AuthProxyError::MessageError)?);
        self.service
            .set_metadata(&mut request, metadata)
            .await
            .map_err(AuthProxyError::ServiceMetadataError)?;

        let response = client
            .authenticate(request)
            .await
            .map_err(AuthProxyError::from)?;

        let (token, message) = extract_response(response.into_inner());
        let token: Option<AuthToken> = token.and_then(|token| token.into());
        let body = encode_protobuf_base64(message).map_err(AuthProxyError::MessageError)?;

        Ok((
            ProxyResponseBody::restore(body),
            token.map(|token| (token, ProxyDomain::restore(self.domain.to_owned()))),
        ))
    }
}

fn decode_request(request: String) -> Result<AuthenticateWithPasswordRequestPb, MessageError> {
    AuthenticateWithPasswordRequestPb::decode(decode_base64(request)?).map_err(invalid_protobuf)
}

fn extract_response(
    response: AuthenticateWithPasswordResponsePb,
) -> (
    Option<AuthTokenPb>,
    AuthenticateWithPasswordMaskedResponsePb,
) {
    (
        response.token,
        AuthenticateWithPasswordMaskedResponsePb {
            success: response.success,
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

    pub struct StaticAuthenticateWithPasswordProxyCall;

    #[async_trait::async_trait]
    impl ProxyCall for StaticAuthenticateWithPasswordProxyCall {
        type Request = String;
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
