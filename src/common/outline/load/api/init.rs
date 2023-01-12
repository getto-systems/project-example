mod menu_badge_repository;

use crate::x_outside_feature::{core::feature::CoreAppFeature, data::RequestId};

use crate::x_content::menu::badge::ActiveOutlineMenuBadgeRepository;

use crate::auth::init::ActiveAuthorizeInfra;

use crate::common::outline::load::action::{
    LoadOutlineMenuBadgeAction, LoadOutlineMenuBadgeMaterial,
};

pub struct ActiveLoadOutlineMenuBadgeMaterial<'a> {
    authorize: ActiveAuthorizeInfra<'a>,
    menu_badge_repository: ActiveOutlineMenuBadgeRepository,
}

impl<'a> ActiveLoadOutlineMenuBadgeMaterial<'a> {
    pub fn action(
        feature: &'a CoreAppFeature,
        request_id: RequestId,
    ) -> LoadOutlineMenuBadgeAction<Self> {
        LoadOutlineMenuBadgeAction::with_material(Self {
            authorize: ActiveAuthorizeInfra::from_service(&feature.auth, request_id),
            menu_badge_repository: ActiveOutlineMenuBadgeRepository::new(),
        })
    }
}

#[async_trait::async_trait]
impl<'a> LoadOutlineMenuBadgeMaterial for ActiveLoadOutlineMenuBadgeMaterial<'a> {
    type Authorize = ActiveAuthorizeInfra<'a>;

    type MenuBadgeRepository = ActiveOutlineMenuBadgeRepository;

    fn authorize(&self) -> &Self::Authorize {
        &self.authorize
    }
    fn menu_badge_repository(&self) -> &Self::MenuBadgeRepository {
        &self.menu_badge_repository
    }
}

#[cfg(test)]
pub mod test {
    pub use crate::common::outline::load::init::menu_badge_repository::test::StaticOutlineMenuBadgeRepository;

    use crate::auth::init::test::StaticAuthorizeInfra;

    use crate::common::outline::load::action::LoadOutlineMenuBadgeMaterial;

    pub struct StaticLoadOutlineMenuBadgeMaterial {
        pub authorize: StaticAuthorizeInfra,
        pub menu_badge_repository: StaticOutlineMenuBadgeRepository,
    }

    #[async_trait::async_trait]
    impl LoadOutlineMenuBadgeMaterial for StaticLoadOutlineMenuBadgeMaterial {
        type Authorize = StaticAuthorizeInfra;
        type MenuBadgeRepository = StaticOutlineMenuBadgeRepository;

        fn authorize(&self) -> &Self::Authorize {
            &self.authorize
        }
        fn menu_badge_repository(&self) -> &Self::MenuBadgeRepository {
            &self.menu_badge_repository
        }
    }
}
