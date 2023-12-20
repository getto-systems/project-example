use crate::auth::user::account::search::proxy::detail::LiveSearchAuthUserAccountProxyInfra;

use crate::{
    auth::user::account::search::proxy::action::SearchAuthUserAccountProxyAction,
    common::proxy::action::ProxyCallAction,
};

impl SearchAuthUserAccountProxyAction {
    pub fn live(
        infra: LiveSearchAuthUserAccountProxyInfra,
    ) -> ProxyCallAction<LiveSearchAuthUserAccountProxyInfra> {
        ProxyCallAction::new(infra)
    }
}
