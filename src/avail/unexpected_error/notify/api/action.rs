use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::common::proxy::action::CoreProxyParams;

use crate::auth::method::proxy::{authorize, AuthorizeEvent, AuthorizeInfra};

use crate::avail::unexpected_error::notify::infra::NotifyUnexpectedErrorFieldsExtract;

use crate::auth::data::{AuthPermissionRequired, AuthorizeTokenExtract};

pub enum NotifyUnexpectedErrorState {
    Authorize(AuthorizeEvent),
    Notify(NotifyUnexpectedErrorEvent),
}

impl std::fmt::Display for NotifyUnexpectedErrorState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authorize(event) => event.fmt(f),
            Self::Notify(event) => event.fmt(f),
        }
    }
}

pub trait NotifyUnexpectedErrorMaterial {
    type Authorize: AuthorizeInfra;

    fn authorize(&self) -> &Self::Authorize;
}

pub struct NotifyUnexpectedErrorAction<M: NotifyUnexpectedErrorMaterial> {
    pub info: NotifyUnexpectedErrorActionInfo,
    pubsub: ActionStatePubSub<NotifyUnexpectedErrorState>,
    material: M,
}

pub struct NotifyUnexpectedErrorActionInfo;

impl NotifyUnexpectedErrorActionInfo {
    pub const fn name(&self) -> &'static str {
        "avail.unexpected-error.notify"
    }

    pub fn required(&self) -> AuthPermissionRequired {
        AuthPermissionRequired::Nothing
    }

    pub fn params(&self) -> CoreProxyParams {
        (self.name(), self.required())
    }
}

impl<M: NotifyUnexpectedErrorMaterial> NotifyUnexpectedErrorAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            info: NotifyUnexpectedErrorActionInfo,
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
        token: impl AuthorizeTokenExtract,
        fields: impl NotifyUnexpectedErrorFieldsExtract,
    ) -> MethodResult<NotifyUnexpectedErrorState> {
        authorize(
            self.material.authorize(),
            (token, self.info.required()),
            |event| {
                self.pubsub
                    .post(NotifyUnexpectedErrorState::Authorize(event))
            },
        )
        .await?;

        notify_unexpected_error(fields, |event| {
            self.pubsub.post(NotifyUnexpectedErrorState::Notify(event))
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
    fields: impl NotifyUnexpectedErrorFieldsExtract,
    post: impl Fn(NotifyUnexpectedErrorEvent) -> S,
) -> MethodResult<S> {
    // TODO おそらくここで slack に通知とかする
    Ok(post(NotifyUnexpectedErrorEvent::Error(
        fields.convert().err,
    )))
}
