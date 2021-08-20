use getto_application_test::ActionTestRunner;

use crate::{
    auth::_api::common::init::test::{
        StaticAuthNonceHeader, StaticAuthTokenHeader, StaticValidateApiTokenStruct,
        StaticValidateService,
    },
    outline::_api::get_menu_badge::init::{
        get_menu_badge_service::test::StaticGetOutlineMenuBadgeService,
        response_encoder::test::StaticGetOutlineMenuBadgeResponseEncoder,
        test::StaticGetOutlineMenuBadgeStruct,
    },
};

use super::action::{GetOutlineMenuBadgeAction, GetOutlineMenuBadgeMaterial};

#[tokio::test]
async fn success_get_menu_badge() {
    let (handler, assert_state) = ActionTestRunner::new();

    let feature = TestFeature::standard();

    let mut action = GetOutlineMenuBadgeAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "authorized; user: USER-ID",
        "get outline menu badge success",
    ]);
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

impl TestFeature {
    fn standard() -> Self {
        Self {
            get_menu_badge: StaticGetOutlineMenuBadgeStruct {
                validate_infra: StaticValidateApiTokenStruct {
                    nonce_header: StaticAuthNonceHeader::new("NONCE"),
                    token_header: StaticAuthTokenHeader::new("TOKEN"),
                    validate_service: StaticValidateService::new("USER-ID"),
                },
                get_menu_service: StaticGetOutlineMenuBadgeService,
                response_encoder: StaticGetOutlineMenuBadgeResponseEncoder,
            },
        }
    }
}
