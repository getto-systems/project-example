use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::ticket::authorize::proxy::{authorize, AuthorizeEvent, AuthorizeInfra};

use crate::auth::user::login_id::change::infra::{
    OverwriteLoginIdFieldsExtract, OverwriteLoginIdRepository,
};

use crate::common::proxy::action::CoreProxyParams;
use crate::{
    auth::{
        ticket::kernel::data::{AuthPermissionRequired, AuthorizeTokenExtract},
        user::login_id::change::data::ValidateOverwriteLoginIdFieldsError,
    },
    common::api::repository::data::RepositoryError,
};

pub enum OverwriteLoginIdState {
    Authorize(AuthorizeEvent),
    Overwrite(OverwriteLoginIdEvent),
}

impl std::fmt::Display for OverwriteLoginIdState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authorize(event) => event.fmt(f),
            Self::Overwrite(event) => event.fmt(f),
        }
    }
}

pub trait OverwriteLoginIdMaterial {
    type Authorize: AuthorizeInfra;

    type LoginIdRepository: OverwriteLoginIdRepository;

    fn authorize(&self) -> &Self::Authorize;

    fn login_id_repository(&self) -> &Self::LoginIdRepository;
}

pub struct OverwriteLoginIdAction<M: OverwriteLoginIdMaterial> {
    pub info: OverwriteLoginIdActionInfo,
    pubsub: ActionStatePubSub<OverwriteLoginIdState>,
    material: M,
}

pub struct OverwriteLoginIdActionInfo;

impl OverwriteLoginIdActionInfo {
    pub const fn name(&self) -> &'static str {
        "auth.user.login-id.change"
    }

    pub fn required(&self) -> AuthPermissionRequired {
        AuthPermissionRequired::user()
    }

    pub fn params(&self) -> CoreProxyParams {
        (self.name(), self.required())
    }
}

impl<M: OverwriteLoginIdMaterial> OverwriteLoginIdAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            info: OverwriteLoginIdActionInfo,
            pubsub: ActionStatePubSub::new(),
            material,
        }
    }

    pub fn subscribe(&mut self, handler: impl 'static + Fn(&OverwriteLoginIdState) + Send + Sync) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(
        self,
        token: impl AuthorizeTokenExtract,
        fields: impl OverwriteLoginIdFieldsExtract,
    ) -> MethodResult<OverwriteLoginIdState> {
        authorize(
            self.material.authorize(),
            (token, self.info.required()),
            |event| self.pubsub.post(OverwriteLoginIdState::Authorize(event)),
        )
        .await?;

        overwrite_login_id(&self.material, fields, |event| {
            self.pubsub.post(OverwriteLoginIdState::Overwrite(event))
        })
        .await
    }
}

pub enum OverwriteLoginIdEvent {
    Success,
    Invalid(ValidateOverwriteLoginIdFieldsError),
    NotFound,
    AlreadyRegistered,
    RepositoryError(RepositoryError),
}

const SUCCESS: &'static str = "overwrite login-id success";
const ERROR: &'static str = "overwrite login-id error";

impl std::fmt::Display for OverwriteLoginIdEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => write!(f, "{}", SUCCESS),
            Self::Invalid(err) => write!(f, "{}; invalid; {}", ERROR, err),
            Self::NotFound => write!(f, "{}; not found", ERROR),
            Self::AlreadyRegistered => {
                write!(f, "{}; new login id is already registered", ERROR)
            }
            Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

async fn overwrite_login_id<S>(
    infra: &impl OverwriteLoginIdMaterial,
    fields: impl OverwriteLoginIdFieldsExtract,
    post: impl Fn(OverwriteLoginIdEvent) -> S,
) -> MethodResult<S> {
    let fields = fields
        .convert()
        .map_err(|err| post(OverwriteLoginIdEvent::Invalid(err)))?;

    if infra
        .login_id_repository()
        .check_login_id_registered(&fields.new_login_id)
        .await
        .map_err(|err| post(OverwriteLoginIdEvent::RepositoryError(err)))?
    {
        return Err(post(OverwriteLoginIdEvent::AlreadyRegistered));
    }

    let user = infra
        .login_id_repository()
        .lookup_user(&fields.login_id)
        .await
        .map_err(|err| post(OverwriteLoginIdEvent::RepositoryError(err)))?
        .ok_or_else(|| post(OverwriteLoginIdEvent::NotFound))?;

    infra
        .login_id_repository()
        .overwrite_login_id(fields.new_login_id, user)
        .await
        .map_err(|err| post(OverwriteLoginIdEvent::RepositoryError(err)))?;

    Ok(post(OverwriteLoginIdEvent::Success))
}
