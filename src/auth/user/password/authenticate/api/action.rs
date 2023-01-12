use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::ticket::{
    encode::method::{encode_auth_token, EncodeAuthTokenEvent, EncodeAuthTokenInfra},
    issue::method::{issue_auth_ticket, IssueAuthTicketEvent, IssueAuthTicketInfra},
};

use crate::auth::user::password::{
    authenticate::infra::{AuthenticatePasswordRepository, AuthenticateWithPasswordFieldsExtract},
    kernel::infra::{AuthUserPasswordMatcher, PlainPassword},
};

use crate::{
    auth::user::{
        kernel::data::AuthUser,
        password::{
            authenticate::data::ValidateAuthenticateWithPasswordFieldsError,
            kernel::data::PasswordHashError,
        },
    },
    common::api::repository::data::RepositoryError,
};

pub enum AuthenticateWithPasswordState {
    AuthenticateWithPassword(AuthenticateWithPasswordEvent),
    Issue(IssueAuthTicketEvent),
    Encode(EncodeAuthTokenEvent),
}

impl std::fmt::Display for AuthenticateWithPasswordState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AuthenticateWithPassword(event) => event.fmt(f),
            Self::Issue(event) => event.fmt(f),
            Self::Encode(event) => event.fmt(f),
        }
    }
}

pub trait AuthenticateWithPasswordMaterial {
    type Issue: IssueAuthTicketInfra;
    type Encode: EncodeAuthTokenInfra;

    type PasswordRepository: AuthenticatePasswordRepository;
    type PasswordMatcher: AuthUserPasswordMatcher;

    fn issue(&self) -> &Self::Issue;
    fn encode(&self) -> &Self::Encode;

    fn password_repository(&self) -> &Self::PasswordRepository;
    fn password_matcher(&self, plain_password: PlainPassword) -> Self::PasswordMatcher {
        Self::PasswordMatcher::new(plain_password)
    }
}

pub struct AuthenticateWithPasswordAction<M: AuthenticateWithPasswordMaterial> {
    pub info: AuthenticateWithPasswordActionInfo,
    pubsub: ActionStatePubSub<AuthenticateWithPasswordState>,
    material: M,
}

pub struct AuthenticateWithPasswordActionInfo;

impl AuthenticateWithPasswordActionInfo {
    pub const fn name(&self) -> &'static str {
        "auth.user.password.authenticate"
    }
}

impl<M: AuthenticateWithPasswordMaterial> AuthenticateWithPasswordAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            info: AuthenticateWithPasswordActionInfo,
            pubsub: ActionStatePubSub::new(),
            material,
        }
    }

    pub fn subscribe(
        &mut self,
        handler: impl 'static + Fn(&AuthenticateWithPasswordState) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(
        self,
        fields: impl AuthenticateWithPasswordFieldsExtract,
    ) -> MethodResult<AuthenticateWithPasswordState> {
        let user = authenticate_with_password(&self.material, fields, |event| {
            self.pubsub
                .post(AuthenticateWithPasswordState::AuthenticateWithPassword(
                    event,
                ))
        })
        .await?;

        let ticket = issue_auth_ticket(self.material.issue(), user.into(), |event| {
            self.pubsub
                .post(AuthenticateWithPasswordState::Issue(event))
        })
        .await?;

        encode_auth_token(self.material.encode(), ticket, |event| {
            self.pubsub
                .post(AuthenticateWithPasswordState::Encode(event))
        })
        .await
    }
}

pub enum AuthenticateWithPasswordEvent {
    Success(AuthUser),
    Invalid(ValidateAuthenticateWithPasswordFieldsError),
    NotFound,
    PasswordNotMatched,
    PasswordHashError(PasswordHashError),
    RepositoryError(RepositoryError),
}

const SUCCESS: &'static str = "authenticate with password success";
const ERROR: &'static str = "authenticate with password error";

impl std::fmt::Display for AuthenticateWithPasswordEvent {
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

async fn authenticate_with_password<S>(
    infra: &impl AuthenticateWithPasswordMaterial,
    fields: impl AuthenticateWithPasswordFieldsExtract,
    post: impl Fn(AuthenticateWithPasswordEvent) -> S,
) -> Result<AuthUser, S> {
    let fields = fields
        .convert()
        .map_err(|err| post(AuthenticateWithPasswordEvent::Invalid(err)))?;

    let (user_id, stored_password, granted) = infra
        .password_repository()
        .lookup_user(&fields.login_id)
        .await
        .map_err(|err| post(AuthenticateWithPasswordEvent::RepositoryError(err)))?
        .ok_or_else(|| post(AuthenticateWithPasswordEvent::NotFound))?;

    if !infra
        .password_matcher(fields.plain_password)
        .match_password(stored_password)
        .map_err(|err| post(AuthenticateWithPasswordEvent::PasswordHashError(err)))?
    {
        return Err(post(AuthenticateWithPasswordEvent::PasswordNotMatched));
    }

    let user = AuthUser {
        user_id,
        granted: granted.unwrap_or_default(),
    };

    post(AuthenticateWithPasswordEvent::Success(user.clone()));
    Ok(user)
}
