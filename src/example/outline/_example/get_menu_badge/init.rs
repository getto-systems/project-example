pub mod menu_badge_repository;

use menu_badge_repository::UnitedOutlineMenuBadgeRepository;
use tonic::metadata::MetadataMap;

use crate::x_outside_feature::_example::feature::ExampleAppFeature;

use crate::auth::remote::init::ValidateApiTokenStruct;

use super::infra::GetOutlineMenuBadgeInfra;

pub struct GetOutlineMenuBadgeStruct<'a> {
    validate_infra: ValidateApiTokenStruct<'a>,
    menu_badge_repository: UnitedOutlineMenuBadgeRepository,
}

impl<'a> GetOutlineMenuBadgeStruct<'a> {
    pub fn new(feature: &'a ExampleAppFeature, request_id: &'a str, metadata: &'a MetadataMap) -> Self {
        Self {
            validate_infra: ValidateApiTokenStruct::new(
                &feature.auth.service,
                request_id,
                metadata,
            ),
            menu_badge_repository: UnitedOutlineMenuBadgeRepository,
        }
    }
}

impl<'a> GetOutlineMenuBadgeInfra for GetOutlineMenuBadgeStruct<'a> {
    type ValidateInfra = ValidateApiTokenStruct<'a>;
    type MenuBadgeRepository = UnitedOutlineMenuBadgeRepository;

    fn validate_infra(&self) -> &Self::ValidateInfra {
        &self.validate_infra
    }
    fn menu_badge_repository(&self) -> &Self::MenuBadgeRepository {
        &self.menu_badge_repository
    }
}

#[cfg(test)]
pub mod test {
    use super::menu_badge_repository::test::StaticOutlineMenuBadgeRepository;
    use crate::auth::remote::init::test::StaticValidateApiTokenStruct;

    use super::super::infra::GetOutlineMenuBadgeInfra;

    pub struct StaticGetOutlineMenuBadgeStruct {
        pub validate_infra: StaticValidateApiTokenStruct,
        pub menu_badge_repository: StaticOutlineMenuBadgeRepository,
    }

    impl GetOutlineMenuBadgeInfra for StaticGetOutlineMenuBadgeStruct {
        type ValidateInfra = StaticValidateApiTokenStruct;
        type MenuBadgeRepository = StaticOutlineMenuBadgeRepository;

        fn validate_infra(&self) -> &Self::ValidateInfra {
            &self.validate_infra
        }
        fn menu_badge_repository(&self) -> &Self::MenuBadgeRepository {
            &self.menu_badge_repository
        }
    }
}
