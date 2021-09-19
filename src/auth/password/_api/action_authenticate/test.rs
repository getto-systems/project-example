use std::collections::HashSet;

use getto_application_test::ActionTestRunner;

use crate::auth::{
    auth_ticket::{
        _api::kernel::init::response_builder::test::StaticAuthTokenResponseBuilder,
        _common::kernel::init::service_metadata::test::StaticAuthServiceMetadata,
    },
    password::_api::authenticate::init::{
        authenticate_service::test::StaticAuthenticatePasswordService,
        request_decoder::test::StaticAuthenticatePasswordRequestDecoder,
        response_encoder::test::StaticAuthenticatePasswordResponseEncoder,
        test::StaticAuthenticatePasswordStruct,
    },
};

use crate::auth::password::_common::authenticate::infra::AuthenticatePasswordFieldsExtract;

use super::action::{AuthenticatePasswordAction, AuthenticatePasswordMaterial};

use crate::auth::auth_user::_common::kernel::data::{AuthUser, AuthUserExtract};

#[tokio::test]
async fn success_authenticate() {
    let (handler, assert_state) = ActionTestRunner::new();

    let feature = TestFeature::standard();
    let request_decoder = standard_request_decoder();

    let mut action = AuthenticatePasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec!["authenticate password"]);
    assert!(result.is_ok());
}

struct TestFeature {
    authenticate: StaticAuthenticatePasswordStruct,
}

impl AuthenticatePasswordMaterial for TestFeature {
    type Authenticate = StaticAuthenticatePasswordStruct;

    fn authenticate(&self) -> &Self::Authenticate {
        &self.authenticate
    }
}

impl<'a> TestFeature {
    fn standard() -> Self {
        Self {
            authenticate: StaticAuthenticatePasswordStruct {
                service_metadata: StaticAuthServiceMetadata {
                    nonce: "NONCE".into(),
                    token: "TOKEN".into(),
                },
                response_builder: StaticAuthTokenResponseBuilder,
                authenticate_service: StaticAuthenticatePasswordService {
                    user: standard_user(),
                },
                response_encoder: StaticAuthenticatePasswordResponseEncoder,
            },
        }
    }
}

fn standard_request_decoder() -> StaticAuthenticatePasswordRequestDecoder {
    StaticAuthenticatePasswordRequestDecoder {
        fields: AuthenticatePasswordFieldsExtract {
            login_id: "LOGIN-ID".into(),
            password: "PASSWORD".into(),
        },
    }
}

fn standard_user() -> AuthUser {
    AuthUserExtract {
        user_id: "USER-ID".into(),
        granted_roles: HashSet::new(),
    }
    .restore()
}
