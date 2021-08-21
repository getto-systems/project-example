pub(in crate::example) mod menu_badge_repository;

use menu_badge_repository::UnitedOutlineMenuBadgeRepository;

use super::infra::GetOutlineMenuBadgeInfra;

pub struct GetOutlineMenuBadgeStruct {
    menu_badge_repository: UnitedOutlineMenuBadgeRepository,
}

impl GetOutlineMenuBadgeStruct {
    pub fn new() -> Self {
        Self {
            menu_badge_repository: UnitedOutlineMenuBadgeRepository,
        }
    }
}

impl GetOutlineMenuBadgeInfra for GetOutlineMenuBadgeStruct {
    type MenuBadgeRepository = UnitedOutlineMenuBadgeRepository;

    fn menu_badge_repository(&self) -> &Self::MenuBadgeRepository {
        &self.menu_badge_repository
    }
}

#[cfg(test)]
pub mod test {
    use super::menu_badge_repository::test::StaticOutlineMenuBadgeRepository;

    use super::super::infra::GetOutlineMenuBadgeInfra;

    pub struct StaticGetOutlineMenuBadgeStruct {
        pub menu_badge_repository: StaticOutlineMenuBadgeRepository,
    }

    impl GetOutlineMenuBadgeInfra for StaticGetOutlineMenuBadgeStruct {
        type MenuBadgeRepository = StaticOutlineMenuBadgeRepository;

        fn menu_badge_repository(&self) -> &Self::MenuBadgeRepository {
            &self.menu_badge_repository
        }
    }
}
