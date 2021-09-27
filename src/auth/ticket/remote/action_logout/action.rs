use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::ticket::remote::discard::{
    event::DiscardAuthTicketEvent, infra::DiscardAuthTicketInfra, method::discard_auth_ticket,
};

pub enum LogoutState {
    Discard(DiscardAuthTicketEvent),
}

impl std::fmt::Display for LogoutState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Discard(event) => write!(f, "{}", event),
        }
    }
}

pub trait LogoutMaterial {
    type Discard: DiscardAuthTicketInfra;

    fn discard(&self) -> &Self::Discard;
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

        discard_auth_ticket(m.discard(), |event| {
            pubsub.post(LogoutState::Discard(event))
        })
        .await
    }
}
