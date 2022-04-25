use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::ticket::{
    encode::method::{encode_auth_ticket, EncodeAuthTicketEvent, EncodeAuthTicketInfra},
    issue::method::{issue_auth_ticket, IssueAuthTicketEvent, IssueAuthTicketInfra},
    validate::method::{validate_auth_nonce, ValidateAuthNonceEvent, ValidateAuthNonceInfra},
};

use crate::auth::{
    ticket::kernel::infra::AuthClock,
    user::password::{
        kernel::infra::{AuthUserPasswordHasher, PlainPassword},
        reset::reset::infra::{
            ResetPasswordFields, ResetPasswordFieldsExtract, ResetPasswordNotifier,
            ResetPasswordRepository, ResetPasswordRequestDecoder, ResetTokenDecoder,
        },
    },
};

use crate::{
    auth::user::{
        kernel::data::AuthUser,
        password::{
            kernel::data::PasswordHashError,
            reset::reset::data::{
                DecodeResetTokenError, NotifyResetPasswordError, NotifyResetPasswordResponse,
                ValidateResetPasswordFieldsError,
            },
        },
    },
    z_lib::repository::data::RepositoryError,
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
    type ResetPasswordRepository: ResetPasswordRepository;
    type PasswordHasher: AuthUserPasswordHasher;
    type TokenDecoder: ResetTokenDecoder;
    type ResetNotifier: ResetPasswordNotifier;

    fn validate_nonce(&self) -> &Self::ValidateNonce;
    fn issue(&self) -> &Self::Issue;
    fn encode(&self) -> &Self::Encode;

    fn clock(&self) -> &Self::Clock;
    fn reset_password_repository(&self) -> &Self::ResetPasswordRepository;
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
    Invalid(ValidateResetPasswordFieldsError),
    NotFound,
    LoginIdNotMatched,
    ResetTokenExpired,
    AlreadyReset,
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
            Self::Invalid(err) => write!(f, "{}; invalid; {}", ERROR, err),
            Self::NotFound => write!(f, "{}; not found", ERROR),
            Self::LoginIdNotMatched => write!(f, "{}; login id not matched", ERROR),
            Self::ResetTokenExpired => write!(f, "{}; reset token expired", ERROR),
            Self::AlreadyReset => write!(f, "{}; already reset", ERROR),
            Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
            Self::PasswordHashError(err) => write!(f, "{}; {}", ERROR, err),
            Self::DecodeError(err) => write!(f, "{}; {}", ERROR, err),
            Self::NotifyError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

async fn reset_password<S>(
    infra: &impl ResetPasswordMaterial,
    fields: ResetPasswordFieldsExtract,
    post: impl Fn(ResetPasswordEvent) -> S,
) -> Result<AuthUser, S> {
    let fields = ResetPasswordFields::convert(fields)
        .map_err(|err| post(ResetPasswordEvent::Invalid(err)))?;

    let token_decoder = infra.token_decoder();
    let reset_notifier = infra.reset_notifier();

    let reset_token = token_decoder
        .decode(fields.reset_token)
        .map_err(|err| post(ResetPasswordEvent::DecodeError(err)))?;

    let reset_password_repository = infra.reset_password_repository();
    let password_hasher = infra.password_hasher(fields.new_password);
    let clock = infra.clock();

    let reset_at = clock.now();

    let (user_id, stored_login_id, destination, moment) = reset_password_repository
        .lookup_reset_token_entry(&reset_token)
        .await
        .map_err(|err| post(ResetPasswordEvent::RepositoryError(err)))?
        .ok_or_else(|| post(ResetPasswordEvent::NotFound))?;

    let granted_roles = reset_password_repository
        .lookup_granted_roles(&user_id)
        .await
        .map_err(|err| post(ResetPasswordEvent::RepositoryError(err)))?
        .ok_or_else(|| post(ResetPasswordEvent::NotFound))?;

    if stored_login_id != fields.login_id {
        return Err(post(ResetPasswordEvent::LoginIdNotMatched));
    }

    if moment.has_already_reset() {
        return Err(post(ResetPasswordEvent::AlreadyReset));
    }

    if moment.has_expired(&reset_at) {
        return Err(post(ResetPasswordEvent::ResetTokenExpired));
    }

    let hashed_password = password_hasher
        .hash_password()
        .map_err(|err| post(ResetPasswordEvent::PasswordHashError(err)))?;

    reset_password_repository
        .reset_password(user_id.clone(), reset_token, hashed_password, reset_at)
        .await
        .map_err(|err| post(ResetPasswordEvent::RepositoryError(err)))?;

    let notify_response = reset_notifier
        .notify(destination)
        .await
        .map_err(|err| post(ResetPasswordEvent::NotifyError(err)))?;

    post(ResetPasswordEvent::ResetNotified(notify_response));

    let user = AuthUser::restore(user_id, granted_roles);
    post(ResetPasswordEvent::Success(user.clone()));
    Ok(user)
}
