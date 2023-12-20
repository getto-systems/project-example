use crate::auth::user::account::register::proxy::detail::LiveRegisterAuthUserAccountProxyInfra;

use crate::{
    auth::user::account::register::proxy::action::RegisterAuthUserAccountProxyAction,
    common::proxy::action::ProxyCallAction,
};

impl RegisterAuthUserAccountProxyAction {
    pub fn live(
        infra: LiveRegisterAuthUserAccountProxyInfra,
    ) -> ProxyCallAction<LiveRegisterAuthUserAccountProxyInfra> {
        ProxyCallAction::new(infra)
    }
}
