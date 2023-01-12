use getto_application_test::ApplicationActionStateHolder;
use pretty_assertions::assert_eq;

use crate::auth::user::password::authenticate::proxy::init::test::StaticAuthenticateWithPasswordProxyMaterial;

use crate::auth::user::password::authenticate::proxy::action::AuthenticateWithPasswordProxyAction;

#[tokio::test]
async fn success_call_proxy() {
    let holder = ApplicationActionStateHolder::new();

    let material = StaticAuthenticateWithPasswordProxyMaterial;

    let mut action = AuthenticateWithPasswordProxyAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(standard_request()).await;
    assert_eq!(
        holder.extract(),
        vec![
            "try to proxy call: auth.user.password.authenticate",
            "proxy call success",
        ],
    );
    assert!(result.is_ok());
}

fn standard_request() -> String {
    "REQUEST".to_owned()
}
