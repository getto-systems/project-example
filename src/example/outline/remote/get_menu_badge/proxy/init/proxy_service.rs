use tonic::Request;

use crate::example::outline::remote::y_protobuf::service::{
    get_menu_badge_pb_client::GetMenuBadgePbClient, GetMenuBadgeRequestPb,
};

use crate::example::remote::x_outside_feature::feature::ExampleOutsideService;

use crate::z_lib::remote::service::init::authorizer::GoogleServiceAuthorizer;

use crate::{
    auth::proxy::helper::infra_error,
    z_lib::remote::{message::helper::encode_protobuf_base64, service::helper::new_endpoint},
};

use crate::auth::proxy::method::set_metadata;

use crate::{auth::proxy::infra::AuthProxyService, auth::remote::infra::AuthMetadataContent};

use crate::auth::proxy::data::{AuthProxyError, AuthProxyResponse};

pub struct ProxyService<'a> {
    service_url: &'static str,
    request_id: &'a str,
    authorizer: GoogleServiceAuthorizer,
}

impl<'a> ProxyService<'a> {
    pub fn new(service: &'a ExampleOutsideService, request_id: &'a str) -> Self {
        Self {
            service_url: service.service_url,
            request_id,
            authorizer: GoogleServiceAuthorizer::new(service.service_url),
        }
    }
}

#[async_trait::async_trait]
impl<'a> AuthProxyService for ProxyService<'a> {
    type Response = AuthProxyResponse;

    fn name(&self) -> &str {
        "example.outline.get_menu_badge"
    }
    async fn call(self, metadata: AuthMetadataContent) -> Result<Self::Response, AuthProxyError> {
        call(self, metadata).await
    }
}

async fn call<'a>(
    service: ProxyService<'a>,
    metadata: AuthMetadataContent,
) -> Result<AuthProxyResponse, AuthProxyError> {
    let mut client = GetMenuBadgePbClient::new(
        new_endpoint(service.service_url)
            .map_err(infra_error)?
            .connect()
            .await
            .map_err(infra_error)?,
    );

    let mut request = Request::new(GetMenuBadgeRequestPb {});
    set_metadata(
        &mut request,
        service.request_id,
        &service.authorizer,
        metadata,
    )
    .await
    .map_err(infra_error)?;

    let response = client
        .get_menu_badge(request)
        .await
        .map_err(AuthProxyError::from)?
        .into_inner();
    Ok(AuthProxyResponse::new(
        encode_protobuf_base64(response).map_err(infra_error)?,
    ))
}
