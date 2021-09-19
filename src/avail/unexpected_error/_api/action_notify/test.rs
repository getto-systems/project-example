use getto_application_test::ActionTestRunner;

use crate::{
    auth::_common::init::test::{
        StaticAuthNonceMetadata, StaticAuthTokenMetadata, StaticValidateApiTokenStruct,
        StaticValidateService,
    },
    avail::unexpected_error::_api::notify::init::{
        request_decoder::test::StaticNotifyUnexpectedErrorRequestDecoder,
        test::StaticNotifyUnexpectedErrorStruct,
    },
};

use super::action::{NotifyUnexpectedErrorAction, NotifyUnexpectedErrorMaterial};

#[tokio::test]
async fn success_logout() {
    let (handler, assert_state) = ActionTestRunner::new();

    let feature = TestFeature::standard();
    let request_decoder = standard_request_decoder();

    let mut action = NotifyUnexpectedErrorAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec!["authorized; user: USER-ID", "ERROR MESSAGE"]);
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

impl TestFeature {
    fn standard() -> Self {
        Self {
            notify: StaticNotifyUnexpectedErrorStruct {
                validate_infra: StaticValidateApiTokenStruct {
                    nonce_metadata: StaticAuthNonceMetadata::new("NONCE".into()),
                    token_metadata: StaticAuthTokenMetadata::new("TOKEN".into()),
                    validate_service: StaticValidateService::new("USER-ID".into()),
                },
            },
        }
    }
}

fn standard_request_decoder() -> StaticNotifyUnexpectedErrorRequestDecoder {
    StaticNotifyUnexpectedErrorRequestDecoder {
        err: "ERROR MESSAGE".into(),
    }
}
