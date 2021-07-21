use std::fmt::Display;

use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::password::_api::authenticate::{
    event::AuthenticatePasswordEvent, infra::AuthenticatePasswordInfra,
    method::authenticate_password,
};

pub enum AuthenticatePasswordState {
    Authenticate(AuthenticatePasswordEvent),
}

impl Display for AuthenticatePasswordState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authenticate(event) => write!(f, "{}", event),
        }
    }
}

pub trait AuthenticatePasswordMaterial {
    type Authenticate: AuthenticatePasswordInfra;

    fn authenticate(&self) -> &Self::Authenticate;
}

pub struct AuthenticatePasswordAction<M: AuthenticatePasswordMaterial> {
    pubsub: ActionStatePubSub<AuthenticatePasswordState>,
    material: M,
}

impl<M: AuthenticatePasswordMaterial> AuthenticatePasswordAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            material,
        }
    }

    pub fn subscribe(
        &mut self,
        handler: impl 'static + Fn(&AuthenticatePasswordState) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(self) -> MethodResult<AuthenticatePasswordState> {
        let pubsub = self.pubsub;
        let m = self.material;

        authenticate_password(m.authenticate(), |event| {
            pubsub.post(AuthenticatePasswordState::Authenticate(event))
        })
        .await
    }
}
