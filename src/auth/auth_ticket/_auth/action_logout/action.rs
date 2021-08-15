use std::fmt::Display;

use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::auth_ticket::_auth::{
    discard::{
        event::DiscardAuthTicketEvent, infra::DiscardAuthTicketInfra, method::discard_auth_ticket,
    },
    validate::{
        event::ValidateAuthTokenEvent, infra::ValidateAuthTokenInfra, method::validate_auth_token,
    },
};

pub enum LogoutState {
    Validate(ValidateAuthTokenEvent),
    Discard(DiscardAuthTicketEvent),
}

impl Display for LogoutState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Validate(event) => write!(f, "{}", event),
            Self::Discard(event) => write!(f, "{}", event),
        }
    }
}

pub trait LogoutMaterial {
    type Validate: ValidateAuthTokenInfra;
    type Discard: DiscardAuthTicketInfra;

    fn extract(self) -> (Self::Validate, Self::Discard);
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
        let (validate, discard) = self.material.extract();

        let ticket =
            validate_auth_token(&validate, |event| pubsub.post(LogoutState::Validate(event)))
                .await?;

        discard_auth_ticket(&discard, ticket, |event| {
            pubsub.post(LogoutState::Discard(event))
        })
        .await
    }
}
