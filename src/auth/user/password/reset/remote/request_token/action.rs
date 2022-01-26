use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::ticket::remote::validate_nonce::{
    data::ValidateAuthNonceError,
    method::{validate_auth_nonce, ValidateAuthNonceInfra},
};

use crate::auth::{
    ticket::remote::kernel::infra::AuthClock,
    user::password::reset::remote::request_token::infra::{
        RegisterResetTokenRepository, RequestResetTokenConfig, RequestResetTokenFieldsExtract,
        RequestResetTokenRequestDecoder, ResetTokenDestinationRepository, ResetTokenEncoder,
        ResetTokenGenerator, ResetTokenNotifier,
    },
};

use crate::{
    auth::{
        ticket::remote::kernel::data::ExpireDateTime,
        user::{
            login_id::remote::data::{LoginId, ValidateLoginIdError},
            password::reset::remote::request_token::data::{
                EncodeResetTokenError, NotifyResetTokenError, NotifyResetTokenResponse,
                RegisterResetTokenRepositoryError, RequestResetTokenError,
            },
        },
    },
    z_lib::remote::repository::data::RepositoryError,
};

pub enum RequestResetTokenState {
    Nonce(ValidateAuthNonceError),
    RequestToken(RequestResetTokenEvent),
}

impl std::fmt::Display for RequestResetTokenState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nonce(err) => err.fmt(f),
            Self::RequestToken(event) => event.fmt(f),
        }
    }
}

pub trait RequestResetTokenMaterial {
    type ValidateNonce: ValidateAuthNonceInfra;

    type Clock: AuthClock;
    type PasswordRepository: RegisterResetTokenRepository;
    type DestinationRepository: ResetTokenDestinationRepository;
    type TokenGenerator: ResetTokenGenerator;
    type TokenEncoder: ResetTokenEncoder;
    type TokenNotifier: ResetTokenNotifier;

    fn validate_nonce(&self) -> &Self::ValidateNonce;

    fn clock(&self) -> &Self::Clock;
    fn password_repository(&self) -> &Self::PasswordRepository;
    fn destination_repository(&self) -> &Self::DestinationRepository;
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

        validate_auth_nonce(m.validate_nonce())
            .await
            .map_err(|err| pubsub.post(RequestResetTokenState::Nonce(err)))?;

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
    InvalidRequest(RequestResetTokenError),
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
            Self::InvalidRequest(err) => write!(f, "{}; {}", ERROR, err),
            Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
            Self::EncodeError(err) => write!(f, "{}; {}", ERROR, err),
            Self::NotifyError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

fn destination_not_found() -> RequestResetTokenEvent {
    RequestResetTokenEvent::InvalidRequest(RequestResetTokenError::DestinationNotFound)
}

impl Into<RequestResetTokenEvent> for ValidateLoginIdError {
    fn into(self) -> RequestResetTokenEvent {
        RequestResetTokenEvent::InvalidRequest(RequestResetTokenError::InvalidLoginId(self))
    }
}

impl Into<RequestResetTokenEvent> for RegisterResetTokenRepositoryError {
    fn into(self) -> RequestResetTokenEvent {
        match self {
            Self::RepositoryError(err) => RequestResetTokenEvent::RepositoryError(err),
            Self::UserNotFound => {
                RequestResetTokenEvent::InvalidRequest(RequestResetTokenError::UserNotFound)
            }
        }
    }
}

async fn request_reset_token<S>(
    infra: &impl RequestResetTokenMaterial,
    fields: RequestResetTokenFieldsExtract,
    post: impl Fn(RequestResetTokenEvent) -> S,
) -> MethodResult<S> {
    let destination_repository = infra.destination_repository();
    let token_generator = infra.token_generator();
    let token_encoder = infra.token_encoder();
    let token_notifier = infra.token_notifier();
    let config = infra.config();

    let login_id = LoginId::validate(fields.login_id).map_err(|err| post(err.into()))?;

    let destination = destination_repository
        .get(&login_id)
        .await
        .map_err(|err| post(RequestResetTokenEvent::RepositoryError(err)))?
        .ok_or_else(|| post(destination_not_found()))?;

    let clock = infra.clock();
    let password_repository = infra.password_repository();

    let reset_token = token_generator.generate();

    let requested_at = clock.now();
    let expires = requested_at.clone().expires(&config.token_expires);

    post(RequestResetTokenEvent::TokenExpiresCalculated(
        expires.clone(),
    ));

    password_repository
        .register_reset_token(
            login_id,
            reset_token.clone(),
            destination.clone(),
            expires.clone(),
            requested_at,
        )
        .await
        .map_err(|err| post(err.into()))?;

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
