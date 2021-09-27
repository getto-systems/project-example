use tonic::Request;

use crate::example::outline::_common::y_protobuf::service::{
    get_menu_badge_pb_client::GetMenuBadgePbClient, GetMenuBadgeRequestPb,
};

use crate::example::_api::x_outside_feature::feature::ExampleOutsideService;

use crate::z_details::_common::service::init::authorizer::GoogleServiceAuthorizer;

use crate::{
    auth::remote::helper::set_metadata, example::_api::service::helper::infra_error,
    z_details::_common::service::helper::new_endpoint,
};

use crate::{
    auth::remote::infra::AuthMetadataContent, example::_api::proxy::ExampleProxyService,
    z_details::_common::service::infra::ServiceAuthorizer,
};

use crate::example::{
    _api::service::data::ExampleServiceError, outline::_common::data::OutlineMenuBadge,
};

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
impl<'a> ExampleProxyService<(), OutlineMenuBadge> for ProxyService<'a> {
    fn name(&self) -> &str {
        "example.outline.get_menu_badge"
    }
    async fn call(
        &self,
        metadata: AuthMetadataContent,
        _params: (),
    ) -> Result<OutlineMenuBadge, ExampleServiceError> {
        let mut client = GetMenuBadgePbClient::new(
            new_endpoint(self.service_url)
                .map_err(infra_error)?
                .connect()
                .await
                .map_err(infra_error)?,
        );

        let mut request = Request::new(GetMenuBadgeRequestPb {});
        set_metadata(
            &mut request,
            self.request_id,
            self.authorizer.fetch_token().await.map_err(infra_error)?,
            metadata,
        )
        .map_err(infra_error)?;

        let response = client
            .get_menu_badge(request)
            .await
            .map_err(ExampleServiceError::from)?
            .into_inner();
        Ok(response.into())
    }
}
