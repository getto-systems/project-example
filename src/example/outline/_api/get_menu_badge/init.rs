pub(in crate::example) mod get_menu_badge_service;
pub(in crate::example) mod response_encoder;

use actix_web::HttpRequest;

use crate::x_outside_feature::_api::feature::AppFeature;

use crate::example::outline::_api::get_menu_badge::init::response_encoder::ProstGetOutlineMenuBadgeResponseEncoder;

use crate::{
    auth::_api::init::ApiAuthMetadata,
    example::outline::_api::get_menu_badge::init::get_menu_badge_service::TonicGetOutlineMenuBadgeService,
};

use super::infra::GetOutlineMenuBadgeInfra;

pub struct GetOutlineMenuBadgeStruct<'a> {
    auth_metadata: ApiAuthMetadata<'a>,
    get_menu_service: TonicGetOutlineMenuBadgeService<'a>,
    response_encoder: ProstGetOutlineMenuBadgeResponseEncoder,
}

impl<'a> GetOutlineMenuBadgeStruct<'a> {
    pub fn new(feature: &'a AppFeature, request_id: &'a str, request: &'a HttpRequest) -> Self {
        Self {
            auth_metadata: ApiAuthMetadata::new(&feature.auth.key, request),
            get_menu_service: TonicGetOutlineMenuBadgeService::new(
                &feature.outline.service,
                request_id,
            ),
            response_encoder: ProstGetOutlineMenuBadgeResponseEncoder,
        }
    }
}

impl<'a> GetOutlineMenuBadgeInfra for GetOutlineMenuBadgeStruct<'a> {
    type AuthMetadata = ApiAuthMetadata<'a>;
    type GetMenuService = TonicGetOutlineMenuBadgeService<'a>;
    type ResponseEncoder = ProstGetOutlineMenuBadgeResponseEncoder;

    fn auth_metadata(&self) -> &Self::AuthMetadata {
        &self.auth_metadata
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
        auth::_common::init::test::StaticAuthMetadata,
        example::outline::_api::get_menu_badge::init::{
            get_menu_badge_service::test::StaticGetOutlineMenuBadgeService,
            response_encoder::test::StaticGetOutlineMenuBadgeResponseEncoder,
        },
    };

    use super::super::infra::GetOutlineMenuBadgeInfra;

    pub struct StaticGetOutlineMenuBadgeStruct {
        pub auth_metadata: StaticAuthMetadata,
        pub get_menu_service: StaticGetOutlineMenuBadgeService,
        pub response_encoder: StaticGetOutlineMenuBadgeResponseEncoder,
    }

    impl GetOutlineMenuBadgeInfra for StaticGetOutlineMenuBadgeStruct {
        type AuthMetadata = StaticAuthMetadata;
        type GetMenuService = StaticGetOutlineMenuBadgeService;
        type ResponseEncoder = StaticGetOutlineMenuBadgeResponseEncoder;

        fn auth_metadata(&self) -> &Self::AuthMetadata {
            &self.auth_metadata
        }
        fn get_menu_service(&self) -> &Self::GetMenuService {
            &self.get_menu_service
        }
        fn response_encoder(&self) -> &Self::ResponseEncoder {
            &self.response_encoder
        }
    }
}
