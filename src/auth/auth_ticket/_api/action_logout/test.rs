use getto_application_test::ActionTestRunner;

use crate::auth::auth_ticket::_api::{
    kernel::init::{
        nonce_header::test::StaticAuthNonceHeader, token_header::test::StaticAuthTokenHeader,
    },
    logout::init::{logout_service::test::StaticLogoutService, test::StaticLogoutStruct},
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
                nonce_header: StaticAuthNonceHeader::new("NONCE"),
                token_header: StaticAuthTokenHeader::new("TOKEN"),
                logout_service: StaticLogoutService,
            },
        }
    }
}
