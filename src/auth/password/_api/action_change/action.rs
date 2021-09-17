use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::password::_api::change::{
    event::ChangePasswordEvent,
    infra::{ChangePasswordInfra, ChangePasswordRequestDecoder},
    method::change_password,
};

use crate::z_details::_api::message::data::MessageError;

pub enum ChangePasswordState {
    Change(ChangePasswordEvent),
    MessageError(MessageError),
}

impl std::fmt::Display for ChangePasswordState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Change(event) => write!(f, "{}", event),
            Self::MessageError(err) => {
                write!(f, "change password error; message error: {}", err)
            }
        }
    }
}

pub trait ChangePasswordMaterial {
    type Change: ChangePasswordInfra;

    fn change(&self) -> &Self::Change;
}

pub struct ChangePasswordAction<M: ChangePasswordMaterial> {
    pubsub: ActionStatePubSub<ChangePasswordState>,
    material: M,
}

impl<M: ChangePasswordMaterial> ChangePasswordAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            material,
        }
    }

    pub fn subscribe(&mut self, handler: impl 'static + Fn(&ChangePasswordState) + Send + Sync) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(
        self,
        request_decoder: impl ChangePasswordRequestDecoder,
    ) -> MethodResult<ChangePasswordState> {
        let pubsub = self.pubsub;
        let m = self.material;

        let fields = request_decoder
            .decode()
            .map_err(ChangePasswordState::MessageError)?;

        change_password(m.change(), fields, |event| {
            pubsub.post(ChangePasswordState::Change(event))
        })
        .await
    }
}
