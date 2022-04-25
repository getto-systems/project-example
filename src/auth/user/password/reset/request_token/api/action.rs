use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::ticket::validate::method::{
    validate_auth_nonce, ValidateAuthNonceEvent, ValidateAuthNonceInfra,
};

use crate::auth::{
    ticket::kernel::infra::AuthClock,
    user::password::reset::request_token::infra::{
        RegisterResetTokenRepository, RequestResetTokenConfig, RequestResetTokenFields,
        RequestResetTokenFieldsExtract, RequestResetTokenRequestDecoder, ResetTokenEncoder,
        ResetTokenGenerator, ResetTokenNotifier,
    },
};

use crate::{
    auth::{
        ticket::kernel::data::ExpireDateTime,
        user::password::reset::request_token::data::{
            EncodeResetTokenError, NotifyResetTokenError, NotifyResetTokenResponse,
            ValidateRequestResetTokenFieldsError,
        },
    },
    z_lib::repository::data::RepositoryError,
};

pub enum RequestResetTokenState {
    ValidateNonce(ValidateAuthNonceEvent),
    RequestToken(RequestResetTokenEvent),
}

impl std::fmt::Display for RequestResetTokenState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ValidateNonce(event) => event.fmt(f),
            Self::RequestToken(event) => event.fmt(f),
        }
    }
}

pub trait RequestResetTokenMaterial {
    type ValidateNonce: ValidateAuthNonceInfra;

    type Clock: AuthClock;
    type ResetTokenRepository: RegisterResetTokenRepository;
    type TokenGenerator: ResetTokenGenerator;
    type TokenEncoder: ResetTokenEncoder;
    type TokenNotifier: ResetTokenNotifier;

    fn validate_nonce(&self) -> &Self::ValidateNonce;

    fn clock(&self) -> &Self::Clock;
    fn reset_token_repository(&self) -> &Self::ResetTokenRepository;
    fn token_generator(&self) -> &Self::TokenGenerator;
    fn token_encoder(&self) -> &Self::TokenEncoder;
    fn token_notifier(&self) -> &Self::TokenNotifier;
    fn config(&self) -> &RequestResetTokenConfig;
}

pub struct RequestResetTokenAction<R: RequestResetTokenRequestDecoder, M: RequestResetTokenMaterial>
{
    pubsub: ActionStatePubSub<RequestResetTokenState>,
    request_decoder: R,
    material: M,
}

impl<R: RequestResetTokenRequestDecoder, M: RequestResetTokenMaterial>
    RequestResetTokenAction<R, M>
{
    pub fn with_material(request_decoder: R, material: M) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            request_decoder,
            material,
        }
    }

    pub fn subscribe(&mut self, handler: impl 'static + Fn(&RequestResetTokenState) + Send + Sync) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(self) -> MethodResult<RequestResetTokenState> {
        let pubsub = self.pubsub;
        let m = self.material;

        let fields = self.request_decoder.decode();

        validate_auth_nonce(m.validate_nonce(), |event| {
            pubsub.post(RequestResetTokenState::ValidateNonce(event))
        })
        .await?;

        request_reset_token(&m, fields, |event| {
            pubsub.post(RequestResetTokenState::RequestToken(event))
        })
        .await
    }
}

pub enum RequestResetTokenEvent {
    TokenExpiresCalculated(ExpireDateTime),
    TokenNotified(NotifyResetTokenResponse),
    Success,
    Invalid(ValidateRequestResetTokenFieldsError),
    NotFound,
    RepositoryError(RepositoryError),
    EncodeError(EncodeResetTokenError),
    NotifyError(NotifyResetTokenError),
}

const SUCCESS: &'static str = "request reset token success";
const ERROR: &'static str = "request reset token error";

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
    fields: RequestResetTokenFieldsExtract,
    post: impl Fn(RequestResetTokenEvent) -> S,
) -> MethodResult<S> {
    let fields = RequestResetTokenFields::convert(fields)
        .map_err(|err| post(RequestResetTokenEvent::Invalid(err)))?;

    let reset_token_repository = infra.reset_token_repository();
    let token_generator = infra.token_generator();
    let token_encoder = infra.token_encoder();
    let token_notifier = infra.token_notifier();
    let config = infra.config();

    let (user_id, destination) = reset_token_repository
        .lookup_user(&fields.login_id)
        .await
        .map_err(|err| post(RequestResetTokenEvent::RepositoryError(err)))?
        .ok_or_else(|| post(RequestResetTokenEvent::NotFound))?;

    let destination = destination.ok_or_else(|| post(RequestResetTokenEvent::NotFound))?;

    let clock = infra.clock();

    let reset_token = token_generator.generate();
    let requested_at = clock.now();
    let expires = requested_at.expires(&config.token_expires);

    post(RequestResetTokenEvent::TokenExpiresCalculated(
        expires.clone(),
    ));

    reset_token_repository
        .register_reset_token(
            reset_token.clone(),
            user_id,
            fields.login_id,
            destination.clone(),
            expires.clone(),
            requested_at,
        )
        .await
        .map_err(|err| post(RequestResetTokenEvent::RepositoryError(err)))?;

    let token_encoded = token_encoder
        .encode(reset_token, expires)
        .map_err(|err| post(RequestResetTokenEvent::EncodeError(err)))?;

    let notify_response = token_notifier
        .notify(destination, token_encoded)
        .await
        .map_err(|err| post(RequestResetTokenEvent::NotifyError(err)))?;

    post(RequestResetTokenEvent::TokenNotified(notify_response));

    Ok(post(RequestResetTokenEvent::Success))
}
