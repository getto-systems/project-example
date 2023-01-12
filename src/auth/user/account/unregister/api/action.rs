use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::common::proxy::action::CoreProxyParams;

use crate::auth::ticket::authorize::proxy::{authorize, AuthorizeEvent, AuthorizeInfra};

use crate::auth::user::account::unregister::infra::{
    DiscardAuthTicketRepository, UnregisterAuthUserAccountFieldsExtract,
    UnregisterAuthUserAccountRepository,
};

use crate::{
    auth::{
        ticket::kernel::data::{AuthPermissionRequired, AuthorizeTokenExtract},
        user::login_id::kernel::data::ValidateLoginIdError,
    },
    common::api::repository::data::RepositoryError,
};

pub enum UnregisterAuthUserAccountState {
    Authorize(AuthorizeEvent),
    UnregisterUser(UnregisterAuthUserAccountEvent),
}

impl std::fmt::Display for UnregisterAuthUserAccountState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authorize(event) => event.fmt(f),
            Self::UnregisterUser(event) => event.fmt(f),
        }
    }
}

pub trait UnregisterAuthUserAccountMaterial {
    type Authorize: AuthorizeInfra;

    type TicketRepository: DiscardAuthTicketRepository;
    type UserRepository: UnregisterAuthUserAccountRepository;

    fn authorize(&self) -> &Self::Authorize;

    fn ticket_repository(&self) -> &Self::TicketRepository;
    fn user_repository(&self) -> &Self::UserRepository;
}

pub struct UnregisterAuthUserAccountAction<M: UnregisterAuthUserAccountMaterial> {
    pub info: UnregisterAuthUserAccountActionInfo,
    pubsub: ActionStatePubSub<UnregisterAuthUserAccountState>,
    material: M,
}

pub struct UnregisterAuthUserAccountActionInfo;

impl UnregisterAuthUserAccountActionInfo {
    pub const fn name(&self) -> &'static str {
        "auth.user.account.unregister"
    }

    pub fn required(&self) -> AuthPermissionRequired {
        AuthPermissionRequired::user()
    }

    pub fn params(&self) -> CoreProxyParams {
        (self.name(), self.required())
    }
}

impl<M: UnregisterAuthUserAccountMaterial> UnregisterAuthUserAccountAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            info: UnregisterAuthUserAccountActionInfo,
            pubsub: ActionStatePubSub::new(),
            material,
        }
    }

    pub fn subscribe(
        &mut self,
        handler: impl 'static + Fn(&UnregisterAuthUserAccountState) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(
        self,
        token: impl AuthorizeTokenExtract,
        fields: impl UnregisterAuthUserAccountFieldsExtract,
    ) -> MethodResult<UnregisterAuthUserAccountState> {
        authorize(
            self.material.authorize(),
            (token, self.info.required()),
            |event| {
                self.pubsub
                    .post(UnregisterAuthUserAccountState::Authorize(event))
            },
        )
        .await?;

        unregister_user(&self.material, fields, |event| {
            self.pubsub
                .post(UnregisterAuthUserAccountState::UnregisterUser(event))
        })
        .await
    }
}

pub enum UnregisterAuthUserAccountEvent {
    Success,
    Invalid(ValidateLoginIdError),
    RepositoryError(RepositoryError),
}

mod unregister_auth_user_account_event {
    use super::UnregisterAuthUserAccountEvent;

    const SUCCESS: &'static str = "unregister auth user account success";
    const ERROR: &'static str = "unregister auth user account error";

    impl std::fmt::Display for UnregisterAuthUserAccountEvent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Success => write!(f, "{}", SUCCESS),
                Self::Invalid(err) => write!(f, "{}; invalid; {}", ERROR, err),
                Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
            }
        }
    }
}

async fn unregister_user<S>(
    infra: &impl UnregisterAuthUserAccountMaterial,
    fields: impl UnregisterAuthUserAccountFieldsExtract,
    post: impl Fn(UnregisterAuthUserAccountEvent) -> S,
) -> MethodResult<S> {
    let fields = fields
        .convert()
        .map_err(|err| post(UnregisterAuthUserAccountEvent::Invalid(err)))?;

    if let Some(user_id) = infra
        .user_repository()
        .lookup_user_id(&fields.login_id)
        .await
        .map_err(|err| post(UnregisterAuthUserAccountEvent::RepositoryError(err)))?
    {
        infra
            .user_repository()
            .unregister_user(&user_id, &fields.login_id)
            .await
            .map_err(|err| post(UnregisterAuthUserAccountEvent::RepositoryError(err)))?;

        infra
            .ticket_repository()
            .discard_all(&user_id)
            .await
            .map_err(|err| post(UnregisterAuthUserAccountEvent::RepositoryError(err)))?;
    }

    Ok(post(UnregisterAuthUserAccountEvent::Success))
}
