use actix_web::HttpRequest;

use crate::x_outside_feature::_api::feature::AppFeature;

use crate::example::outline::_api::get_menu_badge::init::GetOutlineMenuBadgeStruct;

use super::action::{GetOutlineMenuBadgeAction, GetOutlineMenuBadgeMaterial};

pub struct GetOutlineMenuBadgeFeature<'a> {
    get_menu_badge: GetOutlineMenuBadgeStruct<'a>,
}

impl<'a> GetOutlineMenuBadgeFeature<'a> {
    pub fn action(
        feature: &'a AppFeature,
        request_id: &'a str,
        request: &'a HttpRequest,
    ) -> GetOutlineMenuBadgeAction<Self> {
        GetOutlineMenuBadgeAction::with_material(Self {
            get_menu_badge: GetOutlineMenuBadgeStruct::new(feature, request_id, request),
        })
    }
}

impl<'a> GetOutlineMenuBadgeMaterial for GetOutlineMenuBadgeFeature<'a> {
    type GetMenuBadge = GetOutlineMenuBadgeStruct<'a>;

    fn get_menu_badge(&self) -> &Self::GetMenuBadge {
        &self.get_menu_badge
    }
}
