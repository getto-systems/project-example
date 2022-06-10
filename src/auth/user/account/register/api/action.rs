use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::ticket::validate::method::{authenticate, AuthenticateEvent, AuthenticateInfra};

use crate::auth::user::account::register::infra::{
    AuthUserIdGenerator, RegisterAuthUserAccountFields, RegisterAuthUserAccountFieldsExtract,
    RegisterAuthUserAccountRepository, RegisterAuthUserAccountRequestDecoder,
};

use crate::{
    auth::{
        data::RequireAuthRoles, ticket::kernel::data::PermissionError,
        user::account::register::data::ValidateRegisterAuthUserAccountFieldsError,
    },
    z_lib::repository::data::RepositoryError,
};

pub enum RegisterAuthUserAccountState {
    Authenticate(AuthenticateEvent),
    PermissionError(PermissionError),
    RegisterUser(RegisterAuthUserAccountEvent),
}

impl std::fmt::Display for RegisterAuthUserAccountState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authenticate(event) => event.fmt(f),
            Self::PermissionError(event) => event.fmt(f),
            Self::RegisterUser(event) => event.fmt(f),
        }
    }
}

pub trait RegisterAuthUserAccountMaterial {
    type Authenticate: AuthenticateInfra;

    type UserIdGenerator: AuthUserIdGenerator;
    type UserRepository: RegisterAuthUserAccountRepository;

    fn authenticate(&self) -> &Self::Authenticate;

    fn user_id_generator(&self) -> &Self::UserIdGenerator;
    fn user_repository(&self) -> &Self::UserRepository;
}

pub struct RegisterAuthUserAccountAction<
    R: RegisterAuthUserAccountRequestDecoder,
    M: RegisterAuthUserAccountMaterial,
> {
    pubsub: ActionStatePubSub<RegisterAuthUserAccountState>,
    request_decoder: R,
    material: M,
}

impl<R: RegisterAuthUserAccountRequestDecoder, M: RegisterAuthUserAccountMaterial>
    RegisterAuthUserAccountAction<R, M>
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
        handler: impl 'static + Fn(&RegisterAuthUserAccountState) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(self) -> MethodResult<RegisterAuthUserAccountState> {
        let pubsub = self.pubsub;
        let m = self.material;

        let fields = self.request_decoder.decode();

        let ticket = authenticate(m.authenticate(), |event| {
            pubsub.post(RegisterAuthUserAccountState::Authenticate(event))
        })
        .await?;

        ticket
            .check_enough_permission(RequireAuthRoles::user())
            .map_err(|err| pubsub.post(RegisterAuthUserAccountState::PermissionError(err)))?;

        register_user(&m, fields, |event| {
            pubsub.post(RegisterAuthUserAccountState::RegisterUser(event))
        })
        .await
    }
}

pub enum RegisterAuthUserAccountEvent {
    Success,
    Invalid(ValidateRegisterAuthUserAccountFieldsError),
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
    fields: Option<RegisterAuthUserAccountFieldsExtract>,
    post: impl Fn(RegisterAuthUserAccountEvent) -> S,
) -> MethodResult<S> {
    let fields = RegisterAuthUserAccountFields::convert(fields)
        .map_err(|err| post(RegisterAuthUserAccountEvent::Invalid(err)))?;

    let user_id_generator = infra.user_id_generator();
    let user_repository = infra.user_repository();

    if user_repository
        .check_login_id_registered(&fields.login_id)
        .await
        .map_err(|err| post(RegisterAuthUserAccountEvent::RepositoryError(err)))?
    {
        return Err(post(RegisterAuthUserAccountEvent::LoginIdAlreadyRegistered));
    }

    let user_id = user_id_generator.generate();

    user_repository
        .register_user(user_id, fields)
        .await
        .map_err(|err| post(RegisterAuthUserAccountEvent::RepositoryError(err)))?;

    Ok(post(RegisterAuthUserAccountEvent::Success))
}
