use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::ticket::{
    encode::method::{encode_auth_ticket, EncodeAuthTicketEvent, EncodeAuthTicketInfra},
    validate::method::{authenticate, AuthenticateEvent, AuthenticateInfra},
};

pub enum CheckAuthTicketState {
    Authenticate(AuthenticateEvent),
    Encode(EncodeAuthTicketEvent),
}

impl std::fmt::Display for CheckAuthTicketState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authenticate(event) => write!(f, "{}", event),
            Self::Encode(event) => write!(f, "{}", event),
        }
    }
}

pub trait CheckAuthTicketMaterial {
    type Authenticate: AuthenticateInfra;
    type Encode: EncodeAuthTicketInfra;

    fn authenticate(&self) -> &Self::Authenticate;
    fn encode(&self) -> &Self::Encode;
}

pub struct CheckAuthTicketAction<M: CheckAuthTicketMaterial> {
    pubsub: ActionStatePubSub<CheckAuthTicketState>,
    material: M,
}

impl<M: CheckAuthTicketMaterial> CheckAuthTicketAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            material,
        }
    }

    pub fn subscribe(&mut self, handler: impl 'static + Fn(&CheckAuthTicketState) + Send + Sync) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(self) -> MethodResult<CheckAuthTicketState> {
        let pubsub = self.pubsub;
        let m = self.material;

        let ticket = authenticate(m.authenticate(), |event| {
            pubsub.post(CheckAuthTicketState::Authenticate(event))
        })
        .await?;

        encode_auth_ticket(m.encode(), ticket, |event| {
            pubsub.post(CheckAuthTicketState::Encode(event))
        })
        .await
    }
}
