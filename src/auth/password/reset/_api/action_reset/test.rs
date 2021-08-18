use std::collections::HashSet;

use getto_application_test::ActionTestRunner;

use crate::auth::{
    auth_ticket::_api::kernel::init::{
        nonce_header::test::StaticAuthNonceHeader,
        response_builder::test::StaticAuthTokenResponseBuilder,
        token_header::test::StaticAuthTokenHeader,
    },
    password::reset::_api::reset::init::{
        request_decoder::test::StaticResetPasswordRequestDecoder,
        reset_service::test::StaticResetPasswordService,
        response_encoder::test::StaticResetPasswordResponseEncoder,
        test::StaticResetPasswordStruct,
    },
};

use crate::auth::password::reset::_common::reset::infra::ResetPasswordFieldsExtract;

use super::action::{ResetPasswordAction, ResetPasswordMaterial};

use crate::auth::{
    auth_ticket::_common::kernel::data::{AuthNonce, AuthToken},
    auth_user::_common::kernel::data::{AuthUser, AuthUserExtract},
};

#[tokio::test]
async fn success_request_token() {
    let (handler, assert_state) = ActionTestRunner::new();

    let feature = TestFeature::standard();
    let request_decoder = standard_request_decoder();

    let mut action = ResetPasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec!["reset password"]);
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
                nonce_header: StaticAuthNonceHeader::Valid(AuthNonce::new("NONCE".into())),
                token_header: StaticAuthTokenHeader::Valid(AuthToken::new("TOKEN".into())),
                reset_service: StaticResetPasswordService {
                    user: standard_user(),
                },
                response_encoder: StaticResetPasswordResponseEncoder,
                response_builder: StaticAuthTokenResponseBuilder,
            },
        }
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
