use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::common::proxy::action::CoreProxyParams;

use crate::auth::ticket::authorize::proxy::{authorize, AuthorizeEvent, AuthorizeInfra};

use crate::auth::user::password::{
    change::infra::{
        ChangePasswordFieldsExtract, ChangePasswordRepository, OverwritePasswordFieldsExtract,
        OverwritePasswordRepository,
    },
    kernel::infra::{AuthUserPasswordHasher, AuthUserPasswordMatcher, PlainPassword},
};

use crate::{
    auth::{
        ticket::kernel::data::{AuthPermissionRequired, AuthorizeTokenExtract},
        user::{
            kernel::data::AuthUserId,
            password::{
                change::data::{
                    ValidateChangePasswordFieldsError, ValidateOverwritePasswordFieldsError,
                },
                kernel::data::PasswordHashError,
            },
        },
    },
    common::api::repository::data::RepositoryError,
};

pub enum ChangePasswordState {
    Authorize(AuthorizeEvent),
    Change(ChangePasswordEvent),
}

impl std::fmt::Display for ChangePasswordState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authorize(event) => event.fmt(f),
            Self::Change(event) => event.fmt(f),
        }
    }
}

pub trait ChangePasswordMaterial {
    type Authorize: AuthorizeInfra;

    type PasswordRepository: ChangePasswordRepository;
    type PasswordMatcher: AuthUserPasswordMatcher;
    type PasswordHasher: AuthUserPasswordHasher;

    fn authorize(&self) -> &Self::Authorize;

    fn password_repository(&self) -> &Self::PasswordRepository;
    fn password_matcher(&self, plain_password: PlainPassword) -> Self::PasswordMatcher {
        Self::PasswordMatcher::new(plain_password)
    }
    fn password_hasher(&self, plain_password: PlainPassword) -> Self::PasswordHasher {
        Self::PasswordHasher::new(plain_password)
    }
}

pub struct ChangePasswordAction<M: ChangePasswordMaterial> {
    pub info: ChangePasswordActionInfo,
    pubsub: ActionStatePubSub<ChangePasswordState>,
    material: M,
}

pub struct ChangePasswordActionInfo;

impl ChangePasswordActionInfo {
    pub const fn name(&self) -> &'static str {
        "auth.user.password.change"
    }

    pub fn required(&self) -> AuthPermissionRequired {
        AuthPermissionRequired::Nothing
    }

    pub fn params(&self) -> CoreProxyParams {
        (self.name(), self.required())
    }
}

impl<M: ChangePasswordMaterial> ChangePasswordAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            info: ChangePasswordActionInfo,
            pubsub: ActionStatePubSub::new(),
            material,
        }
    }

    pub fn subscribe(&mut self, handler: impl 'static + Fn(&ChangePasswordState) + Send + Sync) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(
        self,
        token: impl AuthorizeTokenExtract,
        fields: impl ChangePasswordFieldsExtract,
    ) -> MethodResult<ChangePasswordState> {
        let user_id = authorize(
            self.material.authorize(),
            (token, self.info.required()),
            |event| self.pubsub.post(ChangePasswordState::Authorize(event)),
        )
        .await?;

        change_password(&self.material, user_id, fields, |event| {
            self.pubsub.post(ChangePasswordState::Change(event))
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
    user_id: AuthUserId,
    fields: impl ChangePasswordFieldsExtract,
    post: impl Fn(ChangePasswordEvent) -> S,
) -> MethodResult<S> {
    let fields = fields
        .convert()
        .map_err(|err| post(ChangePasswordEvent::Invalid(err)))?;

    let stored_password = infra
        .password_repository()
        .lookup_password(&user_id)
        .await
        .map_err(|err| post(ChangePasswordEvent::RepositoryError(err)))?
        .ok_or_else(|| post(ChangePasswordEvent::NotFound))?;

    if !infra
        .password_matcher(fields.current_password)
        .match_password(stored_password)
        .map_err(|err| post(ChangePasswordEvent::PasswordHashError(err)))?
    {
        return Err(post(ChangePasswordEvent::PasswordNotMatched));
    }

    let hashed_password = infra
        .password_hasher(fields.new_password)
        .hash_password()
        .map_err(|err| post(ChangePasswordEvent::PasswordHashError(err)))?;

    infra
        .password_repository()
        .change_password(user_id, hashed_password)
        .await
        .map_err(|err| post(ChangePasswordEvent::RepositoryError(err)))?;

    Ok(post(ChangePasswordEvent::Success))
}

pub enum OverwritePasswordState {
    Authorize(AuthorizeEvent),
    Overwrite(OverwritePasswordEvent),
}

impl std::fmt::Display for OverwritePasswordState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authorize(event) => event.fmt(f),
            Self::Overwrite(event) => event.fmt(f),
        }
    }
}

pub trait OverwritePasswordMaterial {
    type Authorize: AuthorizeInfra;

    type PasswordRepository: OverwritePasswordRepository;
    type PasswordHasher: AuthUserPasswordHasher;

    fn authorize(&self) -> &Self::Authorize;

    fn password_repository(&self) -> &Self::PasswordRepository;
    fn password_hasher(&self, plain_password: PlainPassword) -> Self::PasswordHasher {
        Self::PasswordHasher::new(plain_password)
    }
}

pub struct OverwritePasswordAction<M: OverwritePasswordMaterial> {
    pub info: OverwritePasswordActionInfo,
    pubsub: ActionStatePubSub<OverwritePasswordState>,
    material: M,
}

pub struct OverwritePasswordActionInfo;

impl OverwritePasswordActionInfo {
    pub const fn name(&self) -> &'static str {
        "auth.user.password.overwrite"
    }

    pub fn required(&self) -> AuthPermissionRequired {
        AuthPermissionRequired::user()
    }

    pub fn params(&self) -> CoreProxyParams {
        (self.name(), self.required())
    }
}

impl<M: OverwritePasswordMaterial> OverwritePasswordAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            info: OverwritePasswordActionInfo,
            pubsub: ActionStatePubSub::new(),
            material,
        }
    }

    pub fn subscribe(&mut self, handler: impl 'static + Fn(&OverwritePasswordState) + Send + Sync) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(
        self,
        token: impl AuthorizeTokenExtract,
        fields: impl OverwritePasswordFieldsExtract,
    ) -> MethodResult<OverwritePasswordState> {
        authorize(
            self.material.authorize(),
            (token, self.info.required()),
            |event| self.pubsub.post(OverwritePasswordState::Authorize(event)),
        )
        .await?;

        overwrite_password(&self.material, fields, |event| {
            self.pubsub.post(OverwritePasswordState::Overwrite(event))
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
    fields: impl OverwritePasswordFieldsExtract,
    post: impl Fn(OverwritePasswordEvent) -> S,
) -> MethodResult<S> {
    let fields = fields
        .convert()
        .map_err(|err| post(OverwritePasswordEvent::Invalid(err)))?;

    let user_id = infra
        .password_repository()
        .lookup_user_id(&fields.login_id)
        .await
        .map_err(|err| post(OverwritePasswordEvent::RepositoryError(err)))?
        .ok_or_else(|| post(OverwritePasswordEvent::NotFound))?;

    let hashed_password = infra
        .password_hasher(fields.new_password)
        .hash_password()
        .map_err(|err| post(OverwritePasswordEvent::PasswordHashError(err)))?;

    infra
        .password_repository()
        .overwrite_password(user_id, hashed_password)
        .await
        .map_err(|err| post(OverwritePasswordEvent::RepositoryError(err)))?;

    Ok(post(OverwritePasswordEvent::Success))
}
