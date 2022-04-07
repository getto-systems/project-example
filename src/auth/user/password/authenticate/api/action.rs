use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::ticket::{
    encode::method::{encode_auth_ticket, EncodeAuthTicketEvent, EncodeAuthTicketInfra},
    issue::method::{issue_auth_ticket, IssueAuthTicketEvent, IssueAuthTicketInfra},
    validate::method::{validate_auth_nonce, ValidateAuthNonceEvent, ValidateAuthNonceInfra},
};

use crate::auth::user::password::{
    authenticate::infra::{
        AuthenticatePasswordFields, AuthenticatePasswordFieldsExtract,
        AuthenticatePasswordRepository, AuthenticatePasswordRequestDecoder,
    },
    kernel::infra::{AuthUserPasswordMatcher, PlainPassword},
};

use crate::{
    auth::user::{
        kernel::data::AuthUser,
        password::{
            authenticate::data::ValidateAuthenticatePasswordFieldsError,
            kernel::data::PasswordHashError,
        },
    },
    z_lib::repository::data::RepositoryError,
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

    type PasswordRepository: AuthenticatePasswordRepository;
    type PasswordMatcher: AuthUserPasswordMatcher;

    fn validate_nonce(&self) -> &Self::ValidateNonce;
    fn issue(&self) -> &Self::Issue;
    fn encode(&self) -> &Self::Encode;

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
    Invalid(ValidateAuthenticatePasswordFieldsError),
    NotFound,
    PasswordNotMatched,
    PasswordHashError(PasswordHashError),
    RepositoryError(RepositoryError),
}

const SUCCESS: &'static str = "authenticate password success";
const ERROR: &'static str = "authenticate password error";

impl std::fmt::Display for AuthenticatePasswordEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success(user) => write!(f, "{}; {}", SUCCESS, user),
            Self::Invalid(err) => write!(f, "{}; invalid; {}", ERROR, err),
            Self::NotFound => write!(f, "{}; not found", ERROR),
            Self::PasswordNotMatched => write!(f, "{}; password not matched", ERROR),
            Self::PasswordHashError(err) => write!(f, "{}; {}", ERROR, err),
            Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

async fn authenticate_password<S>(
    infra: &impl AuthenticatePasswordMaterial,
    fields: AuthenticatePasswordFieldsExtract,
    post: impl Fn(AuthenticatePasswordEvent) -> S,
) -> Result<AuthUser, S> {
    let fields = AuthenticatePasswordFields::validate(fields)
        .map_err(|err| post(AuthenticatePasswordEvent::Invalid(err)))?;

    let password_repository = infra.password_repository();
    let password_matcher = infra.password_matcher(fields.password);

    let user_id = password_repository
        .lookup_user_id(&fields.login_id)
        .await
        .map_err(|err| post(AuthenticatePasswordEvent::RepositoryError(err)))?
        .ok_or_else(|| post(AuthenticatePasswordEvent::NotFound))?;

    let (hashed_password, granted_roles) = password_repository
        .lookup_user(&user_id)
        .await
        .map_err(|err| post(AuthenticatePasswordEvent::RepositoryError(err)))?
        .ok_or_else(|| post(AuthenticatePasswordEvent::NotFound))?;

    if !password_matcher
        .match_password(&hashed_password)
        .map_err(|err| post(AuthenticatePasswordEvent::PasswordHashError(err)))?
    {
        return Err(post(AuthenticatePasswordEvent::PasswordNotMatched));
    }

    let user = AuthUser::restore(user_id, granted_roles);

    post(AuthenticatePasswordEvent::Success(user.clone()));
    Ok(user)
}
