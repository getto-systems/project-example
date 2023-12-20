use crate::avail::unexpected_error::notify::proxy::detail::LiveNotifyUnexpectedErrorProxyInfra;

use crate::{
    avail::unexpected_error::notify::proxy::action::NotifyUnexpectedErrorProxyAction,
    common::proxy::action::ProxyCallAction,
};

impl NotifyUnexpectedErrorProxyAction {
    pub fn live(
        infra: LiveNotifyUnexpectedErrorProxyInfra,
    ) -> ProxyCallAction<LiveNotifyUnexpectedErrorProxyInfra> {
        ProxyCallAction::new(infra)
    }
}
