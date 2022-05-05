use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::ticket::validate::method::{
    authenticate, AuthenticateEvent, AuthenticateInfra,
};

use crate::auth::user::password::{
    change::infra::{
        ChangePasswordFields, ChangePasswordFieldsExtract, ChangePasswordRepository,
        ChangePasswordRequestDecoder, OverwritePasswordFields, OverwritePasswordFieldsExtract,
        OverwritePasswordRepository, OverwritePasswordRequestDecoder,
    },
    kernel::infra::{AuthUserPasswordHasher, AuthUserPasswordMatcher, PlainPassword},
};

use crate::{
    auth::{
        ticket::kernel::data::AuthTicket,
        user::password::{
            change::data::{
                ValidateChangePasswordFieldsError, ValidateOverwritePasswordFieldsError,
            },
            kernel::data::PasswordHashError,
        },
    },
    z_lib::repository::data::RepositoryError,
};

pub enum ChangePasswordState {
    Authenticate(AuthenticateEvent),
    Change(ChangePasswordEvent),
}

impl std::fmt::Display for ChangePasswordState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authenticate(event) => event.fmt(f),
            Self::Change(event) => event.fmt(f),
        }
    }
}

pub trait ChangePasswordMaterial {
    type Authenticate: AuthenticateInfra;

    type PasswordRepository: ChangePasswordRepository;
    type PasswordMatcher: AuthUserPasswordMatcher;
    type PasswordHasher: AuthUserPasswordHasher;

    fn authenticate(&self) -> &Self::Authenticate;

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

        let ticket = authenticate(m.authenticate(), |event| {
            pubsub.post(ChangePasswordState::Authenticate(event))
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
    Invalid(ValidateChangePasswordFieldsError),
    NotFound,
    PasswordNotMatched,
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
                Self::Invalid(err) => write!(f, "{}; invalid; {}", ERROR, err),
                Self::NotFound => write!(f, "{}; not found", ERROR),
                Self::PasswordNotMatched => write!(f, "{}; password not matched", ERROR),
                Self::PasswordHashError(err) => write!(f, "{}; {}", ERROR, err),
                Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
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
    let fields = ChangePasswordFields::convert(fields)
        .map_err(|err| post(ChangePasswordEvent::Invalid(err)))?;

    let password_repository = infra.password_repository();
    let password_matcher = infra.password_matcher(fields.current_password);
    let password_hasher = infra.password_hasher(fields.new_password);

    let user_id = ticket.into_user().into_user_id();

    let stored_password = password_repository
        .lookup_password(&user_id)
        .await
        .map_err(|err| post(ChangePasswordEvent::RepositoryError(err)))?
        .ok_or_else(|| post(ChangePasswordEvent::NotFound))?;

    if !password_matcher
        .match_password(&stored_password)
        .map_err(|err| post(ChangePasswordEvent::PasswordHashError(err)))?
    {
        return Err(post(ChangePasswordEvent::PasswordNotMatched));
    }

    let hashed_password = password_hasher
        .hash_password()
        .map_err(|err| post(ChangePasswordEvent::PasswordHashError(err)))?;

    password_repository
        .change_password(user_id, hashed_password)
        .await
        .map_err(|err| post(ChangePasswordEvent::RepositoryError(err)))?;

    Ok(post(ChangePasswordEvent::Success))
}

pub enum OverwritePasswordState {
    Authenticate(AuthenticateEvent),
    Overwrite(OverwritePasswordEvent),
}

impl std::fmt::Display for OverwritePasswordState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authenticate(event) => event.fmt(f),
            Self::Overwrite(event) => event.fmt(f),
        }
    }
}

pub trait OverwritePasswordMaterial {
    type Authenticate: AuthenticateInfra;

    type PasswordRepository: OverwritePasswordRepository;
    type PasswordHasher: AuthUserPasswordHasher;

    fn authenticate(&self) -> &Self::Authenticate;

    fn password_repository(&self) -> &Self::PasswordRepository;
    fn password_hasher(&self, plain_password: PlainPassword) -> Self::PasswordHasher {
        Self::PasswordHasher::new(plain_password)
    }
}

pub struct OverwritePasswordAction<R: OverwritePasswordRequestDecoder, M: OverwritePasswordMaterial> {
    pubsub: ActionStatePubSub<OverwritePasswordState>,
    request_decoder: R,
    material: M,
}

impl<R: OverwritePasswordRequestDecoder, M: OverwritePasswordMaterial> OverwritePasswordAction<R, M> {
    pub fn with_material(request_decoder: R, material: M) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            request_decoder,
            material,
        }
    }

    pub fn subscribe(&mut self, handler: impl 'static + Fn(&OverwritePasswordState) + Send + Sync) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(self) -> MethodResult<OverwritePasswordState> {
        let pubsub = self.pubsub;
        let m = self.material;

        let fields = self.request_decoder.decode();

        authenticate(m.authenticate(), |event| {
            pubsub.post(OverwritePasswordState::Authenticate(event))
        })
        .await?;

        overwrite_password(&m, fields, |event| {
            pubsub.post(OverwritePasswordState::Overwrite(event))
        })
        .await
    }
}

pub enum OverwritePasswordEvent {
    Success,
    Invalid(ValidateOverwritePasswordFieldsError),
    NotFound,
    PasswordHashError(PasswordHashError),
    RepositoryError(RepositoryError),
}

mod overwrite_password_event {
    use super::OverwritePasswordEvent;

    const SUCCESS: &'static str = "overwrite password success";
    const ERROR: &'static str = "overwrite password error";

    impl std::fmt::Display for OverwritePasswordEvent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Success => write!(f, "{}", SUCCESS),
                Self::Invalid(err) => write!(f, "{}; invalid; {}", ERROR, err),
                Self::NotFound => write!(f, "{}; not found", ERROR),
                Self::PasswordHashError(err) => write!(f, "{}; {}", ERROR, err),
                Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
            }
        }
    }
}

async fn overwrite_password<S>(
    infra: &impl OverwritePasswordMaterial,
    fields: OverwritePasswordFieldsExtract,
    post: impl Fn(OverwritePasswordEvent) -> S,
) -> MethodResult<S> {
    let fields = OverwritePasswordFields::convert(fields)
        .map_err(|err| post(OverwritePasswordEvent::Invalid(err)))?;

    let password_repository = infra.password_repository();
    let password_hasher = infra.password_hasher(fields.new_password);

    let user_id = password_repository
        .lookup_user_id(&fields.login_id)
        .await
        .map_err(|err| post(OverwritePasswordEvent::RepositoryError(err)))?
        .ok_or_else(|| post(OverwritePasswordEvent::NotFound))?;

    let hashed_password = password_hasher
        .hash_password()
        .map_err(|err| post(OverwritePasswordEvent::PasswordHashError(err)))?;

    password_repository
        .overwrite_password(user_id, hashed_password)
        .await
        .map_err(|err| post(OverwritePasswordEvent::RepositoryError(err)))?;

    Ok(post(OverwritePasswordEvent::Success))
}
