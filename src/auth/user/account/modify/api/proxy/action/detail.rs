use crate::auth::user::account::modify::proxy::detail::LiveModifyAuthUserAccountProxyInfra;

use crate::{
    auth::user::account::modify::proxy::action::ModifyAuthUserAccountProxyAction,
    common::proxy::action::ProxyCallAction,
};

impl ModifyAuthUserAccountProxyAction {
    pub fn live(
        infra: LiveModifyAuthUserAccountProxyInfra,
    ) -> ProxyCallAction<LiveModifyAuthUserAccountProxyInfra> {
        ProxyCallAction::new(infra)
    }
}
