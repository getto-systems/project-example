use tonic::Request;

use crate::outline::_common::y_protobuf::service::{
    get_menu_pb_client::GetMenuPbClient, GetMenuRequestPb,
};

use crate::outline::_api::x_outside_feature::feature::OutlineOutsideService;

use crate::outline::_api::service::helper::{infra_error, set_metadata};

use crate::outline::_api::get_menu_badge::infra::GetOutlineMenuBadgeService;

use crate::outline::{_api::service::data::OutlineServiceError, _common::data::OutlineMenuBadge};

pub struct TonicGetOutlineMenuBadgeService<'a> {
    outline_service_url: &'static str,
    request_id: &'a str,
}

impl<'a> TonicGetOutlineMenuBadgeService<'a> {
    pub const fn new(service: &'a OutlineOutsideService, request_id: &'a str) -> Self {
        Self {
            outline_service_url: service.outline_service_url,
            request_id,
        }
    }
}

#[async_trait::async_trait]
impl<'a> GetOutlineMenuBadgeService for TonicGetOutlineMenuBadgeService<'a> {
    async fn get_menu(&self) -> Result<OutlineMenuBadge, OutlineServiceError> {
        let mut client = GetMenuPbClient::connect(self.outline_service_url)
            .await
            .map_err(infra_error)?;

        let mut request = Request::new(GetMenuRequestPb {});
        set_metadata(&mut request, self.request_id)?;

        let response = client
            .get_menu(request)
            .await
            .map_err(OutlineServiceError::from)?
            .into_inner();
        Ok(response.into())
    }
}

#[cfg(test)]
pub mod test {
    use crate::outline::_api::get_menu_badge::infra::GetOutlineMenuBadgeService;

    use crate::outline::{
        _api::service::data::OutlineServiceError,
        _common::data::{OutlineMenuBadge, OutlineMenuBadgeCount},
    };

    pub struct StaticGetOutlineMenuBadgeService;

    #[async_trait::async_trait]
    impl GetOutlineMenuBadgeService for StaticGetOutlineMenuBadgeService {
        async fn get_menu(&self) -> Result<OutlineMenuBadge, OutlineServiceError> {
            Ok(OutlineMenuBadge {
                index: OutlineMenuBadgeCount::restore(0),
            })
        }
    }
}
