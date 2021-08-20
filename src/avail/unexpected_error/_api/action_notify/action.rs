use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::avail::unexpected_error::_api::notify::{
    event::NotifyUnexpectedErrorEvent,
    infra::{NotifyUnexpectedErrorInfra, NotifyUnexpectedErrorRequestDecoder},
    method::notify_unexpected_error,
};

use crate::z_details::_api::message::data::MessageError;

pub enum NotifyUnexpectedErrorState {
    Notify(NotifyUnexpectedErrorEvent),
    MessageError(MessageError),
}

impl std::fmt::Display for NotifyUnexpectedErrorState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Notify(event) => write!(f, "{}", event),
            Self::MessageError(err) => write!(f, "{}", err),
        }
    }
}

pub trait NotifyUnexpectedErrorMaterial {
    type Notify: NotifyUnexpectedErrorInfra;

    fn notify(&self) -> &Self::Notify;
}

pub struct NotifyUnexpectedErrorAction<M: NotifyUnexpectedErrorMaterial> {
    pubsub: ActionStatePubSub<NotifyUnexpectedErrorState>,
    material: M,
}

impl<M: NotifyUnexpectedErrorMaterial> NotifyUnexpectedErrorAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            material,
        }
    }

    pub fn subscribe(
        &mut self,
        handler: impl 'static + Fn(&NotifyUnexpectedErrorState) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(
        self,
        request_decoder: impl NotifyUnexpectedErrorRequestDecoder,
    ) -> MethodResult<NotifyUnexpectedErrorState> {
        let pubsub = self.pubsub;
        let m = self.material;

        let err = request_decoder
            .decode()
            .map_err(|err| pubsub.post(NotifyUnexpectedErrorState::MessageError(err)))?;

        notify_unexpected_error(m.notify(), err, |event| {
            pubsub.post(NotifyUnexpectedErrorState::Notify(event))
        })
        .await
    }
}
