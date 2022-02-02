use std::collections::HashSet;

use getto_application_test::ActionTestRunner;

use crate::{
    auth::init::test::{
        StaticAuthMetadata, StaticAuthTokenDecoder, StaticValidateApiTokenStruct,
        StaticValidateService,
    },
    avail::unexpected_error::remote::{
        notify::infra::NotifyUnexpectedErrorRequestDecoder,
        notify::init::request_decoder::test::StaticNotifyUnexpectedErrorRequestDecoder,
    },
};

use crate::avail::unexpected_error::remote::notify::infra::NotifyUnexpectedErrorFieldsExtract;

use super::action::{NotifyUnexpectedErrorAction, NotifyUnexpectedErrorMaterial};

#[tokio::test]
async fn success_notify() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let material = TestStruct::standard(&store);
    let request_decoder = standard_request_decoder();

    let mut action = NotifyUnexpectedErrorAction::with_material(request_decoder, material);
    action.subscribe(handler);

    let result = action.ignite().await;
    assert_state(vec!["validate api token success", "UNEXPECTED-ERROR"]);
    assert!(result.is_ok());
}

struct TestStruct {
    validate: StaticValidateApiTokenStruct,
}

impl NotifyUnexpectedErrorMaterial for TestStruct {
    type CheckPermission = StaticValidateApiTokenStruct;

    fn check_permission(&self) -> &Self::CheckPermission {
        &self.validate
    }
}

struct TestStore {}

impl TestStore {
    fn standard() -> Self {
        Self {}
    }
}

impl TestStruct {
    fn standard(_store: &TestStore) -> Self {
        Self {
            validate: StaticValidateApiTokenStruct {
                auth_metadata: StaticAuthMetadata {
                    nonce: "NONCE".into(),
                    token: "TOKEN".into(),
                },
                token_decoder: StaticAuthTokenDecoder::valid(
                    "TICKET-ID".into(),
                    "USER-ID".into(),
                    HashSet::new(),
                ),
                validate_service: StaticValidateService,
            },
        }
    }
}

fn standard_request_decoder() -> impl NotifyUnexpectedErrorRequestDecoder {
    StaticNotifyUnexpectedErrorRequestDecoder {
        fields: NotifyUnexpectedErrorFieldsExtract {
            err: "UNEXPECTED-ERROR".into(),
        },
    }
}
