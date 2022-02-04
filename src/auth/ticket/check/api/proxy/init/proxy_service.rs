use tonic::Request;

use crate::auth::ticket::check::y_protobuf::service::{
    check_auth_ticket_pb_client::CheckAuthTicketPbClient, CheckAuthTicketRequestPb,
};

use crate::auth::x_outside_feature::api::proxy::feature::AuthOutsideFeature;

use crate::auth::ticket::check::api::x_tonic::route::ServiceCheck;

use crate::{
    auth::ticket::kernel::api::init::response_builder::CookieAuthTokenResponseBuilder,
    z_lib::api::service::init::authorizer::GoogleServiceAuthorizer,
};

use crate::{
    auth::proxy::helper::infra_error,
    z_lib::api::{message::helper::encode_protobuf_base64, service::helper::new_endpoint},
};

use crate::auth::proxy::method::set_metadata;

use crate::auth::{
    proxy::infra::AuthProxyService,
    ticket::{kernel::api::infra::AuthTokenResponseBuilder, validate::infra::AuthMetadataContent},
};

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
}

impl<'a> ProxyService<'a> {
    pub fn new(feature: &'a AuthOutsideFeature, request_id: &'a str) -> Self {
        Self {
            service_url: feature.service.service_url,
            request_id,
            authorizer: GoogleServiceAuthorizer::new(feature.service.service_url),
            response_builder: CookieAuthTokenResponseBuilder::new(&feature.cookie),
        }
    }
}

#[async_trait::async_trait]
impl<'a> AuthProxyService for ProxyService<'a> {
    type Response = AuthTokenResponse;

    fn name(&self) -> &str {
        ServiceCheck::name()
    }
    async fn call(self, metadata: AuthMetadataContent) -> Result<Self::Response, AuthProxyError> {
        call(self, metadata).await
    }
}

async fn call<'a>(
    service: ProxyService<'a>,
    metadata: AuthMetadataContent,
) -> Result<AuthTokenResponse, AuthProxyError> {
    let mut client = CheckAuthTicketPbClient::new(
        new_endpoint(service.service_url)
            .map_err(infra_error)?
            .connect()
            .await
            .map_err(infra_error)?,
    );

    let mut request = Request::new(CheckAuthTicketRequestPb {});
    set_metadata(
        &mut request,
        service.request_id,
        &service.authorizer,
        metadata,
    )
    .await
    .map_err(infra_error)?;

    let response = client
        .check(request)
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
