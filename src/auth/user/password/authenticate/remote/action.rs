use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::ticket::{
    encode::method::{encode_auth_ticket, EncodeAuthTicketEvent, EncodeAuthTicketInfra},
    issue::method::{issue_auth_ticket, IssueAuthTicketEvent, IssueAuthTicketInfra},
    validate::method::{validate_auth_nonce, ValidateAuthNonceEvent, ValidateAuthNonceInfra},
};

use crate::auth::user::{
    password::{
        authenticate::remote::infra::{
            AuthenticatePasswordFieldsExtract, AuthenticatePasswordRequestDecoder,
            VerifyPasswordRepository,
        },
        kernel::infra::{AuthUserPasswordMatcher, PlainPassword},
    },
    remote::kernel::infra::AuthUserRepository,
};

use crate::{
    auth::user::{
        login_id::kernel::data::{LoginId, ValidateLoginIdError},
        password::{
            authenticate::remote::data::{
                AuthenticatePasswordError, VerifyPasswordRepositoryError,
            },
            kernel::data::{PasswordHashError, ValidatePasswordError},
        },
        remote::kernel::data::AuthUser,
    },
    z_lib::remote::repository::data::RepositoryError,
};

pub enum AuthenticatePasswordState {
    Authenticate(AuthenticatePasswordEvent),
    ValidateNonce(ValidateAuthNonceEvent),
    Issue(IssueAuthTicketEvent),
    Encode(EncodeAuthTicketEvent),
}

impl std::fmt::Display for AuthenticatePasswordState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authenticate(event) => event.fmt(f),
            Self::ValidateNonce(event) => event.fmt(f),
            Self::Issue(event) => event.fmt(f),
            Self::Encode(event) => event.fmt(f),
        }
    }
}

pub trait AuthenticatePasswordMaterial {
    type ValidateNonce: ValidateAuthNonceInfra;
    type Issue: IssueAuthTicketInfra;
    type Encode: EncodeAuthTicketInfra;

    type UserRepository: AuthUserRepository;
    type PasswordRepository: VerifyPasswordRepository;
    type PasswordMatcher: AuthUserPasswordMatcher;

    fn validate_nonce(&self) -> &Self::ValidateNonce;
    fn issue(&self) -> &Self::Issue;
    fn encode(&self) -> &Self::Encode;

    fn user_repository(&self) -> &Self::UserRepository;
    fn password_repository(&self) -> &Self::PasswordRepository;
    fn password_matcher(&self, plain_password: PlainPassword) -> Self::PasswordMatcher {
        Self::PasswordMatcher::new(plain_password)
    }
}

pub struct AuthenticatePasswordAction<
    R: AuthenticatePasswordRequestDecoder,
    M: AuthenticatePasswordMaterial,
> {
    pubsub: ActionStatePubSub<AuthenticatePasswordState>,
    request_decoder: R,
    material: M,
}

impl<R: AuthenticatePasswordRequestDecoder, M: AuthenticatePasswordMaterial>
    AuthenticatePasswordAction<R, M>
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
        handler: impl 'static + Fn(&AuthenticatePasswordState) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(self) -> MethodResult<AuthenticatePasswordState> {
        let pubsub = self.pubsub;
        let m = self.material;

        let fields = self.request_decoder.decode();

        validate_auth_nonce(m.validate_nonce(), |event| {
            pubsub.post(AuthenticatePasswordState::ValidateNonce(event))
        })
        .await?;

        let user = authenticate_password(&m, fields, |event| {
            pubsub.post(AuthenticatePasswordState::Authenticate(event))
        })
        .await?;

        let ticket = issue_auth_ticket(m.issue(), user, |event| {
            pubsub.post(AuthenticatePasswordState::Issue(event))
        })
        .await?;

        encode_auth_ticket(m.encode(), ticket, |event| {
            pubsub.post(AuthenticatePasswordState::Encode(event))
        })
        .await
    }
}

pub enum AuthenticatePasswordEvent {
    Success(AuthUser),
    UserNotFound,
    InvalidPassword(AuthenticatePasswordError),
    PasswordHashError(PasswordHashError),
    RepositoryError(RepositoryError),
}

const SUCCESS: &'static str = "authenticate password success";
const ERROR: &'static str = "authenticate password error";

impl std::fmt::Display for AuthenticatePasswordEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success(user) => write!(f, "{}; {}", SUCCESS, user),
            Self::InvalidPassword(response) => write!(f, "{}; {}", ERROR, response),
            Self::UserNotFound => write!(f, "{}; user not found", ERROR),
            Self::PasswordHashError(err) => write!(f, "{}; {}", ERROR, err),
            Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

impl Into<AuthenticatePasswordEvent> for VerifyPasswordRepositoryError {
    fn into(self) -> AuthenticatePasswordEvent {
        match self {
            Self::PasswordHashError(err) => AuthenticatePasswordEvent::PasswordHashError(err),
            Self::RepositoryError(err) => AuthenticatePasswordEvent::RepositoryError(err),
            Self::PasswordNotFound => AuthenticatePasswordEvent::InvalidPassword(
                AuthenticatePasswordError::PasswordNotFound,
            ),
            Self::PasswordNotMatched => AuthenticatePasswordEvent::InvalidPassword(
                AuthenticatePasswordError::PasswordNotMatched,
            ),
        }
    }
}

impl Into<AuthenticatePasswordEvent> for ValidateLoginIdError {
    fn into(self) -> AuthenticatePasswordEvent {
        AuthenticatePasswordEvent::InvalidPassword(AuthenticatePasswordError::InvalidLoginId(self))
    }
}

impl Into<AuthenticatePasswordEvent> for ValidatePasswordError {
    fn into(self) -> AuthenticatePasswordEvent {
        AuthenticatePasswordEvent::InvalidPassword(AuthenticatePasswordError::InvalidPassword(self))
    }
}

async fn authenticate_password<S>(
    infra: &impl AuthenticatePasswordMaterial,
    fields: AuthenticatePasswordFieldsExtract,
    post: impl Fn(AuthenticatePasswordEvent) -> S,
) -> Result<AuthUser, S> {
    let login_id = LoginId::validate(fields.login_id).map_err(|err| post(err.into()))?;
    let plain_password =
        PlainPassword::validate(fields.password).map_err(|err| post(err.into()))?;

    let password_repository = infra.password_repository();
    let password_matcher = infra.password_matcher(plain_password);

    let user_id = password_repository
        .verify_password(&login_id, password_matcher)
        .await
        .map_err(|err| post(err.into()))?;

    let user_repository = infra.user_repository();
    let user = user_repository
        .get(&user_id)
        .await
        .map_err(|err| post(AuthenticatePasswordEvent::RepositoryError(err)))?
        .ok_or_else(|| post(AuthenticatePasswordEvent::UserNotFound))?;

    post(AuthenticatePasswordEvent::Success(user.clone()));
    Ok(user)
}
