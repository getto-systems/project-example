use std::collections::HashSet;

use getto_application_test::ActionTestRunner;

use crate::{
    auth::remote::init::test::{
        StaticAuthMetadata, StaticAuthTokenDecoder, StaticValidateApiTokenStruct,
        StaticValidateService,
    },
    avail::unexpected_error::_example::notify::init::{
        request_decoder::test::StaticNotifyUnexpectedErrorRequestDecoder,
        test::StaticNotifyUnexpectedErrorStruct,
    },
};

use crate::avail::unexpected_error::{
    _common::notify::infra::NotifyUnexpectedErrorFieldsExtract,
    _example::notify::infra::NotifyUnexpectedErrorRequestDecoder,
};

use super::action::{NotifyUnexpectedErrorAction, NotifyUnexpectedErrorMaterial};

#[tokio::test]
async fn success_notify() {
    let (handler, assert_state) = ActionTestRunner::new();

    let store = TestStore::standard();
    let feature = TestFeature::standard(&store);
    let request_decoder = standard_request_decoder();

    let mut action = NotifyUnexpectedErrorAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec!["UNEXPECTED-ERROR"]);
    assert!(result.is_ok());
}

struct TestFeature {
    notify: StaticNotifyUnexpectedErrorStruct,
}

impl NotifyUnexpectedErrorMaterial for TestFeature {
    type Notify = StaticNotifyUnexpectedErrorStruct;

    fn notify(&self) -> &Self::Notify {
        &self.notify
    }
}

struct TestStore {}

impl TestStore {
    fn standard() -> Self {
        Self {}
    }
}

impl TestFeature {
    fn standard(_store: &TestStore) -> Self {
        Self {
            notify: StaticNotifyUnexpectedErrorStruct {
                validate_infra: StaticValidateApiTokenStruct {
                    auth_metadata: StaticAuthMetadata {
                        nonce: "NONCE".into(),
                        token: "TOKEN".into(),
                    },
                    token_decoder: StaticAuthTokenDecoder::valid(
                        "TICKET-ID".into(),
                        "USER-ID".into(),
                        HashSet::new(),
                    ),
                    validate_service: StaticValidateService::new("USER-ID".into()),
                },
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
