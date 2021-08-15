use std::fmt::Display;

use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::auth_ticket::_auth::{
    encode::{
        event::EncodeAuthTicketEvent, infra::EncodeAuthTicketInfra, method::encode_auth_ticket,
    },
    validate::{
        event::ValidateAuthTokenEvent, infra::ValidateAuthTokenInfra, method::validate_auth_token,
    },
};

pub enum RenewAuthTicketState {
    Validate(ValidateAuthTokenEvent),
    Encode(EncodeAuthTicketEvent),
}

impl Display for RenewAuthTicketState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Validate(event) => write!(f, "{}", event),
            Self::Encode(event) => write!(f, "{}", event),
        }
    }
}

pub trait RenewAuthTicketMaterial {
    type Validate: ValidateAuthTokenInfra;
    type Encode: EncodeAuthTicketInfra;

    fn extract(self) -> (Self::Validate, Self::Encode);
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
        let (validate, encode) = self.material.extract();

        let ticket = validate_auth_token(&validate, |event| {
            pubsub.post(RenewAuthTicketState::Validate(event))
        })
        .await?;

        encode_auth_ticket(&encode, ticket, |event| {
            pubsub.post(RenewAuthTicketState::Encode(event))
        })
        .await
    }
}
