use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::ticket::validate::method::{
    validate_auth_token, ValidateAuthTokenEvent, ValidateAuthTokenInfra,
};

use crate::auth::user::password::{
    change::infra::{
        ChangePasswordFieldsExtract, ChangePasswordRepository, ChangePasswordRequestDecoder,
        OverridePasswordFieldsExtract, OverridePasswordRepository, OverridePasswordRequestDecoder,
    },
    kernel::infra::{AuthUserPasswordHasher, AuthUserPasswordMatcher, PlainPassword},
};

use crate::{
    auth::{
        ticket::kernel::data::AuthTicket,
        user::{
            login_id::kernel::data::LoginId,
            password::{
                change::data::{
                    ChangePasswordError, ChangePasswordRepositoryError, OverridePasswordError,
                    OverridePasswordRepositoryError,
                },
                kernel::data::{PasswordHashError, ValidatePasswordError},
            },
        },
    },
    z_lib::repository::data::RepositoryError,
};

pub enum ChangePasswordState {
    Validate(ValidateAuthTokenEvent),
    Change(ChangePasswordEvent),
}

impl std::fmt::Display for ChangePasswordState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Validate(event) => event.fmt(f),
            Self::Change(event) => event.fmt(f),
        }
    }
}

pub trait ChangePasswordMaterial {
    type Validate: ValidateAuthTokenInfra;

    type PasswordRepository: ChangePasswordRepository;
    type PasswordMatcher: AuthUserPasswordMatcher;
    type PasswordHasher: AuthUserPasswordHasher;

    fn validate(&self) -> &Self::Validate;

    fn password_repository(&self) -> &Self::PasswordRepository;
    fn password_matcher(&self, plain_password: PlainPassword) -> Self::PasswordMatcher {
        Self::PasswordMatcher::new(plain_password)
    }
    fn password_hasher(&self, plain_password: PlainPassword) -> Self::PasswordHasher {
        Self::PasswordHasher::new(plain_password)
    }
}

pub struct ChangePasswordAction<R: ChangePasswordRequestDecoder, M: ChangePasswordMaterial> {
    pubsub: ActionStatePubSub<ChangePasswordState>,
    request_decoder: R,
    material: M,
}

impl<R: ChangePasswordRequestDecoder, M: ChangePasswordMaterial> ChangePasswordAction<R, M> {
    pub fn with_material(request_decoder: R, material: M) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            request_decoder,
            material,
        }
    }

    pub fn subscribe(&mut self, handler: impl 'static + Fn(&ChangePasswordState) + Send + Sync) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(self) -> MethodResult<ChangePasswordState> {
        let pubsub = self.pubsub;
        let m = self.material;

        let fields = self.request_decoder.decode();

        let ticket = validate_auth_token(m.validate(), |event| {
            pubsub.post(ChangePasswordState::Validate(event))
        })
        .await?;

        change_password(&m, ticket, fields, |event| {
            pubsub.post(ChangePasswordState::Change(event))
        })
        .await
    }
}

pub enum ChangePasswordEvent {
    Success,
    InvalidPassword(ChangePasswordError),
    PasswordHashError(PasswordHashError),
    RepositoryError(RepositoryError),
}

mod change_password_event {
    use super::ChangePasswordEvent;

    const SUCCESS: &'static str = "change password success";
    const ERROR: &'static str = "change password error";

    impl std::fmt::Display for ChangePasswordEvent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Success => write!(f, "{}", SUCCESS),
                Self::InvalidPassword(response) => write!(f, "{}; {}", ERROR, response),
                Self::PasswordHashError(err) => write!(f, "{}; {}", ERROR, err),
                Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
            }
        }
    }
}

impl Into<ChangePasswordEvent> for ChangePasswordRepositoryError {
    fn into(self) -> ChangePasswordEvent {
        match self {
            Self::PasswordHashError(err) => ChangePasswordEvent::PasswordHashError(err),
            Self::RepositoryError(err) => ChangePasswordEvent::RepositoryError(err),
            Self::PasswordNotFound => {
                ChangePasswordEvent::InvalidPassword(ChangePasswordError::PasswordNotFound)
            }
            Self::PasswordNotMatched => {
                ChangePasswordEvent::InvalidPassword(ChangePasswordError::PasswordNotMatched)
            }
        }
    }
}

pub enum ChangePasswordKind {
    Current,
    New,
}

impl Into<ChangePasswordEvent> for (ValidatePasswordError, ChangePasswordKind) {
    fn into(self) -> ChangePasswordEvent {
        match self {
            (err, ChangePasswordKind::Current) => ChangePasswordEvent::InvalidPassword(
                ChangePasswordError::InvalidCurrentPassword(err),
            ),
            (err, ChangePasswordKind::New) => {
                ChangePasswordEvent::InvalidPassword(ChangePasswordError::InvalidNewPassword(err))
            }
        }
    }
}

async fn change_password<S>(
    infra: &impl ChangePasswordMaterial,
    ticket: AuthTicket,
    fields: ChangePasswordFieldsExtract,
    post: impl Fn(ChangePasswordEvent) -> S,
) -> MethodResult<S> {
    let current_password = PlainPassword::validate(fields.current_password)
        .map_err(|err| post((err, ChangePasswordKind::Current).into()))?;
    let new_password = PlainPassword::validate(fields.new_password)
        .map_err(|err| post((err, ChangePasswordKind::New).into()))?;

    let password_repository = infra.password_repository();
    let password_matcher = infra.password_matcher(current_password);
    let password_hasher = infra.password_hasher(new_password);

    let user_id = ticket.into_user().into_user_id();

    password_repository
        .change_password(&user_id, password_matcher, password_hasher)
        .await
        .map_err(|err| post(err.into()))?;

    Ok(post(ChangePasswordEvent::Success))
}

pub enum OverridePasswordState {
    Validate(ValidateAuthTokenEvent),
    Override(OverridePasswordEvent),
}

impl std::fmt::Display for OverridePasswordState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Validate(event) => event.fmt(f),
            Self::Override(event) => event.fmt(f),
        }
    }
}

pub trait OverridePasswordMaterial {
    type Validate: ValidateAuthTokenInfra;

    type PasswordRepository: OverridePasswordRepository;
    type PasswordHasher: AuthUserPasswordHasher;

    fn validate(&self) -> &Self::Validate;

    fn password_repository(&self) -> &Self::PasswordRepository;
    fn password_hasher(&self, plain_password: PlainPassword) -> Self::PasswordHasher {
        Self::PasswordHasher::new(plain_password)
    }
}

pub struct OverridePasswordAction<R: OverridePasswordRequestDecoder, M: OverridePasswordMaterial> {
    pubsub: ActionStatePubSub<OverridePasswordState>,
    request_decoder: R,
    material: M,
}

impl<R: OverridePasswordRequestDecoder, M: OverridePasswordMaterial> OverridePasswordAction<R, M> {
    pub fn with_material(request_decoder: R, material: M) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            request_decoder,
            material,
        }
    }

    pub fn subscribe(&mut self, handler: impl 'static + Fn(&OverridePasswordState) + Send + Sync) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(self) -> MethodResult<OverridePasswordState> {
        let pubsub = self.pubsub;
        let m = self.material;

        let fields = self.request_decoder.decode();

        validate_auth_token(m.validate(), |event| {
            pubsub.post(OverridePasswordState::Validate(event))
        })
        .await?;

        override_password(&m, fields, |event| {
            pubsub.post(OverridePasswordState::Override(event))
        })
        .await
    }
}

pub enum OverridePasswordEvent {
    Success,
    InvalidPassword(OverridePasswordError),
    PasswordHashError(PasswordHashError),
    RepositoryError(RepositoryError),
}

mod override_password_event {
    use super::OverridePasswordEvent;

    const SUCCESS: &'static str = "override password success";
    const ERROR: &'static str = "override password error";

    impl std::fmt::Display for OverridePasswordEvent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Success => write!(f, "{}", SUCCESS),
                Self::InvalidPassword(response) => write!(f, "{}; {}", ERROR, response),
                Self::PasswordHashError(err) => write!(f, "{}; {}", ERROR, err),
                Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
            }
        }
    }
}

impl Into<OverridePasswordEvent> for OverridePasswordRepositoryError {
    fn into(self) -> OverridePasswordEvent {
        match self {
            Self::PasswordHashError(err) => OverridePasswordEvent::PasswordHashError(err),
            Self::RepositoryError(err) => OverridePasswordEvent::RepositoryError(err),
            Self::UserNotFound => {
                OverridePasswordEvent::InvalidPassword(OverridePasswordError::UserNotFound)
            }
        }
    }
}

async fn override_password<S>(
    infra: &impl OverridePasswordMaterial,
    fields: OverridePasswordFieldsExtract,
    post: impl Fn(OverridePasswordEvent) -> S,
) -> MethodResult<S> {
    let login_id = LoginId::validate(fields.login_id).map_err(|err| {
        post(OverridePasswordEvent::InvalidPassword(
            OverridePasswordError::InvalidLoginId(err),
        ))
    })?;
    let new_password = PlainPassword::validate(fields.new_password).map_err(|err| {
        post(OverridePasswordEvent::InvalidPassword(
            OverridePasswordError::InvalidPassword(err),
        ))
    })?;

    let password_repository = infra.password_repository();
    let password_hasher = infra.password_hasher(new_password);

    password_repository
        .override_password(&login_id, password_hasher)
        .await
        .map_err(|err| post(err.into()))?;

    Ok(post(OverridePasswordEvent::Success))
}
