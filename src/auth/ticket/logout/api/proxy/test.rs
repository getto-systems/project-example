use getto_application_test::ApplicationActionStateHolder;
use pretty_assertions::assert_eq;

use crate::auth::ticket::{
    authenticate::init::test::StaticAuthenticateWithTokenInfra,
    kernel::init::{
        request::test::StaticAuthenticateToken,
        token::authenticate::decoder::test::StaticAuthenticateTokenDecoder,
    },
    logout::proxy::init::test::StaticLogoutProxyMaterial,
};

use crate::auth::ticket::logout::proxy::action::LogoutProxyAction;

use crate::auth::ticket::kernel::data::AuthTicket;

#[tokio::test]
async fn success_call_proxy() {
    let holder = ApplicationActionStateHolder::new();

    let material = StaticLogoutProxyMaterial {
        authenticate_with_token: StaticAuthenticateWithTokenInfra {
            token_decoder: standard_token_decoder(),
        },
    };

    let mut action = LogoutProxyAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(StaticAuthenticateToken).await;
    assert_eq!(
        holder.extract(),
        vec![
            "authenticate with token success; ticket: ticket-id / user-id: user-id (granted: [])",
            "try to proxy call: auth.ticket.logout",
            "proxy call success",
        ],
    );
    assert!(result.is_ok());
}

fn standard_token_decoder() -> StaticAuthenticateTokenDecoder {
    StaticAuthenticateTokenDecoder::Valid(AuthTicket::standard())
}
