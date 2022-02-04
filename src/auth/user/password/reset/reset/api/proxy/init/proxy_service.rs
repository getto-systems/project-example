use prost::Message;
use tonic::Request;

use crate::auth::user::password::reset::reset::y_protobuf::service::{
    reset_password_pb_client::ResetPasswordPbClient, ResetPasswordRequestPb,
};

use crate::auth::x_outside_feature::api::proxy::feature::AuthOutsideFeature;

use crate::auth::user::password::reset::reset::api::x_tonic::route::ServiceReset;

use crate::{
    auth::ticket::kernel::api::init::response_builder::CookieAuthTokenResponseBuilder,
    z_lib::api::service::init::authorizer::GoogleServiceAuthorizer,
};

use crate::{
    auth::{proxy::helper::infra_error, ticket::kernel::api::infra::AuthTokenResponseBuilder},
    z_lib::api::{
        message::helper::{decode_base64, encode_protobuf_base64, invalid_protobuf},
        service::helper::new_endpoint,
    },
};

use crate::auth::proxy::method::set_metadata;

use crate::auth::{proxy::infra::AuthProxyService, ticket::validate::infra::AuthMetadataContent};

use crate::{
    auth::{
        proxy::data::AuthProxyError,
        ticket::kernel::api::data::{AuthTokenMessage, AuthTokenResponse},
    },
    z_lib::api::message::data::MessageError,
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
    type Response = AuthTokenResponse;

    fn name(&self) -> &str {
        ServiceReset::name()
    }
    async fn call(self, metadata: AuthMetadataContent) -> Result<Self::Response, AuthProxyError> {
        call(self, metadata).await
    }
}

async fn call<'a>(
    service: ProxyService<'a>,
    metadata: AuthMetadataContent,
) -> Result<AuthTokenResponse, AuthProxyError> {
    let mut client = ResetPasswordPbClient::new(
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
        .reset(request)
        .await
        .map_err(AuthProxyError::from)?
        .into_inner();

    let (token, message) = response.extract();
    Ok(service.response_builder.build(AuthTokenMessage {
        body: encode_protobuf_base64(message).map_err(AuthProxyError::MessageError)?,
        token: token
            .and_then(|token| token.into())
            .ok_or(AuthProxyError::MessageError(MessageError::Invalid(
                "invalid response; token not exists".into(),
            )))?,
    }))
}

fn decode_request(body: String) -> Result<ResetPasswordRequestPb, MessageError> {
    ResetPasswordRequestPb::decode(decode_base64(body)?).map_err(invalid_protobuf)
}
