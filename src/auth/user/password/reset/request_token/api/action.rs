use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::{
    kernel::infra::AuthClock,
    user::password::reset::request_token::infra::{
        RegisterResetPasswordTokenRepository, RequestResetPasswordTokenConfig,
        RequestResetPasswordTokenFieldsExtract, ResetPasswordIdGenerator,
        ResetPasswordTokenEncoder, ResetPasswordTokenNotifier,
    },
};

use crate::{
    auth::{
        kernel::data::ExpireDateTime,
        user::{
            login_id::kernel::data::ValidateLoginIdError,
            password::reset::request_token::data::{
                EncodeResetTokenError, NotifyResetTokenError, NotifyResetTokenResponse,
            },
        },
    },
    common::api::repository::data::RepositoryError,
};

pub enum RequestResetTokenState {
    RequestToken(RequestResetTokenEvent),
}

impl std::fmt::Display for RequestResetTokenState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RequestToken(event) => event.fmt(f),
        }
    }
}

pub trait RequestResetTokenMaterial {
    type Clock: AuthClock;
    type ResetTokenRepository: RegisterResetPasswordTokenRepository;
    type IdGenerator: ResetPasswordIdGenerator;
    type TokenEncoder: ResetPasswordTokenEncoder;
    type TokenNotifier: ResetPasswordTokenNotifier;

    fn clock(&self) -> &Self::Clock;
    fn reset_token_repository(&self) -> &Self::ResetTokenRepository;
    fn id_generator(&self) -> &Self::IdGenerator;
    fn token_encoder(&self) -> &Self::TokenEncoder;
    fn token_notifier(&self) -> &Self::TokenNotifier;
    fn config(&self) -> &RequestResetPasswordTokenConfig;
}

pub struct RequestResetTokenAction<M: RequestResetTokenMaterial> {
    pub info: RequestResetTokenActionInfo,
    pubsub: ActionStatePubSub<RequestResetTokenState>,
    material: M,
}

pub struct RequestResetTokenActionInfo;

impl RequestResetTokenActionInfo {
    pub const fn name(&self) -> &'static str {
        "auth.user.password.reset.request-token"
    }
}

impl<M: RequestResetTokenMaterial> RequestResetTokenAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            info: RequestResetTokenActionInfo,
            pubsub: ActionStatePubSub::new(),
            material,
        }
    }

    pub fn subscribe(&mut self, handler: impl 'static + Fn(&RequestResetTokenState) + Send + Sync) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(
        self,
        fields: impl RequestResetPasswordTokenFieldsExtract,
    ) -> MethodResult<RequestResetTokenState> {
        request_reset_token(&self.material, fields, |event| {
            self.pubsub
                .post(RequestResetTokenState::RequestToken(event))
        })
        .await
    }
}

pub enum RequestResetTokenEvent {
    TokenExpiresCalculated(ExpireDateTime),
    TokenNotified(NotifyResetTokenResponse),
    Success,
    Invalid(ValidateLoginIdError),
    NotFound,
    RepositoryError(RepositoryError),
    EncodeError(EncodeResetTokenError),
    NotifyError(NotifyResetTokenError),
}

const SUCCESS: &'static str = "request reset-token success";
const ERROR: &'static str = "request reset-token error";

impl std::fmt::Display for RequestResetTokenEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TokenExpiresCalculated(expires) => {
                write!(f, "token expires calculated; {}", expires)
            }
            Self::TokenNotified(response) => {
                write!(f, "token notified; {}", response)
            }
            Self::Success => write!(f, "{}", SUCCESS),
            Self::Invalid(err) => write!(f, "{}; invalid; {}", ERROR, err),
            Self::NotFound => write!(f, "{}; not found", ERROR),
            Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
            Self::EncodeError(err) => write!(f, "{}; {}", ERROR, err),
            Self::NotifyError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

async fn request_reset_token<S>(
    infra: &impl RequestResetTokenMaterial,
    fields: impl RequestResetPasswordTokenFieldsExtract,
    post: impl Fn(RequestResetTokenEvent) -> S,
) -> MethodResult<S> {
    let fields = fields
        .convert()
        .map_err(|err| post(RequestResetTokenEvent::Invalid(err)))?;

    let (user_id, destination) = infra
        .reset_token_repository()
        .lookup_user(&fields.login_id)
        .await
        .map_err(|err| post(RequestResetTokenEvent::RepositoryError(err)))?
        .ok_or_else(|| post(RequestResetTokenEvent::NotFound))?;

    let destination = destination.ok_or_else(|| post(RequestResetTokenEvent::NotFound))?;

    let reset_password_id = infra.id_generator().generate();
    let requested_at = infra.clock().now();
    let expires = requested_at.expires(&infra.config().token_expires);

    post(RequestResetTokenEvent::TokenExpiresCalculated(
        expires.clone(),
    ));

    infra
        .reset_token_repository()
        .register_reset_token(
            reset_password_id.clone(),
            user_id,
            fields.login_id,
            destination.clone(),
            expires.clone(),
            requested_at,
        )
        .await
        .map_err(|err| post(RequestResetTokenEvent::RepositoryError(err)))?;

    let token_encoded = infra
        .token_encoder()
        .encode(reset_password_id, expires)
        .map_err(|err| post(RequestResetTokenEvent::EncodeError(err)))?;

    let notify_response = infra
        .token_notifier()
        .notify(destination, token_encoded)
        .await
        .map_err(|err| post(RequestResetTokenEvent::NotifyError(err)))?;

    post(RequestResetTokenEvent::TokenNotified(notify_response));

    Ok(post(RequestResetTokenEvent::Success))
}
