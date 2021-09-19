pub(in crate::example) mod get_menu_badge_service;
pub(in crate::example) mod response_encoder;

use actix_web::HttpRequest;

use crate::x_outside_feature::_api::feature::AppFeature;

use crate::example::outline::_api::get_menu_badge::init::response_encoder::ProstGetOutlineMenuBadgeResponseEncoder;

use crate::{
    auth::_api::init::ApiServiceMetadata,
    example::outline::_api::get_menu_badge::init::get_menu_badge_service::TonicGetOutlineMenuBadgeService,
};

use super::infra::GetOutlineMenuBadgeInfra;

pub struct GetOutlineMenuBadgeStruct<'a> {
    service_metadata: ApiServiceMetadata<'a>,
    get_menu_service: TonicGetOutlineMenuBadgeService<'a>,
    response_encoder: ProstGetOutlineMenuBadgeResponseEncoder,
}

impl<'a> GetOutlineMenuBadgeStruct<'a> {
    pub fn new(feature: &'a AppFeature, request_id: &'a str, request: &'a HttpRequest) -> Self {
        Self {
            service_metadata: ApiServiceMetadata::new(&feature.auth.key, request),
            get_menu_service: TonicGetOutlineMenuBadgeService::new(
                &feature.outline.service,
                request_id,
            ),
            response_encoder: ProstGetOutlineMenuBadgeResponseEncoder,
        }
    }
}

impl<'a> GetOutlineMenuBadgeInfra for GetOutlineMenuBadgeStruct<'a> {
    type ServiceMetadata = ApiServiceMetadata<'a>;
    type GetMenuService = TonicGetOutlineMenuBadgeService<'a>;
    type ResponseEncoder = ProstGetOutlineMenuBadgeResponseEncoder;

    fn service_metadata(&self) -> &Self::ServiceMetadata {
        &self.service_metadata
    }
    fn get_menu_service(&self) -> &Self::GetMenuService {
        &self.get_menu_service
    }
    fn response_encoder(&self) -> &Self::ResponseEncoder {
        &self.response_encoder
    }
}

#[cfg(test)]
pub mod test {
    use crate::{
        auth::_common::init::test::StaticAuthServiceMetadata,
        example::outline::_api::get_menu_badge::init::{
            get_menu_badge_service::test::StaticGetOutlineMenuBadgeService,
            response_encoder::test::StaticGetOutlineMenuBadgeResponseEncoder,
        },
    };

    use super::super::infra::GetOutlineMenuBadgeInfra;

    pub struct StaticGetOutlineMenuBadgeStruct {
        pub service_metadata: StaticAuthServiceMetadata,
        pub get_menu_service: StaticGetOutlineMenuBadgeService,
        pub response_encoder: StaticGetOutlineMenuBadgeResponseEncoder,
    }

    impl GetOutlineMenuBadgeInfra for StaticGetOutlineMenuBadgeStruct {
        type ServiceMetadata = StaticAuthServiceMetadata;
        type GetMenuService = StaticGetOutlineMenuBadgeService;
        type ResponseEncoder = StaticGetOutlineMenuBadgeResponseEncoder;

        fn service_metadata(&self) -> &Self::ServiceMetadata {
            &self.service_metadata
        }
        fn get_menu_service(&self) -> &Self::GetMenuService {
            &self.get_menu_service
        }
        fn response_encoder(&self) -> &Self::ResponseEncoder {
            &self.response_encoder
        }
    }
}
