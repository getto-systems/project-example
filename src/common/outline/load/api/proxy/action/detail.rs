use crate::common::outline::load::proxy::detail::LiveLoadOutlineMenuBadgeProxyInfra;

use crate::common::{
    outline::load::proxy::action::LoadOutlineMenuBadgeProxyAction, proxy::action::ProxyCallAction,
};

impl LoadOutlineMenuBadgeProxyAction {
    pub fn live(
        infra: LiveLoadOutlineMenuBadgeProxyInfra,
    ) -> ProxyCallAction<LiveLoadOutlineMenuBadgeProxyInfra> {
        ProxyCallAction::new(infra)
    }
}
