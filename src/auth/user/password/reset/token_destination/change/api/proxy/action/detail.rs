use crate::auth::user::password::reset::token_destination::change::proxy::detail::LiveChangeResetTokenDestinationProxyInfra;

use crate::{
    auth::user::password::reset::token_destination::change::proxy::action::ChangeResetTokenDestinationProxyAction,
    common::proxy::action::ProxyCallAction,
};

impl ChangeResetTokenDestinationProxyAction {
    pub fn live(
        infra: LiveChangeResetTokenDestinationProxyInfra,
    ) -> ProxyCallAction<LiveChangeResetTokenDestinationProxyInfra> {
        ProxyCallAction::new(infra)
    }
}
