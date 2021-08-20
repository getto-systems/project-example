pub(in crate::outline) mod get_menu_badge_service;
pub(in crate::outline) mod response_encoder;

use actix_web::HttpRequest;

use crate::outline::_api::get_menu_badge::init::response_encoder::ProstGetOutlineMenuBadgeResponseEncoder;
use crate::x_outside_feature::_api::feature::AppFeature;

use crate::{
    auth::_api::common::init::ValidateApiTokenStruct,
    outline::_api::get_menu_badge::init::get_menu_badge_service::TonicGetOutlineMenuBadgeService,
};

use super::infra::GetOutlineMenuBadgeInfra;

pub struct GetOutlineMenuBadgeStruct<'a> {
    validate_infra: ValidateApiTokenStruct<'a>,
    get_menu_service: TonicGetOutlineMenuBadgeService<'a>,
    response_encoder: ProstGetOutlineMenuBadgeResponseEncoder,
}

impl<'a> GetOutlineMenuBadgeStruct<'a> {
    pub fn new(feature: &'a AppFeature, request_id: &'a str, request: &'a HttpRequest) -> Self {
        Self {
            validate_infra: ValidateApiTokenStruct::new(&feature.auth, request_id, request),
            get_menu_service: TonicGetOutlineMenuBadgeService::new(
                &feature.outline.service,
                request_id,
            ),
            response_encoder: ProstGetOutlineMenuBadgeResponseEncoder,
        }
    }
}

impl<'a> GetOutlineMenuBadgeInfra for GetOutlineMenuBadgeStruct<'a> {
    type ValidateInfra = ValidateApiTokenStruct<'a>;
    type GetMenuService = TonicGetOutlineMenuBadgeService<'a>;
    type ResponseEncoder = ProstGetOutlineMenuBadgeResponseEncoder;

    fn validate_infra(&self) -> &Self::ValidateInfra {
        &self.validate_infra
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
        auth::_api::common::init::test::StaticValidateApiTokenStruct,
        outline::_api::get_menu_badge::init::{
            get_menu_badge_service::test::StaticGetOutlineMenuBadgeService,
            response_encoder::test::StaticGetOutlineMenuBadgeResponseEncoder,
        },
    };

    use super::super::infra::GetOutlineMenuBadgeInfra;

    pub struct StaticGetOutlineMenuBadgeStruct {
        pub validate_infra: StaticValidateApiTokenStruct,
        pub get_menu_service: StaticGetOutlineMenuBadgeService,
        pub response_encoder: StaticGetOutlineMenuBadgeResponseEncoder,
    }

    impl GetOutlineMenuBadgeInfra for StaticGetOutlineMenuBadgeStruct {
        type ValidateInfra = StaticValidateApiTokenStruct;
        type GetMenuService = StaticGetOutlineMenuBadgeService;
        type ResponseEncoder = StaticGetOutlineMenuBadgeResponseEncoder;

        fn validate_infra(&self) -> &Self::ValidateInfra {
            &self.validate_infra
        }
        fn get_menu_service(&self) -> &Self::GetMenuService {
            &self.get_menu_service
        }
        fn response_encoder(&self) -> &Self::ResponseEncoder {
            &self.response_encoder
        }
    }
}
