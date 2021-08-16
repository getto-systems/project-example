use getto_application_test::ActionTestRunner;

use crate::auth::auth_ticket::_api::{
    kernel::init::{
        nonce_header::test::StaticAuthNonceHeader, test::StaticAuthHeaderStruct,
        token_header::test::StaticAuthTokenHeader,
    },
    logout::init::{logout_service::test::StaticLogoutService, test::StaticLogoutStruct},
};

use super::action::{LogoutAction, LogoutMaterial};

use crate::auth::auth_ticket::_common::kernel::data::{AuthNonceValue, AuthTokenValue};

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
                header_infra: standard_header_infra(),
                logout_service: StaticLogoutService,
            },
        }
    }
}

fn standard_header_infra() -> StaticAuthHeaderStruct {
    StaticAuthHeaderStruct {
        nonce_header: StaticAuthNonceHeader::Valid(AuthNonceValue::new("NONCE".into())),
        token_header: StaticAuthTokenHeader::Valid(AuthTokenValue::new("TOKEN".into())),
    }
}
