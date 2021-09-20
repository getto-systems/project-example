use std::collections::HashSet;

use getto_application_test::ActionTestRunner;

use crate::auth::{
    auth_ticket::{
        _api::{
            kernel::init::response_builder::test::StaticAuthTokenResponseBuilder,
            renew::init::{
                renew_service::test::StaticRenewAuthTicketService,
                response_encoder::test::StaticRenewAuthTicketResponseEncoder,
                test::StaticRenewAuthTicketStruct,
            },
        },
        _common::kernel::init::service_metadata::test::StaticAuthServiceMetadata,
    },
    auth_user::_common::kernel::data::{AuthUser, AuthUserExtract},
};

use super::action::{RenewAuthTicketAction, RenewAuthTicketMaterial};

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
                service_metadata: StaticAuthServiceMetadata {
                    nonce: "NONCE".into(),
                    token: "TOKEN".into(),
                },
                response_builder: StaticAuthTokenResponseBuilder,
                renew_service: StaticRenewAuthTicketService {
                    user: standard_user(),
                },
                response_encoder: StaticRenewAuthTicketResponseEncoder,
            },
        }
    }
}

fn standard_user() -> AuthUser {
    AuthUserExtract {
        user_id: "USER-ID".into(),
        granted_roles: HashSet::new(),
    }
    .restore()
}
