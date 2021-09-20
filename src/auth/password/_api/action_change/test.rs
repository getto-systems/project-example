use getto_application_test::ActionTestRunner;

use crate::auth::{
    auth_ticket::_common::kernel::init::service_metadata::test::StaticAuthServiceMetadata,
    password::_api::change::init::{
        change_service::test::StaticChangePasswordService,
        request_decoder::test::StaticChangePasswordRequestDecoder,
        response_encoder::test::StaticChangePasswordResponseEncoder,
        test::StaticChangePasswordStruct,
    },
};

use crate::auth::password::_common::change::infra::ChangePasswordFieldsExtract;

use super::action::{ChangePasswordAction, ChangePasswordMaterial};

#[tokio::test]
async fn success_change() {
    let (handler, assert_state) = ActionTestRunner::new();

    let feature = TestFeature::standard();
    let request_decoder = standard_request_decoder();

    let mut action = ChangePasswordAction::with_material(feature);
    action.subscribe(handler);

    let result = action.ignite(request_decoder).await;
    assert_state(vec!["change password"]);
    assert!(result.is_ok());
}

struct TestFeature {
    change: StaticChangePasswordStruct,
}

impl ChangePasswordMaterial for TestFeature {
    type Change = StaticChangePasswordStruct;

    fn change(&self) -> &Self::Change {
        &self.change
    }
}

impl<'a> TestFeature {
    fn standard() -> Self {
        Self {
            change: StaticChangePasswordStruct {
                service_metadata: StaticAuthServiceMetadata {
                    nonce: "NONCE".into(),
                    token: "TOKEN".into(),
                },
                change_service: StaticChangePasswordService,
                response_encoder: StaticChangePasswordResponseEncoder,
            },
        }
    }
}

fn standard_request_decoder() -> StaticChangePasswordRequestDecoder {
    StaticChangePasswordRequestDecoder {
        fields: ChangePasswordFieldsExtract {
            current_password: "CURRENT-PASSWORD".into(),
            new_password: "NEW-PASSWORD".into(),
        },
    }
}
