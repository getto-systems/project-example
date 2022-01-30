use prost::Message;
use tonic::Request;

use crate::avail::unexpected_error::remote::y_protobuf::service::{
    notify_pb_client::NotifyPbClient, NotifyRequestPb,
};

use crate::example::remote::x_outside_feature::feature::ExampleOutsideService;

use crate::z_lib::remote::message::helper::{
    decode_base64, encode_protobuf_base64, invalid_protobuf,
};

use crate::z_lib::remote::service::init::authorizer::GoogleServiceAuthorizer;

use crate::{
    auth::remote::proxy::helper::infra_error, z_lib::remote::service::helper::new_endpoint,
};

use crate::auth::remote::proxy::method::set_metadata;

use crate::auth::remote::{infra::AuthMetadataContent, proxy::infra::AuthProxyService};

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
    pub fn new(service: &'a ExampleOutsideService, request_id: &'a str, body: String) -> Self {
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
        "avail.unexpected_error.notify"
    }
    async fn call(self, metadata: AuthMetadataContent) -> Result<Self::Response, AuthProxyError> {
        call(self, metadata).await
    }
}

async fn call<'a>(
    service: ProxyService<'a>,
    metadata: AuthMetadataContent,
) -> Result<AuthProxyResponse, AuthProxyError> {
    let mut client = NotifyPbClient::new(
        new_endpoint(service.service_url)
            .map_err(infra_error)?
            .connect()
            .await
            .map_err(infra_error)?,
    );

    let mut request = Request::new(decode_request(service.body).map_err(infra_error)?);
    set_metadata(
        &mut request,
        service.request_id,
        &service.authorizer,
        metadata,
    )
    .await
    .map_err(infra_error)?;

    let response = client
        .notify(request)
        .await
        .map_err(AuthProxyError::from)?
        .into_inner();
    Ok(AuthProxyResponse::new(
        encode_protobuf_base64(response).map_err(infra_error)?,
    ))
}

fn decode_request(body: String) -> Result<NotifyRequestPb, MessageError> {
    NotifyRequestPb::decode(decode_base64(body)?).map_err(invalid_protobuf)
}
