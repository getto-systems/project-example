use prost::Message;
use tonic::Request;

use crate::auth::{
    ticket::y_protobuf::service::AuthTokenPb,
    user::password::reset::reset::y_protobuf::service::{
        reset_password_pb_client::ResetPasswordPbClient, ResetPasswordMaskedResponsePb,
        ResetPasswordRequestPb, ResetPasswordResponsePb,
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

pub struct TonicResetPasswordProxyCall<'a> {
    service: GoogleTonicService<'a>,
    domain: &'a str,
}

impl<'a> TonicResetPasswordProxyCall<'a> {
    pub fn new(feature: &'a AuthProxyOutsideFeature, request_id: RequestId) -> Self {
        Self {
            service: GoogleTonicService::new(&feature.service, request_id),
            domain: feature.cookie.domain,
        }
    }
}

#[async_trait::async_trait]
impl<'a> ProxyCall for TonicResetPasswordProxyCall<'a> {
    type Request = String;
    type Response = ProxyResponseAuthenticated;
    type Error = AuthProxyError;

    async fn call(
        &self,
        metadata: impl ProxyMetadataExtract,
        request: Self::Request,
    ) -> Result<Self::Response, Self::Error> {
        let mut client = ResetPasswordPbClient::new(
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

        let response = client.reset(request).await.map_err(AuthProxyError::from)?;

        let (token, message) = extract_response(response.into_inner());
        let token: Option<AuthToken> = token.and_then(|token| token.into());
        let body = encode_protobuf_base64(message).map_err(AuthProxyError::MessageError)?;

        Ok((
            ProxyResponseBody::restore(body),
            token.map(|token| (token, ProxyDomain::restore(self.domain.to_owned()))),
        ))
    }
}

fn decode_request(request: String) -> Result<ResetPasswordRequestPb, MessageError> {
    ResetPasswordRequestPb::decode(decode_base64(request)?).map_err(invalid_protobuf)
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

    pub struct StaticResetPasswordProxyCall;

    #[async_trait::async_trait]
    impl ProxyCall for StaticResetPasswordProxyCall {
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
