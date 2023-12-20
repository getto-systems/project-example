use crate::auth::user::account::unregister::proxy::detail::LiveUnregisterAuthUserAccountProxyInfra;

use crate::{
    auth::user::account::unregister::proxy::action::UnregisterAuthUserAccountProxyAction,
    common::proxy::action::ProxyCallAction,
};

impl UnregisterAuthUserAccountProxyAction {
    pub fn live(
        infra: LiveUnregisterAuthUserAccountProxyInfra,
    ) -> ProxyCallAction<LiveUnregisterAuthUserAccountProxyInfra> {
        ProxyCallAction::new(infra)
    }
}
