use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::common::proxy::action::CoreProxyParams;

use crate::auth::ticket::authorize::proxy::{authorize, AuthorizeEvent, AuthorizeInfra};

use crate::auth::user::account::modify::infra::{
    ModifyAuthUserAccountFieldsExtract, ModifyAuthUserAccountRepository,
};

use crate::{
    auth::{
        ticket::kernel::data::{AuthPermissionRequired, AuthorizeTokenExtract},
        user::account::modify::data::ValidateModifyAuthUserAccountFieldsError,
    },
    common::api::repository::data::RepositoryError,
};

pub enum ModifyAuthUserAccountState {
    Authorize(AuthorizeEvent),
    ModifyUser(ModifyAuthUserAccountEvent),
}

impl std::fmt::Display for ModifyAuthUserAccountState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authorize(event) => event.fmt(f),
            Self::ModifyUser(event) => event.fmt(f),
        }
    }
}

pub trait ModifyAuthUserAccountMaterial {
    type Authorize: AuthorizeInfra;

    type UserRepository: ModifyAuthUserAccountRepository;

    fn authorize(&self) -> &Self::Authorize;

    fn user_repository(&self) -> &Self::UserRepository;
}

pub struct ModifyAuthUserAccountAction<M: ModifyAuthUserAccountMaterial> {
    pub info: ModifyAuthUserAccountActionInfo,
    pubsub: ActionStatePubSub<ModifyAuthUserAccountState>,
    material: M,
}

pub struct ModifyAuthUserAccountActionInfo;

impl ModifyAuthUserAccountActionInfo {
    pub const fn name(&self) -> &'static str {
        "auth.user.account.modify"
    }

    pub fn required(&self) -> AuthPermissionRequired {
        AuthPermissionRequired::user()
    }

    pub fn params(&self) -> CoreProxyParams {
        (self.name(), self.required())
    }
}

impl<M: ModifyAuthUserAccountMaterial> ModifyAuthUserAccountAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            info: ModifyAuthUserAccountActionInfo,
            pubsub: ActionStatePubSub::new(),
            material,
        }
    }

    pub fn subscribe(
        &mut self,
        handler: impl 'static + Fn(&ModifyAuthUserAccountState) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(
        self,
        token: impl AuthorizeTokenExtract,
        fields: impl ModifyAuthUserAccountFieldsExtract,
    ) -> MethodResult<ModifyAuthUserAccountState> {
        authorize(
            self.material.authorize(),
            (token, self.info.required()),
            |event| {
                self.pubsub
                    .post(ModifyAuthUserAccountState::Authorize(event))
            },
        )
        .await?;

        modify_user(&self.material, fields, |event| {
            self.pubsub
                .post(ModifyAuthUserAccountState::ModifyUser(event))
        })
        .await
    }
}

pub enum ModifyAuthUserAccountEvent {
    Success,
    Invalid(ValidateModifyAuthUserAccountFieldsError),
    NotFound,
    Conflict,
    RepositoryError(RepositoryError),
}

mod modify_auth_user_account_event {
    use super::ModifyAuthUserAccountEvent;

    const SUCCESS: &'static str = "modify auth user account success";
    const ERROR: &'static str = "modify auth user account error";

    impl std::fmt::Display for ModifyAuthUserAccountEvent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Success => write!(f, "{}", SUCCESS),
                Self::Invalid(err) => write!(f, "{}; {}", ERROR, err),
                Self::NotFound => write!(f, "{}; not found", ERROR),
                Self::Conflict => write!(f, "{}; changes conflicted", ERROR),
                Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
            }
        }
    }
}

async fn modify_user<S>(
    infra: &impl ModifyAuthUserAccountMaterial,
    fields: impl ModifyAuthUserAccountFieldsExtract,
    post: impl Fn(ModifyAuthUserAccountEvent) -> S,
) -> MethodResult<S> {
    let fields = fields
        .convert()
        .map_err(|err| post(ModifyAuthUserAccountEvent::Invalid(err)))?;

    let user_id = infra
        .user_repository()
        .lookup_user_id(&fields.login_id)
        .await
        .map_err(|err| post(ModifyAuthUserAccountEvent::RepositoryError(err)))?
        .ok_or_else(|| post(ModifyAuthUserAccountEvent::NotFound))?;

    let stored_user = infra
        .user_repository()
        .lookup_attrs(&user_id)
        .await
        .map_err(|err| post(ModifyAuthUserAccountEvent::RepositoryError(err)))?
        .ok_or_else(|| post(ModifyAuthUserAccountEvent::NotFound))?;

    if stored_user != fields.from {
        return Err(post(ModifyAuthUserAccountEvent::Conflict));
    }

    infra
        .user_repository()
        .modify_user(user_id, fields.to)
        .await
        .map_err(|err| post(ModifyAuthUserAccountEvent::RepositoryError(err)))?;

    Ok(post(ModifyAuthUserAccountEvent::Success))
}
