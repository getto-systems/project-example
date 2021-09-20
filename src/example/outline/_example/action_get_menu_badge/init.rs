use tonic::metadata::MetadataMap;

use crate::{
    example::outline::_example::get_menu_badge::init::GetOutlineMenuBadgeStruct,
    x_outside_feature::_example::feature::AppFeature,
};

use super::action::{GetOutlineMenuBadgeAction, GetOutlineMenuBadgeMaterial};

pub struct GetOutlineMenuBadgeFeature<'a> {
    get_menu_badge: GetOutlineMenuBadgeStruct<'a>,
}

impl<'a> GetOutlineMenuBadgeFeature<'a> {
    pub fn action(
        feature: &'a AppFeature,
        request_id: &'a str,
        metadata: &'a MetadataMap,
    ) -> GetOutlineMenuBadgeAction<Self> {
        GetOutlineMenuBadgeAction::with_material(Self {
            get_menu_badge: GetOutlineMenuBadgeStruct::new(feature, request_id, metadata),
        })
    }
}

#[async_trait::async_trait]
impl<'a> GetOutlineMenuBadgeMaterial for GetOutlineMenuBadgeFeature<'a> {
    type GetMenuBadge = GetOutlineMenuBadgeStruct<'a>;

    fn get_menu_badge(&self) -> &Self::GetMenuBadge {
        &self.get_menu_badge
    }
}
