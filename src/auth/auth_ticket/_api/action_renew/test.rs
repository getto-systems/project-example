use std::collections::HashSet;

use getto_application_test::ActionTestRunner;

use crate::auth::{
    auth_ticket::_api::{
        kernel::init::test::{
            StaticAuthHeaderStruct, StaticAuthNonceHeader, StaticAuthTokenHeader,
        },
        renew::init::test::{
            StaticRenewAuthTicketMessenger, StaticRenewAuthTicketService,
            StaticRenewAuthTicketStruct,
        },
    },
    auth_user::_common::kernel::data::{AuthUser, AuthUserExtract},
};

use super::action::{RenewAuthTicketAction, RenewAuthTicketMaterial};

use crate::auth::auth_ticket::_api::kernel::data::{AuthNonceValue, AuthTokenValue};

#[tokio::test]
async fn success_renew_ticket() {
    let (handler, assert_state) = ActionTestRunner::new();

    let feature = TestFeature::standard();

    let mut action = RenewAuthTicketAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec!["renew success"]);
    assert!(result.is_ok());
}

struct TestFeature {
    renew: StaticRenewAuthTicketStruct,
}

impl RenewAuthTicketMaterial for TestFeature {
    type Renew = StaticRenewAuthTicketStruct;

    fn renew(&self) -> &Self::Renew {
        &self.renew
    }
}

impl TestFeature {
    fn standard() -> Self {
        Self {
            renew: StaticRenewAuthTicketStruct {
                header_infra: standard_header_infra(),
                renew_service: StaticRenewAuthTicketService {
                    user: standard_user(),
                },
                messenger: StaticRenewAuthTicketMessenger,
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

fn standard_user() -> AuthUser {
    AuthUserExtract {
        user_id: "USER-ID".into(),
        granted_roles: HashSet::new(),
    }
    .into()
}
