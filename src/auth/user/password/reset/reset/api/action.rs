use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::ticket::{
    encode::method::{encode_auth_token, EncodeAuthTokenEvent, EncodeAuthTokenInfra},
    issue::method::{issue_auth_ticket, IssueAuthTicketEvent, IssueAuthTicketInfra},
};

use crate::auth::{
    kernel::infra::AuthClock,
    user::password::{
        kernel::infra::{AuthUserPasswordHasher, PlainPassword},
        reset::reset::infra::{
            ResetPasswordFieldsExtract, ResetPasswordNotifier, ResetPasswordRepository,
            ResetPasswordTokenDecoder,
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
    common::api::repository::data::RepositoryError,
};

pub enum ResetPasswordState {
    Reset(ResetPasswordEvent),
    Issue(IssueAuthTicketEvent),
    Encode(EncodeAuthTokenEvent),
}

impl std::fmt::Display for ResetPasswordState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reset(event) => event.fmt(f),
            Self::Issue(event) => event.fmt(f),
            Self::Encode(event) => event.fmt(f),
        }
    }
}

pub trait ResetPasswordMaterial {
    type Issue: IssueAuthTicketInfra;
    type Encode: EncodeAuthTokenInfra;

    type Clock: AuthClock;
    type ResetPasswordRepository: ResetPasswordRepository;
    type PasswordHasher: AuthUserPasswordHasher;
    type TokenDecoder: ResetPasswordTokenDecoder;
    type ResetNotifier: ResetPasswordNotifier;

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

pub struct ResetPasswordAction<M: ResetPasswordMaterial> {
    pub info: ResetPasswordActionInfo,
    pubsub: ActionStatePubSub<ResetPasswordState>,
    material: M,
}

pub struct ResetPasswordActionInfo;

impl ResetPasswordActionInfo {
    pub const fn name(&self) -> &'static str {
        "auth.user.password.reset"
    }
}

impl<M: ResetPasswordMaterial> ResetPasswordAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            info: ResetPasswordActionInfo,
            pubsub: ActionStatePubSub::new(),
            material,
        }
    }

    pub fn subscribe(&mut self, handler: impl 'static + Fn(&ResetPasswordState) + Send + Sync) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(
        self,
        fields: impl ResetPasswordFieldsExtract,
    ) -> MethodResult<ResetPasswordState> {
        let user = reset_password(&self.material, fields, |event| {
            self.pubsub.post(ResetPasswordState::Reset(event))
        })
        .await?;

        let ticket = issue_auth_ticket(self.material.issue(), user.into(), |event| {
            self.pubsub.post(ResetPasswordState::Issue(event))
        })
        .await?;

        encode_auth_token(self.material.encode(), ticket, |event| {
            self.pubsub.post(ResetPasswordState::Encode(event))
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
    fields: impl ResetPasswordFieldsExtract,
    post: impl Fn(ResetPasswordEvent) -> S,
) -> Result<AuthUser, S> {
    let fields = fields
        .convert()
        .map_err(|err| post(ResetPasswordEvent::Invalid(err)))?;

    let reset_id = infra
        .token_decoder()
        .decode(fields.reset_token)
        .map_err(|err| post(ResetPasswordEvent::DecodeError(err)))?;

    let reset_at = infra.clock().now();

    let (user_id, stored_login_id, destination, moment) = infra
        .reset_password_repository()
        .lookup_reset_token_entry(&reset_id)
        .await
        .map_err(|err| post(ResetPasswordEvent::RepositoryError(err)))?
        .ok_or_else(|| post(ResetPasswordEvent::NotFound))?;

    let granted = infra
        .reset_password_repository()
        .lookup_permission_granted(&user_id)
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

    let hashed_password = infra
        .password_hasher(fields.new_password)
        .hash_password()
        .map_err(|err| post(ResetPasswordEvent::PasswordHashError(err)))?;

    infra
        .reset_password_repository()
        .reset_password(user_id.clone(), reset_id, hashed_password, reset_at)
        .await
        .map_err(|err| post(ResetPasswordEvent::RepositoryError(err)))?;

    let notify_response = infra
        .reset_notifier()
        .notify(destination)
        .await
        .map_err(|err| post(ResetPasswordEvent::NotifyError(err)))?;

    post(ResetPasswordEvent::ResetNotified(notify_response));

    let user = AuthUser { user_id, granted };

    post(ResetPasswordEvent::Success(user.clone()));
    Ok(user)
}
