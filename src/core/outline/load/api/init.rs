pub mod menu_badge_repository;

use tonic::metadata::MetadataMap;

use crate::x_outside_feature::core::feature::CoreAppFeature;

use crate::{
    auth::init::AuthorizeStruct,
    core::outline::load::init::menu_badge_repository::UnitedOutlineMenuBadgeRepository,
};

use super::action::{LoadOutlineMenuBadgeAction, LoadOutlineMenuBadgeMaterial};

pub struct LoadOutlineMenuBadgeStruct<'a> {
    check_permission: AuthorizeStruct<'a>,

    menu_badge_repository: UnitedOutlineMenuBadgeRepository,
}

impl<'a> LoadOutlineMenuBadgeStruct<'a> {
    pub fn action(
        feature: &'a CoreAppFeature,
        request_id: &'a str,
        metadata: &'a MetadataMap,
    ) -> LoadOutlineMenuBadgeAction<Self> {
        LoadOutlineMenuBadgeAction::with_material(Self {
            check_permission: AuthorizeStruct::new(
                &feature.auth.service,
                request_id,
                metadata,
            ),

            menu_badge_repository: UnitedOutlineMenuBadgeRepository,
        })
    }
}

#[async_trait::async_trait]
impl<'a> LoadOutlineMenuBadgeMaterial for LoadOutlineMenuBadgeStruct<'a> {
    type Authorize = AuthorizeStruct<'a>;

    type MenuBadgeRepository = UnitedOutlineMenuBadgeRepository;

    fn authorize(&self) -> &Self::Authorize {
        &self.check_permission
    }

    fn menu_badge_repository(&self) -> &Self::MenuBadgeRepository {
        &self.menu_badge_repository
    }
}
