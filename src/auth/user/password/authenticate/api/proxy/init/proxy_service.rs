use prost::Message;
use tonic::Request;

use crate::auth::user::password::authenticate::y_protobuf::service::{
    authenticate_password_pb_client::AuthenticatePasswordPbClient, AuthenticatePasswordRequestPb,
};

use crate::auth::x_outside_feature::proxy::feature::AuthOutsideFeature;

use crate::auth::user::password::authenticate::x_tonic::route::ServiceAuthenticate;

use crate::{
    auth::ticket::kernel::init::response_builder::CookieAuthTokenResponseBuilder,
    z_lib::service::init::authorizer::GoogleServiceAuthorizer,
};

use crate::{
    auth::proxy::helper::infra_error,
    z_lib::{
        message::helper::{decode_base64, encode_protobuf_base64, invalid_protobuf},
        service::helper::new_endpoint,
    },
};

use crate::auth::proxy::method::set_metadata;

use crate::auth::{
    proxy::infra::AuthProxyService,
    ticket::{kernel::infra::AuthTokenResponseBuilder, validate::infra::AuthMetadataContent},
};

use crate::{
    auth::{
        proxy::data::AuthProxyError,
        ticket::kernel::data::{AuthResponse, AuthTokenMessage, EncodedAuthTokens},
    },
    z_lib::message::data::MessageError,
};

pub struct ProxyService<'a> {
    service_url: &'static str,
    request_id: &'a str,
    authorizer: GoogleServiceAuthorizer,
    response_builder: CookieAuthTokenResponseBuilder<'a>,
    body: String,
}

impl<'a> ProxyService<'a> {
    pub fn new(feature: &'a AuthOutsideFeature, request_id: &'a str, body: String) -> Self {
        Self {
            service_url: feature.service.service_url,
            request_id,
            authorizer: GoogleServiceAuthorizer::new(feature.service.service_url),
            response_builder: CookieAuthTokenResponseBuilder::new(&feature.cookie),
            body,
        }
    }
}

#[async_trait::async_trait]
impl<'a> AuthProxyService for ProxyService<'a> {
    type Response = AuthResponse;

    fn name(&self) -> &str {
        ServiceAuthenticate::name()
    }
    async fn call(self, metadata: AuthMetadataContent) -> Result<Self::Response, AuthProxyError> {
        call(self, metadata).await
    }
}

async fn call<'a>(
    service: ProxyService<'a>,
    metadata: AuthMetadataContent,
) -> Result<AuthResponse, AuthProxyError> {
    let mut client = AuthenticatePasswordPbClient::new(
        new_endpoint(service.service_url)
            .map_err(infra_error)?
            .connect()
            .await
            .map_err(infra_error)?,
    );

    let mut request =
        Request::new(decode_request(service.body).map_err(AuthProxyError::MessageError)?);
    set_metadata(
        &mut request,
        service.request_id,
        &service.authorizer,
        metadata,
    )
    .await
    .map_err(infra_error)?;

    let response = client
        .authenticate(request)
        .await
        .map_err(AuthProxyError::from)?
        .into_inner();

    let (token, message) = response.extract();
    let token: Option<EncodedAuthTokens> = token.and_then(|token| token.into());
    let body = encode_protobuf_base64(message).map_err(AuthProxyError::MessageError)?;
    Ok(match token {
        None => AuthResponse::Failed(body),
        Some(token) => AuthResponse::Succeeded(
            service
                .response_builder
                .build(AuthTokenMessage { body, token }),
        ),
    })
}

fn decode_request(body: String) -> Result<AuthenticatePasswordRequestPb, MessageError> {
    AuthenticatePasswordRequestPb::decode(decode_base64(body)?).map_err(invalid_protobuf)
}
