use getto_application_test::ApplicationActionStateHolder;
use pretty_assertions::assert_eq;

use crate::auth::ticket::{
    authenticate::{
        init::test::StaticAuthenticateWithTokenInfra,
        proxy::init::test::StaticAuthenticateWithTokenProxyMaterial,
    },
    kernel::init::{
        request::test::StaticAuthenticateToken,
        token::authenticate::decoder::test::StaticAuthenticateTokenDecoder,
    },
};

use crate::auth::ticket::authenticate::proxy::action::AuthenticateWithTokenProxyAction;

use crate::auth::ticket::kernel::data::AuthTicket;

#[tokio::test]
async fn success_call_proxy() {
    let holder = ApplicationActionStateHolder::new();

    let material = StaticAuthenticateWithTokenProxyMaterial {
        authenticate_with_token: StaticAuthenticateWithTokenInfra {
            token_decoder: standard_token_decoder(),
        },
    };

    let mut action = AuthenticateWithTokenProxyAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(StaticAuthenticateToken).await;
    assert_eq!(
        holder.extract(),
        vec![
            "authenticate with token success; ticket: ticket-id / user-id: user-id (granted: [])",
            "try to proxy call: auth.ticket.authenticate",
            "proxy call success",
        ],
    );
    assert!(result.is_ok());
}

fn standard_token_decoder() -> StaticAuthenticateTokenDecoder {
    StaticAuthenticateTokenDecoder::Valid(AuthTicket::standard())
}
