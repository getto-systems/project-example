use std::fmt::Display;

use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::auth_ticket::_api::renew::{
    event::RenewAuthTicketEvent, infra::RenewAuthTicketInfra, method::renew,
};

pub enum RenewAuthTicketState {
    Renew(RenewAuthTicketEvent),
}

impl Display for RenewAuthTicketState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Renew(event) => write!(f, "{}", event),
        }
    }
}

pub trait RenewAuthTicketMaterial {
    type Renew: RenewAuthTicketInfra;

    fn renew(&self) -> &Self::Renew;
}

pub struct RenewAuthTicketAction<M: RenewAuthTicketMaterial> {
    pubsub: ActionStatePubSub<RenewAuthTicketState>,
    material: M,
}

impl<M: RenewAuthTicketMaterial> RenewAuthTicketAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            material,
        }
    }

    pub fn subscribe(&mut self, handler: impl 'static + Fn(&RenewAuthTicketState) + Send + Sync) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(self) -> MethodResult<RenewAuthTicketState> {
        let pubsub = self.pubsub;
        let m = self.material;

        renew(m.renew(), |event| {
            pubsub.post(RenewAuthTicketState::Renew(event))
        })
        .await
    }
}
