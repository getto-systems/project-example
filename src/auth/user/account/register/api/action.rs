use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::ticket::authorize::proxy::{authorize, AuthorizeEvent, AuthorizeInfra};

use crate::auth::user::account::register::infra::{
    AuthUserIdGenerator, RegisterAuthUserAccountFieldsExtract, RegisterAuthUserAccountRepository,
};

use crate::common::proxy::action::CoreProxyParams;
use crate::{
    auth::{
        ticket::kernel::data::{AuthPermissionRequired, AuthorizeTokenExtract},
        user::account::kernel::data::ValidateAuthUserAccountError,
    },
    common::api::repository::data::RepositoryError,
};

pub enum RegisterAuthUserAccountState {
    Authorize(AuthorizeEvent),
    RegisterUser(RegisterAuthUserAccountEvent),
}

impl std::fmt::Display for RegisterAuthUserAccountState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authorize(event) => event.fmt(f),
            Self::RegisterUser(event) => event.fmt(f),
        }
    }
}

pub trait RegisterAuthUserAccountMaterial {
    type Authorize: AuthorizeInfra;

    type UserIdGenerator: AuthUserIdGenerator;
    type UserRepository: RegisterAuthUserAccountRepository;

    fn authorize(&self) -> &Self::Authorize;

    fn user_id_generator(&self) -> &Self::UserIdGenerator;
    fn user_repository(&self) -> &Self::UserRepository;
}

pub struct RegisterAuthUserAccountAction<M: RegisterAuthUserAccountMaterial> {
    pub info: RegisterAuthUserAccountActionInfo,
    pubsub: ActionStatePubSub<RegisterAuthUserAccountState>,
    material: M,
}

pub struct RegisterAuthUserAccountActionInfo;

impl RegisterAuthUserAccountActionInfo {
    pub const fn name(&self) -> &'static str {
        "auth.user.account.register"
    }

    pub fn required(&self) -> AuthPermissionRequired {
        AuthPermissionRequired::user()
    }

    pub fn params(&self) -> CoreProxyParams {
        (self.name(), self.required())
    }
}

impl<M: RegisterAuthUserAccountMaterial> RegisterAuthUserAccountAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            info: RegisterAuthUserAccountActionInfo,
            pubsub: ActionStatePubSub::new(),
            material,
        }
    }

    pub fn subscribe(
        &mut self,
        handler: impl 'static + Fn(&RegisterAuthUserAccountState) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(
        self,
        token: impl AuthorizeTokenExtract,
        fields: impl RegisterAuthUserAccountFieldsExtract,
    ) -> MethodResult<RegisterAuthUserAccountState> {
        authorize(
            self.material.authorize(),
            (token, self.info.required()),
            |event| {
                self.pubsub
                    .post(RegisterAuthUserAccountState::Authorize(event))
            },
        )
        .await?;

        register_user(&self.material, fields, |event| {
            self.pubsub
                .post(RegisterAuthUserAccountState::RegisterUser(event))
        })
        .await
    }
}

pub enum RegisterAuthUserAccountEvent {
    Success,
    Invalid(ValidateAuthUserAccountError),
    LoginIdAlreadyRegistered,
    RepositoryError(RepositoryError),
}

const SUCCESS: &'static str = "register auth user account success";
const ERROR: &'static str = "register auth user account error";

impl std::fmt::Display for RegisterAuthUserAccountEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => write!(f, "{}", SUCCESS),
            Self::Invalid(err) => write!(f, "{}; invalid; {}", ERROR, err),
            Self::LoginIdAlreadyRegistered => {
                write!(f, "{}; login-id already registered", ERROR)
            }
            Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

async fn register_user<S>(
    infra: &impl RegisterAuthUserAccountMaterial,
    fields: impl RegisterAuthUserAccountFieldsExtract,
    post: impl Fn(RegisterAuthUserAccountEvent) -> S,
) -> MethodResult<S> {
    let fields = fields
        .convert()
        .map_err(|err| post(RegisterAuthUserAccountEvent::Invalid(err)))?;

    if infra
        .user_repository()
        .check_login_id_registered(&fields.login_id)
        .await
        .map_err(|err| post(RegisterAuthUserAccountEvent::RepositoryError(err)))?
    {
        return Err(post(RegisterAuthUserAccountEvent::LoginIdAlreadyRegistered));
    }

    let user_id = infra.user_id_generator().generate();

    infra
        .user_repository()
        .register_user(user_id, fields)
        .await
        .map_err(|err| post(RegisterAuthUserAccountEvent::RepositoryError(err)))?;

    Ok(post(RegisterAuthUserAccountEvent::Success))
}
