use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::{
    auth::remote::{
        data::{RequireAuthRoles, ValidateApiTokenError},
        infra::ValidateApiTokenInfra,
        method::validate_api_token,
    },
    avail::unexpected_error::remote::notify::infra::{
        NotifyUnexpectedErrorFieldsExtract, NotifyUnexpectedErrorRequestDecoder,
    },
};

pub enum NotifyUnexpectedErrorState {
    Validate(ValidateApiTokenError),
    Notify(NotifyUnexpectedErrorEvent),
}

impl std::fmt::Display for NotifyUnexpectedErrorState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Validate(err) => err.fmt(f),
            Self::Notify(event) => event.fmt(f),
        }
    }
}

pub trait NotifyUnexpectedErrorMaterial {
    type Validate: ValidateApiTokenInfra;

    fn validate(&self) -> &Self::Validate;
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

        validate_api_token(m.validate(), RequireAuthRoles::Nothing)
            .await
            .map_err(|err| pubsub.post(NotifyUnexpectedErrorState::Validate(err)))?;

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
