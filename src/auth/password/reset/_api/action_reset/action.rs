use std::fmt::Display;

use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::password::reset::_api::reset::{
    event::ResetPasswordEvent, infra::ResetPasswordInfra, method::reset_password,
};

pub enum ResetPasswordState {
    Reset(ResetPasswordEvent),
}

impl Display for ResetPasswordState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reset(event) => write!(f, "{}", event),
        }
    }
}

pub trait ResetPasswordMaterial {
    type Reset: ResetPasswordInfra;

    fn reset(&self) -> &Self::Reset;
}

pub struct ResetPasswordAction<M: ResetPasswordMaterial> {
    pubsub: ActionStatePubSub<ResetPasswordState>,
    material: M,
}

impl<M: ResetPasswordMaterial> ResetPasswordAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            material,
        }
    }

    pub fn subscribe(&mut self, handler: impl 'static + Fn(&ResetPasswordState) + Send + Sync) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(self) -> MethodResult<ResetPasswordState> {
        let pubsub = self.pubsub;
        let m = self.material;

        reset_password(m.reset(), |event| {
            pubsub.post(ResetPasswordState::Reset(event))
        })
        .await
    }
}
