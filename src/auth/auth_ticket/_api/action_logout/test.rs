use getto_application_test::ActionTestRunner;

use crate::auth::auth_ticket::{
    _api::logout::init::{logout_service::test::StaticLogoutService, test::StaticLogoutStruct},
    _common::kernel::init::service_metadata::test::StaticAuthServiceMetadata,
};

use super::action::{LogoutAction, LogoutMaterial};

#[tokio::test]
async fn success_logout() {
    let (handler, assert_state) = ActionTestRunner::new();

    let feature = TestFeature::standard();

    let mut action = LogoutAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec!["logout success"]);
    assert!(result.is_ok());
}

struct TestFeature {
    logout: StaticLogoutStruct,
}

impl LogoutMaterial for TestFeature {
    type Logout = StaticLogoutStruct;

    fn logout(&self) -> &Self::Logout {
        &self.logout
    }
}

impl TestFeature {
    fn standard() -> Self {
        Self {
            logout: StaticLogoutStruct {
                service_metadata: StaticAuthServiceMetadata {
                    nonce: "NONCE".into(),
                    token: "TOKEN".into(),
                },
                logout_service: StaticLogoutService,
            },
        }
    }
}
