use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::{
    data::RequireAuthRoles,
    method::{check_permission, CheckPermissionEvent, CheckPermissionInfra},
};

use crate::avail::unexpected_error::notify::infra::{
    NotifyUnexpectedErrorFieldsExtract, NotifyUnexpectedErrorRequestDecoder,
};

pub enum NotifyUnexpectedErrorState {
    Validate(CheckPermissionEvent),
    Notify(NotifyUnexpectedErrorEvent),
}

impl std::fmt::Display for NotifyUnexpectedErrorState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Validate(event) => event.fmt(f),
            Self::Notify(event) => event.fmt(f),
        }
    }
}

pub trait NotifyUnexpectedErrorMaterial {
    type CheckPermission: CheckPermissionInfra;

    fn check_permission(&self) -> &Self::CheckPermission;
}

pub struct NotifyUnexpectedErrorAction<
    R: NotifyUnexpectedErrorRequestDecoder,
    M: NotifyUnexpectedErrorMaterial,
> {
    pubsub: ActionStatePubSub<NotifyUnexpectedErrorState>,
    request_decoder: R,
    material: M,
}

impl<R: NotifyUnexpectedErrorRequestDecoder, M: NotifyUnexpectedErrorMaterial>
    NotifyUnexpectedErrorAction<R, M>
{
    pub fn with_material(request_decoder: R, material: M) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            request_decoder,
            material,
        }
    }

    pub fn subscribe(
        &mut self,
        handler: impl 'static + Fn(&NotifyUnexpectedErrorState) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(self) -> MethodResult<NotifyUnexpectedErrorState> {
        let pubsub = self.pubsub;
        let m = self.material;

        let fields = self.request_decoder.decode();

        check_permission(m.check_permission(), RequireAuthRoles::Nothing, |event| {
            pubsub.post(NotifyUnexpectedErrorState::Validate(event))
        })
        .await?;

        notify_unexpected_error(&m, fields, |event| {
            pubsub.post(NotifyUnexpectedErrorState::Notify(event))
        })
        .await
    }
}

pub enum NotifyUnexpectedErrorEvent {
    Error(String),
}

impl std::fmt::Display for NotifyUnexpectedErrorEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Error(err) => write!(f, "{}", err),
        }
    }
}

async fn notify_unexpected_error<S>(
    _: &impl NotifyUnexpectedErrorMaterial,
    fields: NotifyUnexpectedErrorFieldsExtract,
    post: impl Fn(NotifyUnexpectedErrorEvent) -> S,
) -> MethodResult<S> {
    // TODO おそらくここで slack に通知とかする
    Ok(post(NotifyUnexpectedErrorEvent::Error(fields.err)))
}
