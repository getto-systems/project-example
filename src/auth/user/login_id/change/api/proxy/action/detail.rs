use crate::auth::user::login_id::change::proxy::detail::LiveOverwriteLoginIdProxyInfra;

use crate::{
    auth::user::login_id::change::proxy::action::OverwriteLoginIdProxyAction,
    common::proxy::action::ProxyCallAction,
};

impl OverwriteLoginIdProxyAction {
    pub fn live(
        infra: LiveOverwriteLoginIdProxyInfra,
    ) -> ProxyCallAction<LiveOverwriteLoginIdProxyInfra> {
        ProxyCallAction::new(infra)
    }
}
