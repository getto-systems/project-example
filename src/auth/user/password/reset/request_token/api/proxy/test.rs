use getto_application_test::ApplicationActionStateHolder;
use pretty_assertions::assert_eq;

use crate::auth::user::password::reset::request_token::proxy::init::test::StaticRequestResetTokenProxyMaterial;

use crate::auth::user::password::reset::request_token::proxy::action::RequestResetTokenProxyAction;

#[tokio::test]
async fn success_call_proxy() {
    let holder = ApplicationActionStateHolder::new();

    let material = StaticRequestResetTokenProxyMaterial;

    let mut action = RequestResetTokenProxyAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(standard_request()).await;
    assert_eq!(
        holder.extract(),
        vec![
            "try to proxy call: auth.user.password.reset.request-token",
            "proxy call success",
        ],
    );
    assert!(result.is_ok());
}

fn standard_request() -> String {
    "REQUEST".to_owned()
}
