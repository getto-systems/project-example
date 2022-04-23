use std::collections::HashSet;

use getto_application_test::ActionTestRunner;

use crate::auth::{
    proxy::init::test::{StaticProxyResponse, StaticProxyService},
    ticket::validate::init::{
        auth_metadata::test::StaticAuthMetadata, test::StaticAuthMetadataStruct,
        token_decoder::test::StaticAuthTokenDecoder,
    },
};

use super::action::{AuthProxyAction, AuthProxyMaterial};

use crate::auth::ticket::kernel::data::AuthTicketExtract;

#[tokio::test]
async fn success_call_proxy() {
    let (handler, assert_state) = ActionTestRunner::new();

    let material = TestStruct::standard();

    let mut action = AuthProxyAction::with_material(material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec![
        "validate metadata success",
        "try to proxy call: test proxy",
        "proxy call success",
    ]);
    assert!(result.is_ok());
}

#[tokio::test]
async fn expired_token() {
    let (handler, assert_state) = ActionTestRunner::new();

    let material = TestStruct::expired();

    let mut action = AuthProxyAction::with_material(material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec!["validate metadata error; token expired"]);
    assert!(result.is_err());
}

struct TestStruct {
    validate: StaticAuthMetadataStruct,
    proxy_service: StaticProxyService,
}

impl<'a> AuthProxyMaterial for TestStruct {
    type Validate = StaticAuthMetadataStruct;
    type ProxyService = StaticProxyService;

    fn extract(self) -> (Self::Validate, Self::ProxyService) {
        (self.validate, self.proxy_service)
    }
}

impl TestStruct {
    fn standard() -> Self {
        Self::with_token_decoder(standard_token_decoder())
    }
    fn expired() -> Self {
        Self::with_token_decoder(expired_token_decoder())
    }
    fn with_token_decoder(token_decoder: StaticAuthTokenDecoder) -> Self {
        Self {
            validate: StaticAuthMetadataStruct {
                auth_metadata: StaticAuthMetadata {
                    nonce: NONCE.into(),
                    token: TOKEN.into(),
                },
                token_decoder,
            },
            proxy_service: StaticProxyService {
                name: "test proxy",
                response: StaticProxyResponse::Succeed("response".into()),
            },
        }
    }
}

const NONCE: &'static str = "nonce";
const TOKEN: &'static str = "token";
const TICKET_ID: &'static str = "ticket-id";

fn standard_token_decoder() -> StaticAuthTokenDecoder {
    let mut granted_roles = HashSet::new();
    granted_roles.insert("something".into());

    StaticAuthTokenDecoder::Valid(AuthTicketExtract {
        ticket_id: TICKET_ID.into(),
        user_id: "something-role-user-id".into(),
        granted_roles,
    })
}
fn expired_token_decoder() -> StaticAuthTokenDecoder {
    StaticAuthTokenDecoder::Expired
}
