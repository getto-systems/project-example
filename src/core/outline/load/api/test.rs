use std::collections::HashSet;

use getto_application_test::ActionTestRunner;

use crate::{
    auth::init::test::{
        StaticAuthMetadata, StaticAuthTokenDecoder, StaticCheckPermissionStruct,
        StaticValidateService,
    },
    core::outline::load::init::menu_badge_repository::test::StaticOutlineMenuBadgeRepository,
};

use super::action::{LoadOutlineMenuBadgeAction, LoadOutlineMenuBadgeMaterial};

#[tokio::test]
async fn success_load_menu_badge() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let material = TestStruct::standard(&store);

    let mut action = LoadOutlineMenuBadgeAction::with_material(material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec!["authorize success", "load menu badge success"]);
    assert!(result.is_ok());
}

struct TestStruct {
    validate: StaticCheckPermissionStruct,

    menu_badge_repository: StaticOutlineMenuBadgeRepository,
}

impl LoadOutlineMenuBadgeMaterial for TestStruct {
    type CheckPermission = StaticCheckPermissionStruct;

    type MenuBadgeRepository = StaticOutlineMenuBadgeRepository;

    fn check_permission(&self) -> &Self::CheckPermission {
        &self.validate
    }

    fn menu_badge_repository(&self) -> &Self::MenuBadgeRepository {
        &self.menu_badge_repository
    }
}

struct TestStore {}

impl TestStore {
    fn standard() -> Self {
        Self {}
    }
}

impl TestStruct {
    fn standard(_store: &TestStore) -> Self {
        Self {
            validate: StaticCheckPermissionStruct {
                auth_metadata: StaticAuthMetadata {
                    nonce: "NONCE".into(),
                    token: "TOKEN".into(),
                },
                token_decoder: StaticAuthTokenDecoder::valid(
                    "TICKET-ID".into(),
                    "USER-ID".into(),
                    HashSet::new(),
                ),
                validate_service: StaticValidateService,
            },
            menu_badge_repository: StaticOutlineMenuBadgeRepository,
        }
    }
}
