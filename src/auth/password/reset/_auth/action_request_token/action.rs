use std::fmt::Display;

use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::password::reset::_auth::request_token::{
    event::RequestResetTokenEvent, infra::RequestResetTokenInfra, method::request_reset_token,
};

pub enum RequestResetTokenState {
    RequestToken(RequestResetTokenEvent),
}

impl Display for RequestResetTokenState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RequestToken(event) => write!(f, "{}", event),
        }
    }
}

pub trait RequestResetTokenMaterial {
    type RequestToken: RequestResetTokenInfra;

    fn extract(self) -> Self::RequestToken;
}

pub struct RequestResetTokenAction<M: RequestResetTokenMaterial> {
    pubsub: ActionStatePubSub<RequestResetTokenState>,
    material: M,
}

impl<M: RequestResetTokenMaterial> RequestResetTokenAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            material,
        }
    }

    pub fn subscribe(&mut self, handler: impl 'static + Fn(&RequestResetTokenState) + Send + Sync) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(self) -> MethodResult<RequestResetTokenState> {
        let pubsub = self.pubsub;
        let request_token = self.material.extract();

        request_reset_token(request_token, |event| {
            pubsub.post(RequestResetTokenState::RequestToken(event))
        })
        .await
    }
}
