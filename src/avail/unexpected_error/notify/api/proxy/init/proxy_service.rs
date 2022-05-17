use prost::Message;
use tonic::Request;

use crate::avail::unexpected_error::notify::y_protobuf::service::{
    notify_pb_client::NotifyPbClient, NotifyRequestPb,
};

use crate::common::x_outside_feature::feature::CoreOutsideService;

use crate::avail::unexpected_error::notify::x_tonic::route::ServiceNotify;

use crate::z_lib::service::init::authorizer::GoogleServiceAuthorizer;

use crate::{
    auth::proxy::helper::{infra_error, set_metadata},
    z_lib::{
        message::helper::{decode_base64, encode_protobuf_base64, invalid_protobuf},
        service::helper::new_endpoint,
    },
};

use crate::auth::{infra::AuthMetadataContent, proxy::infra::AuthProxyService};

use crate::{
    auth::proxy::data::{AuthProxyError, AuthProxyResponse},
    z_lib::message::data::MessageError,
};

pub struct ProxyService<'a> {
    service_url: &'static str,
    request_id: &'a str,
    authorizer: GoogleServiceAuthorizer<'a>,
    body: String,
}

impl<'a> ProxyService<'a> {
    pub fn new(service: &'a CoreOutsideService, request_id: &'a str, body: String) -> Self {
        Self {
            service_url: service.service_url,
            request_id,
            authorizer: GoogleServiceAuthorizer::new(&service.google_authorizer),
            body,
        }
    }
}

#[async_trait::async_trait]
impl<'a> AuthProxyService for ProxyService<'a> {
    type Response = AuthProxyResponse;

    fn name(&self) -> &str {
        ServiceNotify::name()
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
            .map_err(|err| infra_error("service endpoint error", err))?
            .connect()
            .await
            .map_err(|err| infra_error("connect error", err))?,
    );

    let mut request = Request::new(
        decode_request(service.body).map_err(|err| infra_error("decode request error", err))?,
    );
    set_metadata(
        &mut request,
        service.request_id,
        &service.authorizer,
        metadata,
    )
    .await
    .map_err(|err| infra_error("metadata error", err))?;

    let response = client
        .notify(request)
        .await
        .map_err(AuthProxyError::from)?
        .into_inner();
    Ok(AuthProxyResponse::new(
        encode_protobuf_base64(response)
            .map_err(|err| infra_error("decode response error", err))?,
    ))
}

fn decode_request(body: String) -> Result<NotifyRequestPb, MessageError> {
    NotifyRequestPb::decode(decode_base64(body)?).map_err(invalid_protobuf)
}
