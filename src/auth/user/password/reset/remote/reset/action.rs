use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::ticket::remote::{
    encode::method::{encode_auth_ticket, EncodeAuthTicketEvent, EncodeAuthTicketInfra},
    issue::method::{issue_auth_ticket, IssueAuthTicketEvent, IssueAuthTicketInfra},
    validate::method::{validate_auth_nonce, ValidateAuthNonceEvent, ValidateAuthNonceInfra},
};

use crate::auth::{
    ticket::remote::kernel::infra::AuthClock,
    user::{
        password::{
            remote::kernel::infra::{AuthUserPasswordHasher, PlainPassword},
            reset::remote::{
                kernel::infra::ResetTokenEntry,
                reset::infra::{
                    ResetPasswordFieldsExtract, ResetPasswordNotifier, ResetPasswordRepository,
                    ResetPasswordRequestDecoder, ResetTokenDecoder,
                },
            },
        },
        remote::kernel::infra::AuthUserRepository,
    },
};

use crate::{
    auth::{
        ticket::remote::kernel::data::AuthDateTime,
        user::{
            login_id::remote::data::{LoginId, ValidateLoginIdError},
            password::{
                remote::kernel::data::{PasswordHashError, ValidatePasswordError},
                reset::remote::{
                    kernel::data::{
                        ResetTokenDestination, ResetTokenEncoded, ValidateResetTokenError,
                    },
                    reset::data::{
                        DecodeResetTokenError, NotifyResetPasswordError,
                        NotifyResetPasswordResponse, ResetPasswordError,
                        ResetPasswordRepositoryError, VerifyResetTokenEntryError,
                    },
                },
            },
            remote::kernel::data::AuthUser,
        },
    },
    z_lib::remote::repository::data::RepositoryError,
};

pub enum ResetPasswordState {
    ValidateNonce(ValidateAuthNonceEvent),
    Reset(ResetPasswordEvent),
    Issue(IssueAuthTicketEvent),
    Encode(EncodeAuthTicketEvent),
}

impl std::fmt::Display for ResetPasswordState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ValidateNonce(event) => event.fmt(f),
            Self::Reset(event) => event.fmt(f),
            Self::Issue(event) => event.fmt(f),
            Self::Encode(event) => event.fmt(f),
        }
    }
}

pub trait ResetPasswordMaterial {
    type ValidateNonce: ValidateAuthNonceInfra;
    type Issue: IssueAuthTicketInfra;
    type Encode: EncodeAuthTicketInfra;

    type Clock: AuthClock;
    type UserRepository: AuthUserRepository;
    type PasswordRepository: ResetPasswordRepository;
    type PasswordHasher: AuthUserPasswordHasher;
    type TokenDecoder: ResetTokenDecoder;
    type ResetNotifier: ResetPasswordNotifier;

    fn validate_nonce(&self) -> &Self::ValidateNonce;
    fn issue(&self) -> &Self::Issue;
    fn encode(&self) -> &Self::Encode;

    fn clock(&self) -> &Self::Clock;
    fn user_repository(&self) -> &Self::UserRepository;
    fn password_repository(&self) -> &Self::PasswordRepository;
    fn password_hasher(&self, plain_password: PlainPassword) -> Self::PasswordHasher {
        Self::PasswordHasher::new(plain_password)
    }
    fn token_decoder(&self) -> &Self::TokenDecoder;
    fn reset_notifier(&self) -> &Self::ResetNotifier;
}

pub struct ResetPasswordAction<R: ResetPasswordRequestDecoder, M: ResetPasswordMaterial> {
    pubsub: ActionStatePubSub<ResetPasswordState>,
    request_decoder: R,
    material: M,
}

impl<R: ResetPasswordRequestDecoder, M: ResetPasswordMaterial> ResetPasswordAction<R, M> {
    pub fn with_material(request_decoder: R, material: M) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            request_decoder,
            material,
        }
    }

    pub fn subscribe(&mut self, handler: impl 'static + Fn(&ResetPasswordState) + Send + Sync) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(self) -> MethodResult<ResetPasswordState> {
        let pubsub = self.pubsub;
        let m = self.material;

        let fields = self.request_decoder.decode();

        validate_auth_nonce(m.validate_nonce(), |event| {
            pubsub.post(ResetPasswordState::ValidateNonce(event))
        })
        .await?;

        let user = reset_password(&m, fields, |event| {
            pubsub.post(ResetPasswordState::Reset(event))
        })
        .await?;

        let ticket = issue_auth_ticket(m.issue(), user, |event| {
            pubsub.post(ResetPasswordState::Issue(event))
        })
        .await?;

        encode_auth_ticket(m.encode(), ticket, |event| {
            pubsub.post(ResetPasswordState::Encode(event))
        })
        .await
    }
}

pub enum ResetPasswordEvent {
    ResetNotified(NotifyResetPasswordResponse),
    Success(AuthUser),
    InvalidReset(ResetPasswordError),
    UserNotFound,
    RepositoryError(RepositoryError),
    PasswordHashError(PasswordHashError),
    DecodeError(DecodeResetTokenError),
    NotifyError(NotifyResetPasswordError),
}

const SUCCESS: &'static str = "reset password success";
const ERROR: &'static str = "reset password error";

impl std::fmt::Display for ResetPasswordEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ResetNotified(response) => write!(f, "reset password notified; {}", response),
            Self::Success(user) => write!(f, "{}; {}", SUCCESS, user),
            Self::InvalidReset(err) => write!(f, "{}; {}", ERROR, err),
            Self::UserNotFound => write!(f, "{}; user not found", ERROR),
            Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
            Self::PasswordHashError(err) => write!(f, "{}; {}", ERROR, err),
            Self::DecodeError(err) => write!(f, "{}; {}", ERROR, err),
            Self::NotifyError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

impl Into<ResetPasswordEvent> for ValidateLoginIdError {
    fn into(self) -> ResetPasswordEvent {
        ResetPasswordEvent::InvalidReset(ResetPasswordError::InvalidLoginId(self))
    }
}

impl Into<ResetPasswordEvent> for ValidatePasswordError {
    fn into(self) -> ResetPasswordEvent {
        ResetPasswordEvent::InvalidReset(ResetPasswordError::InvalidPassword(self))
    }
}

impl Into<ResetPasswordEvent> for ValidateResetTokenError {
    fn into(self) -> ResetPasswordEvent {
        ResetPasswordEvent::InvalidReset(ResetPasswordError::InvalidResetToken(self))
    }
}

impl Into<ResetPasswordEvent> for VerifyResetTokenEntryError {
    fn into(self) -> ResetPasswordEvent {
        ResetPasswordEvent::InvalidReset(ResetPasswordError::InvalidResetTokenEntry(self))
    }
}

impl Into<ResetPasswordEvent> for ResetPasswordRepositoryError {
    fn into(self) -> ResetPasswordEvent {
        match self {
            Self::RepositoryError(err) => ResetPasswordEvent::RepositoryError(err),
            Self::PasswordHashError(err) => ResetPasswordEvent::PasswordHashError(err),
        }
    }
}

async fn reset_password<S>(
    infra: &impl ResetPasswordMaterial,
    fields: ResetPasswordFieldsExtract,
    post: impl Fn(ResetPasswordEvent) -> S,
) -> Result<AuthUser, S> {
    let login_id = LoginId::validate(fields.login_id).map_err(|err| post(err.into()))?;
    let plain_password =
        PlainPassword::validate(fields.password).map_err(|err| post(err.into()))?;
    let reset_token =
        ResetTokenEncoded::validate(fields.reset_token).map_err(|err| post(err.into()))?;

    let token_decoder = infra.token_decoder();
    let reset_notifier = infra.reset_notifier();

    let reset_token = token_decoder
        .decode(&reset_token)
        .map_err(|err| post(ResetPasswordEvent::DecodeError(err)))?;

    let password_repository = infra.password_repository();
    let password_hasher = infra.password_hasher(plain_password);
    let clock = infra.clock();

    let reset_at = clock.now();

    let entry = password_repository
        .reset_token_entry(&reset_token)
        .await
        .map_err(|err| post(ResetPasswordEvent::RepositoryError(err)))?;

    let destination =
        verify_reset_token_entry(entry, &reset_at, &login_id).map_err(|err| post(err.into()))?;

    let user_id = password_repository
        .reset_password(&reset_token, password_hasher, reset_at)
        .await
        .map_err(|err| post(err.into()))?;

    let user_repository = infra.user_repository();
    let user = user_repository
        .get(&user_id)
        .await
        .map_err(|err| post(ResetPasswordEvent::RepositoryError(err)))?
        .ok_or_else(|| post(ResetPasswordEvent::UserNotFound))?;

    let notify_response = reset_notifier
        .notify(destination)
        .await
        .map_err(|err| post(ResetPasswordEvent::NotifyError(err)))?;

    post(ResetPasswordEvent::ResetNotified(notify_response));

    post(ResetPasswordEvent::Success(user.clone()));
    Ok(user)
}

fn verify_reset_token_entry(
    entry: Option<ResetTokenEntry>,
    reset_at: &AuthDateTime,
    login_id: &LoginId,
) -> Result<ResetTokenDestination, VerifyResetTokenEntryError> {
    let entry = entry.ok_or(VerifyResetTokenEntryError::ResetTokenEntryNotFound)?;
    if entry.has_already_reset() {
        return Err(VerifyResetTokenEntryError::AlreadyReset);
    }
    if entry.has_expired(reset_at) {
        return Err(VerifyResetTokenEntryError::Expired);
    }
    if !entry.verify_login_id(login_id) {
        return Err(VerifyResetTokenEntryError::LoginIdNotMatched);
    }
    Ok(entry.into_destination())
}
