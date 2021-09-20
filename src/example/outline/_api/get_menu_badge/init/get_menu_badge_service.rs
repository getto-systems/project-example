use tonic::Request;

use crate::auth::_common::infra::AuthMetadataContent;
use crate::example::outline::_common::y_protobuf::service::{
    get_menu_badge_pb_client::GetMenuBadgePbClient, GetMenuBadgeRequestPb,
};

use crate::example::_api::x_outside_feature::feature::ExampleOutsideService;

use crate::z_details::_common::service::init::authorizer::GoogleServiceAuthorizer;

use crate::example::_api::service::helper::{
    infra_error, new_endpoint, set_authorization, set_metadata,
};

use crate::example::outline::_api::get_menu_badge::infra::GetOutlineMenuBadgeService;

use crate::example::{
    _api::service::data::ExampleServiceError, outline::_common::data::OutlineMenuBadge,
};

pub struct TonicGetOutlineMenuBadgeService<'a> {
    service_url: &'static str,
    request_id: &'a str,
    authorizer: GoogleServiceAuthorizer,
}

impl<'a> TonicGetOutlineMenuBadgeService<'a> {
    pub fn new(service: &'a ExampleOutsideService, request_id: &'a str) -> Self {
        Self {
            service_url: service.service_url,
            request_id,
            authorizer: GoogleServiceAuthorizer::new(service.service_url),
        }
    }
}

#[async_trait::async_trait]
impl<'a> GetOutlineMenuBadgeService for TonicGetOutlineMenuBadgeService<'a> {
    async fn get_menu(
        &self,
        metadata: AuthMetadataContent,
    ) -> Result<OutlineMenuBadge, ExampleServiceError> {
        let mut client = GetMenuBadgePbClient::new(
            new_endpoint(self.service_url)?
                .connect()
                .await
                .map_err(infra_error)?,
        );

        let mut request = Request::new(GetMenuBadgeRequestPb {});
        set_authorization(&mut request, &self.authorizer).await?;
        set_metadata(&mut request, self.request_id, metadata)?;

        let response = client
            .get_menu_badge(request)
            .await
            .map_err(ExampleServiceError::from)?
            .into_inner();
        Ok(response.into())
    }
}

#[cfg(test)]
pub mod test {
    use crate::{
        auth::_common::infra::AuthMetadataContent,
        example::outline::_api::get_menu_badge::infra::GetOutlineMenuBadgeService,
    };

    use crate::example::{
        _api::service::data::ExampleServiceError,
        outline::_common::data::{OutlineMenuBadge, OutlineMenuBadgeCount},
    };

    pub struct StaticGetOutlineMenuBadgeService;

    #[async_trait::async_trait]
    impl GetOutlineMenuBadgeService for StaticGetOutlineMenuBadgeService {
        async fn get_menu(
            &self,
            _metadata: AuthMetadataContent,
        ) -> Result<OutlineMenuBadge, ExampleServiceError> {
            Ok(OutlineMenuBadge {
                index: OutlineMenuBadgeCount::restore(0),
            })
        }
    }
}
