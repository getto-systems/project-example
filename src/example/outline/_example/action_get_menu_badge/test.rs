use std::collections::HashSet;

use getto_application_test::ActionTestRunner;

use crate::{
    auth::remote::init::test::{
        StaticAuthMetadata, StaticAuthTokenDecoder, StaticValidateApiTokenStruct,
        StaticValidateService,
    },
    example::outline::_example::get_menu_badge::init::{
        menu_badge_repository::test::StaticOutlineMenuBadgeRepository,
        test::StaticGetOutlineMenuBadgeStruct,
    },
};

use super::action::{GetOutlineMenuBadgeAction, GetOutlineMenuBadgeMaterial};

#[tokio::test]
async fn success_get_menu_badge() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::standard(&store);

    let mut action = GetOutlineMenuBadgeAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec!["get menu badge success"]);
    assert!(result.is_ok());
}

struct TestFeature {
    get_menu_badge: StaticGetOutlineMenuBadgeStruct,
}

impl GetOutlineMenuBadgeMaterial for TestFeature {
    type GetMenuBadge = StaticGetOutlineMenuBadgeStruct;

    fn get_menu_badge(&self) -> &Self::GetMenuBadge {
        &self.get_menu_badge
    }
}

struct TestStore {}

impl TestStore {
    fn standard() -> Self {
        Self {}
    }
}

impl TestFeature {
    fn standard(_store: &TestStore) -> Self {
        Self {
            get_menu_badge: StaticGetOutlineMenuBadgeStruct {
                validate_infra: StaticValidateApiTokenStruct {
                    auth_metadata: StaticAuthMetadata {
                        nonce: "NONCE".into(),
                        token: "TOKEN".into(),
                    },
                    token_decoder: StaticAuthTokenDecoder::valid(
                        "TICKET-ID".into(),
                        "USER-ID".into(),
                        HashSet::new(),
                    ),
                    validate_service: StaticValidateService::new("USER-ID".into()),
                },
                menu_badge_repository: StaticOutlineMenuBadgeRepository,
            },
        }
    }
}
