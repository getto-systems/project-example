use tonic::Request;

use crate::auth::ticket::check::y_protobuf::service::{
    check_auth_ticket_pb_client::CheckAuthTicketPbClient, CheckAuthTicketRequestPb,
};

use crate::auth::ticket::check::x_tonic::route::ServiceCheck;

use crate::auth::x_outside_feature::feature::AuthProxyOutsideFeature;

use crate::{
    auth::ticket::kernel::init::response_builder::CookieAuthTokenResponseBuilder,
    z_lib::service::init::authorizer::GoogleServiceAuthorizer,
};

use crate::{
    auth::proxy::helper::{infra_error, set_metadata},
    z_lib::{message::helper::encode_protobuf_base64, service::helper::new_endpoint},
};

use crate::auth::{
    proxy::infra::AuthProxyService,
    ticket::{kernel::infra::AuthTokenResponseBuilder, validate::infra::AuthMetadataContent},
};

use crate::auth::{
    proxy::data::AuthProxyError,
    ticket::kernel::data::{AuthResponse, AuthTokenMessage, EncodedAuthTokens},
};

pub struct ProxyService<'a> {
    service_url: &'static str,
    request_id: &'a str,
    authorizer: GoogleServiceAuthorizer<'a>,
    response_builder: CookieAuthTokenResponseBuilder<'a>,
}

impl<'a> ProxyService<'a> {
    pub fn new(feature: &'a AuthProxyOutsideFeature, request_id: &'a str) -> Self {
        Self {
            service_url: feature.service.service_url,
            request_id,
            authorizer: GoogleServiceAuthorizer::new(&feature.service.google_authorizer),
            response_builder: CookieAuthTokenResponseBuilder::new(&feature.cookie),
        }
    }
}

#[async_trait::async_trait]
impl<'a> AuthProxyService for ProxyService<'a> {
    type Response = AuthResponse;

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
) -> Result<AuthResponse, AuthProxyError> {
    let mut client = CheckAuthTicketPbClient::new(
        new_endpoint(service.service_url)
            .map_err(|err| infra_error("service endpoint error", err))?
            .connect()
            .await
            .map_err(|err| infra_error("connect error", err))?,
    );

    let mut request = Request::new(CheckAuthTicketRequestPb {});
    set_metadata(
        &mut request,
        service.request_id,
        &service.authorizer,
        metadata,
    )
    .await
    .map_err(|err| infra_error("metadata error", err))?;

    let response = client
        .check(request)
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
