use tonic::Request;

use crate::common::outline::load::y_protobuf::service::{
    load_menu_badge_pb_client::LoadMenuBadgePbClient, LoadMenuBadgeRequestPb,
};

use crate::common::x_outside_feature::feature::CommonOutsideService;

use crate::common::outline::load::x_tonic::route::ServiceLoadMenuBadge;

use crate::z_lib::service::init::authorizer::GoogleServiceAuthorizer;

use crate::{
    auth::proxy::helper::{proxy_infra_error, set_metadata},
    z_lib::{message::helper::encode_protobuf_base64, service::helper::new_endpoint},
};

use crate::{auth::infra::AuthMetadataContent, auth::proxy::infra::AuthProxyService};

use crate::auth::proxy::data::{AuthProxyError, AuthProxyResponse};

pub struct ProxyService<'a> {
    service_url: &'static str,
    request_id: &'a str,
    authorizer: GoogleServiceAuthorizer<'a>,
}

impl<'a> ProxyService<'a> {
    pub fn new(service: &'a CommonOutsideService, request_id: &'a str) -> Self {
        Self {
            service_url: service.service_url,
            request_id,
            authorizer: GoogleServiceAuthorizer::new(&service.google_authorizer),
        }
    }
}

#[async_trait::async_trait]
impl<'a> AuthProxyService for ProxyService<'a> {
    type Response = AuthProxyResponse;

    fn name(&self) -> &str {
        ServiceLoadMenuBadge::name()
    }
    async fn call(self, metadata: AuthMetadataContent) -> Result<Self::Response, AuthProxyError> {
        call(self, metadata).await
    }
}

async fn call<'a>(
    service: ProxyService<'a>,
    metadata: AuthMetadataContent,
) -> Result<AuthProxyResponse, AuthProxyError> {
    let mut client = LoadMenuBadgePbClient::new(
        new_endpoint(service.service_url)
            .map_err(|err| proxy_infra_error("service endpoint error", err))?
            .connect()
            .await
            .map_err(|err| proxy_infra_error("connect error", err))?,
    );

    let mut request = Request::new(LoadMenuBadgeRequestPb {});
    set_metadata(
        &mut request,
        service.request_id,
        &service.authorizer,
        metadata,
    )
    .await
    .map_err(|err| proxy_infra_error("metadata error", err))?;

    Ok(AuthProxyResponse::new(
        encode_protobuf_base64(
            client
                .load_menu_badge(request)
                .await
                .map_err(AuthProxyError::from)?
                .into_inner(),
        )
        .map_err(|err| proxy_infra_error("decode response error", err))?,
    ))
}
