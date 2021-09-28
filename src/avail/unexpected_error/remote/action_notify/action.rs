use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::avail::unexpected_error::remote::notify::{
    event::NotifyUnexpectedErrorEvent,
    infra::{NotifyUnexpectedErrorInfra, NotifyUnexpectedErrorRequestDecoder},
    method::notify_unexpected_error,
};

pub enum NotifyUnexpectedErrorState {
    Notify(NotifyUnexpectedErrorEvent),
}

impl std::fmt::Display for NotifyUnexpectedErrorState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Notify(event) => event.fmt(f),
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

        let fields = request_decoder.decode();

        notify_unexpected_error(m.notify(), fields, |event| {
            pubsub.post(NotifyUnexpectedErrorState::Notify(event))
        })
        .await
    }
}
