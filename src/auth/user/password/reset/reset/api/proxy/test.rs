use getto_application_test::ApplicationActionStateHolder;
use pretty_assertions::assert_eq;

use crate::auth::user::password::reset::reset::proxy::init::test::StaticResetPasswordProxyMaterial;

use crate::auth::user::password::reset::reset::proxy::action::ResetPasswordProxyAction;

#[tokio::test]
async fn success_call_proxy() {
    let holder = ApplicationActionStateHolder::new();

    let material = StaticResetPasswordProxyMaterial;

    let mut action = ResetPasswordProxyAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(standard_request()).await;
    assert_eq!(
        holder.extract(),
        vec![
            "try to proxy call: auth.user.password.reset",
            "proxy call success",
        ],
    );
    assert!(result.is_ok());
}

fn standard_request() -> String {
    "REQUEST".to_owned()
}
