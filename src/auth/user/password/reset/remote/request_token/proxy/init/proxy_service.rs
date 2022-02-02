use prost::Message;
use tonic::Request;

use crate::auth::user::password::reset::remote::y_protobuf::service::{
    request_reset_token_pb_client::RequestResetTokenPbClient, RequestResetTokenRequestPb,
};

use crate::auth::remote::x_outside_feature::common::feature::AuthOutsideService;

use crate::z_lib::remote::service::init::authorizer::GoogleServiceAuthorizer;

use crate::{
    auth::remote::proxy::helper::infra_error,
    z_lib::remote::{
        message::helper::{decode_base64, encode_protobuf_base64, invalid_protobuf},
        service::helper::new_endpoint,
    },
};

use crate::auth::remote::proxy::method::set_metadata;

use crate::auth::{
    remote::proxy::infra::AuthProxyService, ticket::validate::infra::AuthMetadataContent,
};

use crate::{
    auth::remote::proxy::data::{AuthProxyError, AuthProxyResponse},
    z_lib::remote::message::data::MessageError,
};

pub struct ProxyService<'a> {
    service_url: &'static str,
    request_id: &'a str,
    authorizer: GoogleServiceAuthorizer,
    body: String,
}

impl<'a> ProxyService<'a> {
    pub fn new(service: &'a AuthOutsideService, request_id: &'a str, body: String) -> Self {
        Self {
            service_url: service.service_url,
            request_id,
            authorizer: GoogleServiceAuthorizer::new(service.service_url),
            body,
        }
    }
}

#[async_trait::async_trait]
impl<'a> AuthProxyService for ProxyService<'a> {
    type Response = AuthProxyResponse;

    fn name(&self) -> &str {
        "auth.user.password.reset.request_token"
    }
    async fn call(self, metadata: AuthMetadataContent) -> Result<Self::Response, AuthProxyError> {
        call(self, metadata).await
    }
}

async fn call<'a>(
    service: ProxyService<'a>,
    metadata: AuthMetadataContent,
) -> Result<AuthProxyResponse, AuthProxyError> {
    let mut client = RequestResetTokenPbClient::new(
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
        .request_token(request)
        .await
        .map_err(AuthProxyError::from)?
        .into_inner();

    Ok(AuthProxyResponse::new(
        encode_protobuf_base64(response).map_err(AuthProxyError::MessageError)?,
    ))
}

fn decode_request(body: String) -> Result<RequestResetTokenRequestPb, MessageError> {
    RequestResetTokenRequestPb::decode(decode_base64(body)?).map_err(invalid_protobuf)
}
