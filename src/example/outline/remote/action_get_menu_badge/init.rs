use tonic::metadata::MetadataMap;

use crate::x_outside_feature::_example::feature::ExampleAppFeature;

use crate::example::outline::remote::get_menu_badge::init::GetOutlineMenuBadgeStruct;

use super::action::{GetOutlineMenuBadgeAction, GetOutlineMenuBadgeMaterial};

pub struct GetOutlineMenuBadgeFeature<'a> {
    get_menu_badge: GetOutlineMenuBadgeStruct<'a>,
}

impl<'a> GetOutlineMenuBadgeFeature<'a> {
    pub fn action(
        feature: &'a ExampleAppFeature,
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
