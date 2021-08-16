use getto_application_test::ActionTestRunner;

use crate::auth::{
    auth_ticket::_api::kernel::init::{
        nonce_header::test::StaticAuthNonceHeader, test::StaticAuthHeaderStruct,
        token_header::test::StaticAuthTokenHeader,
    },
    password::reset::_api::request_token::init::{
        request_decoder::test::StaticRequestResetTokenRequestDecoder,
        request_token_service::test::StaticRequestResetTokenService,
        response_encoder::test::StaticRequestResetTokenResponseEncoder,
        test::StaticRequestResetTokenStruct,
    },
};

use crate::auth::password::reset::_common::request_token::infra::RequestResetTokenFieldsExtract;

use super::action::{RequestResetTokenAction, RequestResetTokenMaterial};

use crate::auth::auth_ticket::_common::kernel::data::{AuthNonceValue, AuthTokenValue};

#[tokio::test]
async fn success_request_token() {
    let (handler, assert_state) = ActionTestRunner::new();

    let feature = TestFeature::standard();

    let mut action = RequestResetTokenAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec!["request reset token"]);
    assert!(result.is_ok());
}

struct TestFeature {
    request_token: StaticRequestResetTokenStruct,
}

impl RequestResetTokenMaterial for TestFeature {
    type RequestToken = StaticRequestResetTokenStruct;

    fn request_token(&self) -> &Self::RequestToken {
        &self.request_token
    }
}

impl TestFeature {
    fn standard() -> Self {
        Self {
            request_token: StaticRequestResetTokenStruct {
                header_infra: standard_header_infra(),
                request_decoder: standard_request_decoder(),
                request_token_service: StaticRequestResetTokenService,
                response_encoder: StaticRequestResetTokenResponseEncoder,
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

fn standard_request_decoder() -> StaticRequestResetTokenRequestDecoder {
    StaticRequestResetTokenRequestDecoder {
        fields: RequestResetTokenFieldsExtract {
            login_id: "LOGIN-ID".into(),
        },
    }
}
