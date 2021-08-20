use crate::outline::_example::get_menu_badge::init::GetOutlineMenuBadgeStruct;

use super::action::{GetOutlineMenuBadgeAction, GetOutlineMenuBadgeMaterial};

pub struct GetOutlineMenuBadgeFeature {
    get_menu_badge: GetOutlineMenuBadgeStruct,
}

impl GetOutlineMenuBadgeFeature {
    pub fn action() -> GetOutlineMenuBadgeAction<Self> {
        GetOutlineMenuBadgeAction::with_material(Self {
            get_menu_badge: GetOutlineMenuBadgeStruct::new(),
        })
    }
}

#[async_trait::async_trait]
impl GetOutlineMenuBadgeMaterial for GetOutlineMenuBadgeFeature {
    type GetMenuBadge = GetOutlineMenuBadgeStruct;

    fn get_menu_badge(&self) -> &Self::GetMenuBadge {
        &self.get_menu_badge
    }
}
