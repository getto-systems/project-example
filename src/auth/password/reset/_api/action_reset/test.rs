use std::collections::HashSet;

use getto_application_test::ActionTestRunner;

use crate::auth::{
    auth_ticket::_api::kernel::init::test::{
        StaticAuthHeaderStruct, StaticAuthNonceHeader, StaticAuthTokenHeader,
        StaticAuthTokenMessenger, StaticAuthTokenStruct,
    },
    password::reset::_api::reset::init::test::{
        StaticResetPasswordRequestDecoder, StaticResetPasswordResponseEncoder,
        StaticResetPasswordService, StaticResetPasswordStruct,
    },
};

use crate::auth::password::reset::_common::reset::infra::ResetPasswordFieldsExtract;

use super::action::{ResetPasswordAction, ResetPasswordMaterial};

use crate::auth::{
    auth_ticket::_common::kernel::data::{AuthNonceValue, AuthTokenValue},
    auth_user::_common::kernel::data::{AuthUser, AuthUserExtract},
};

#[tokio::test]
async fn success_request_token() {
    let (handler, assert_state) = ActionTestRunner::new();

    let feature = TestFeature::standard();

    let mut action = ResetPasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec!["reset password success"]);
    assert!(result.is_ok());
}

struct TestFeature {
    reset: StaticResetPasswordStruct,
}

impl ResetPasswordMaterial for TestFeature {
    type Reset = StaticResetPasswordStruct;

    fn reset(&self) -> &Self::Reset {
        &self.reset
    }
}

impl TestFeature {
    fn standard() -> Self {
        Self {
            reset: StaticResetPasswordStruct {
                header_infra: standard_header_infra(),
                token_infra: StaticAuthTokenStruct {
                    token_messenger: StaticAuthTokenMessenger,
                },
                request_decoder: standard_request_decoder(),
                reset_service: StaticResetPasswordService {
                    user: standard_user(),
                },
                response_encoder: StaticResetPasswordResponseEncoder,
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

fn standard_request_decoder() -> StaticResetPasswordRequestDecoder {
    StaticResetPasswordRequestDecoder {
        fields: ResetPasswordFieldsExtract {
            reset_token: "RESET-TOKEN".into(),
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
