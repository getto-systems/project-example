use std::fmt::Display;

use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::avail::_api::notify_unexpected_error::{
    event::LogoutEvent, infra::LogoutInfra, method::logout,
};

pub enum LogoutState {
    Logout(LogoutEvent),
}

impl Display for LogoutState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Logout(event) => write!(f, "{}", event),
        }
    }
}

pub trait LogoutMaterial {
    type Logout: LogoutInfra;

    fn logout(&self) -> &Self::Logout;
}

pub struct LogoutAction<M: LogoutMaterial> {
    pubsub: ActionStatePubSub<LogoutState>,
    material: M,
}

impl<M: LogoutMaterial> LogoutAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            material,
        }
    }

    pub fn subscribe(&mut self, handler: impl 'static + Fn(&LogoutState) + Send + Sync) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(self) -> MethodResult<LogoutState> {
        let pubsub = self.pubsub;
        let m = self.material;

        logout(m.logout(), |event| pubsub.post(LogoutState::Logout(event))).await
    }
}
