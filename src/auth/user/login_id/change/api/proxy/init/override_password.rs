use prost::Message;
use tonic::Request;

use crate::auth::user::login_id::change::y_protobuf::service::{
    override_login_id_pb_client::OverrideLoginIdPbClient, OverrideLoginIdRequestPb,
};

use crate::auth::x_outside_feature::feature::AuthOutsideService;

use crate::auth::user::login_id::change::x_tonic::route::ServiceOverrideLoginId;

use crate::z_lib::service::init::authorizer::GoogleServiceAuthorizer;

use crate::{
    auth::proxy::helper::{infra_error, set_metadata},
    z_lib::{
        message::helper::{decode_base64, encode_protobuf_base64, invalid_protobuf},
        service::helper::new_endpoint,
    },
};

use crate::auth::{proxy::infra::AuthProxyService, ticket::validate::infra::AuthMetadataContent};

use crate::{
    auth::proxy::data::{AuthProxyError, AuthProxyResponse},
    z_lib::message::data::MessageError,
};

pub struct OverrideLoginIdProxyService<'a> {
    service_url: &'static str,
    request_id: &'a str,
    authorizer: GoogleServiceAuthorizer<'a>,
    body: String,
}

impl<'a> OverrideLoginIdProxyService<'a> {
    pub fn new(service: &'a AuthOutsideService, request_id: &'a str, body: String) -> Self {
        Self {
            service_url: service.service_url,
            request_id,
            authorizer: GoogleServiceAuthorizer::new(&service.google_authorizer),
            body,
        }
    }
}

#[async_trait::async_trait]
impl<'a> AuthProxyService for OverrideLoginIdProxyService<'a> {
    type Response = AuthProxyResponse;

    fn name(&self) -> &str {
        ServiceOverrideLoginId::name()
    }
    async fn call(self, metadata: AuthMetadataContent) -> Result<Self::Response, AuthProxyError> {
        call(self, metadata).await
    }
}

async fn call<'a>(
    service: OverrideLoginIdProxyService<'a>,
    metadata: AuthMetadataContent,
) -> Result<AuthProxyResponse, AuthProxyError> {
    let mut client = OverrideLoginIdPbClient::new(
        new_endpoint(service.service_url)
            .map_err(|err| infra_error("service endpoint error", err))?
            .connect()
            .await
            .map_err(|err| infra_error("connect error", err))?,
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
    .map_err(|err| infra_error("metadata error", err))?;

    let response = client
        .override_login_id(request)
        .await
        .map_err(AuthProxyError::from)?
        .into_inner();

    Ok(AuthProxyResponse::new(
        encode_protobuf_base64(response).map_err(AuthProxyError::MessageError)?,
    ))
}

fn decode_request(body: String) -> Result<OverrideLoginIdRequestPb, MessageError> {
    OverrideLoginIdRequestPb::decode(decode_base64(body)?).map_err(invalid_protobuf)
}
