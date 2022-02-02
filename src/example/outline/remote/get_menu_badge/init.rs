pub mod menu_badge_repository;

use tonic::metadata::MetadataMap;

use crate::x_outside_feature::remote::example::feature::ExampleAppFeature;

use crate::{
    auth::init::CheckPermissionStruct,
    example::outline::remote::get_menu_badge::init::menu_badge_repository::UnitedOutlineMenuBadgeRepository,
};

use super::action::{GetOutlineMenuBadgeAction, GetOutlineMenuBadgeMaterial};

pub struct GetOutlineMenuBadgeFeature<'a> {
    check_permission: CheckPermissionStruct<'a>,

    menu_badge_repository: UnitedOutlineMenuBadgeRepository,
}

impl<'a> GetOutlineMenuBadgeFeature<'a> {
    pub fn action(
        feature: &'a ExampleAppFeature,
        request_id: &'a str,
        metadata: &'a MetadataMap,
    ) -> GetOutlineMenuBadgeAction<Self> {
        GetOutlineMenuBadgeAction::with_material(Self {
            check_permission: CheckPermissionStruct::new(
                &feature.auth.service,
                request_id,
                metadata,
            ),

            menu_badge_repository: UnitedOutlineMenuBadgeRepository,
        })
    }
}

#[async_trait::async_trait]
impl<'a> GetOutlineMenuBadgeMaterial for GetOutlineMenuBadgeFeature<'a> {
    type CheckPermission = CheckPermissionStruct<'a>;

    type MenuBadgeRepository = UnitedOutlineMenuBadgeRepository;

    fn check_permission(&self) -> &Self::CheckPermission {
        &self.check_permission
    }

    fn menu_badge_repository(&self) -> &Self::MenuBadgeRepository {
        &self.menu_badge_repository
    }
}
