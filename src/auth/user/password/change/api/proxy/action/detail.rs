use crate::auth::user::password::change::proxy::detail::{
    change::LiveChangePasswordProxyInfra, overwrite::LiveOverwritePasswordProxyInfra,
};

use crate::{
    auth::user::password::change::proxy::action::{
        ChangePasswordProxyAction, OverwritePasswordProxyAction,
    },
    common::proxy::action::ProxyCallAction,
};

impl ChangePasswordProxyAction {
    pub fn live(
        infra: LiveChangePasswordProxyInfra,
    ) -> ProxyCallAction<LiveChangePasswordProxyInfra> {
        ProxyCallAction::new(infra)
    }
}

impl OverwritePasswordProxyAction {
    pub fn live(
        infra: LiveOverwritePasswordProxyInfra,
    ) -> ProxyCallAction<LiveOverwritePasswordProxyInfra> {
        ProxyCallAction::new(infra)
    }
}
