use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::common::proxy::action::CoreProxyParams;

use crate::auth::ticket::authorize::proxy::{authorize, AuthorizeEvent, AuthorizeInfra};

use crate::auth::user::account::search::infra::{
    SearchAuthUserAccountFilterExtract, SearchAuthUserAccountRepository,
};

use crate::{
    auth::{
        ticket::kernel::data::{AuthPermissionRequired, AuthorizeTokenExtract},
        user::account::search::data::AuthUserAccountSearch,
    },
    common::api::repository::data::RepositoryError,
};

pub enum SearchAuthUserAccountState {
    Authorize(AuthorizeEvent),
    Search(SearchAuthUserAccountEvent),
}

impl std::fmt::Display for SearchAuthUserAccountState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authorize(event) => event.fmt(f),
            Self::Search(event) => event.fmt(f),
        }
    }
}

pub trait SearchAuthUserAccountMaterial {
    type Authorize: AuthorizeInfra;
    type UserRepository: SearchAuthUserAccountRepository;

    fn authorize(&self) -> &Self::Authorize;
    fn user_repository(&self) -> &Self::UserRepository;
}

pub struct SearchAuthUserAccountAction<M: SearchAuthUserAccountMaterial> {
    pub info: SearchAuthUserAccountActionInfo,
    pubsub: ActionStatePubSub<SearchAuthUserAccountState>,
    material: M,
}

pub struct SearchAuthUserAccountActionInfo;

impl SearchAuthUserAccountActionInfo {
    pub const fn name(&self) -> &'static str {
        "auth.user.account.search"
    }

    pub fn required(&self) -> AuthPermissionRequired {
        AuthPermissionRequired::user()
    }

    pub fn params(&self) -> CoreProxyParams {
        (self.name(), self.required())
    }
}

impl<M: SearchAuthUserAccountMaterial> SearchAuthUserAccountAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            info: SearchAuthUserAccountActionInfo,
            pubsub: ActionStatePubSub::new(),
            material,
        }
    }

    pub fn subscribe(
        &mut self,
        handler: impl 'static + Fn(&SearchAuthUserAccountState) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(
        self,
        token: impl AuthorizeTokenExtract,
        filter: impl SearchAuthUserAccountFilterExtract,
    ) -> MethodResult<SearchAuthUserAccountState> {
        authorize(
            self.material.authorize(),
            (token, self.info.required()),
            |event| {
                self.pubsub
                    .post(SearchAuthUserAccountState::Authorize(event))
            },
        )
        .await?;

        search_user_account(&self.material, filter, |event| {
            self.pubsub.post(SearchAuthUserAccountState::Search(event))
        })
        .await
    }
}

pub enum SearchAuthUserAccountEvent {
    Success(AuthUserAccountSearch),
    RepositoryError(RepositoryError),
}

const SUCCESS: &'static str = "search user account success";
const ERROR: &'static str = "search user account error";

impl std::fmt::Display for SearchAuthUserAccountEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success(_) => write!(f, "{}", SUCCESS),
            Self::RepositoryError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

async fn search_user_account<S>(
    infra: &impl SearchAuthUserAccountMaterial,
    filter: impl SearchAuthUserAccountFilterExtract,
    post: impl Fn(SearchAuthUserAccountEvent) -> S,
) -> MethodResult<S> {
    let response = infra
        .user_repository()
        .search(filter.convert())
        .await
        .map_err(|err| post(SearchAuthUserAccountEvent::RepositoryError(err)))?;

    Ok(post(SearchAuthUserAccountEvent::Success(response)))
}
