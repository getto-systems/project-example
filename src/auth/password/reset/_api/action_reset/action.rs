use std::fmt::Display;

use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::password::reset::_api::reset::infra::ResetPasswordRequestDecoder;
use crate::auth::password::reset::_api::reset::{
    event::ResetPasswordEvent, infra::ResetPasswordInfra, method::reset_password,
};

use crate::z_details::_api::message::data::MessageError;

pub enum ResetPasswordState {
    Reset(ResetPasswordEvent),
    MessageError(MessageError),
}

impl Display for ResetPasswordState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reset(event) => write!(f, "{}", event),
            Self::MessageError(err) => write!(f, "reset password error; message error: {}", err),
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

    pub async fn ignite(
        self,
        request_decoder: impl ResetPasswordRequestDecoder,
    ) -> MethodResult<ResetPasswordState> {
        let pubsub = self.pubsub;
        let m = self.material;

        let fields = request_decoder
            .decode()
            .map_err(ResetPasswordState::MessageError)?;

        reset_password(m.reset(), fields, |event| {
            pubsub.post(ResetPasswordState::Reset(event))
        })
        .await
    }
}
