use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::ticket::validate::method::{authenticate, AuthenticateEvent, AuthenticateInfra};

use crate::auth::user::account::search::infra::{
    SearchAuthUserAccountRepository, SearchAuthUserAccountRequestDecoder,
};

use crate::{
    auth::{
        ticket::kernel::data::PermissionError,
        user::{
            account::search::data::{AuthUserAccountSearch, SearchAuthUserAccountFilterExtract},
            kernel::data::RequireAuthRoles,
        },
    },
    z_lib::repository::data::RepositoryError,
};

pub enum SearchAuthUserAccountState {
    Authenticate(AuthenticateEvent),
    PermissionError(PermissionError),
    Search(SearchAuthUserAccountEvent),
}

impl std::fmt::Display for SearchAuthUserAccountState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authenticate(event) => event.fmt(f),
            Self::PermissionError(err) => err.fmt(f),
            Self::Search(event) => event.fmt(f),
        }
    }
}

pub trait SearchAuthUserAccountMaterial {
    type Authenticate: AuthenticateInfra;
    type SearchRepository: SearchAuthUserAccountRepository;

    fn authenticate(&self) -> &Self::Authenticate;
    fn search_repository(&self) -> &Self::SearchRepository;
}

pub struct SearchAuthUserAccountAction<
    R: SearchAuthUserAccountRequestDecoder,
    M: SearchAuthUserAccountMaterial,
> {
    pubsub: ActionStatePubSub<SearchAuthUserAccountState>,
    request_decoder: R,
    material: M,
}

impl<R: SearchAuthUserAccountRequestDecoder, M: SearchAuthUserAccountMaterial>
    SearchAuthUserAccountAction<R, M>
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
        handler: impl 'static + Fn(&SearchAuthUserAccountState) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(self) -> MethodResult<SearchAuthUserAccountState> {
        let pubsub = self.pubsub;

        let fields = self.request_decoder.decode();

        let ticket = authenticate(self.material.authenticate(), |event| {
            pubsub.post(SearchAuthUserAccountState::Authenticate(event))
        })
        .await?;

        ticket
            .check_enough_permission(RequireAuthRoles::user())
            .map_err(|err| pubsub.post(SearchAuthUserAccountState::PermissionError(err)))?;

        search_user_account(&self.material, fields, |event| {
            pubsub.post(SearchAuthUserAccountState::Search(event))
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
    filter: SearchAuthUserAccountFilterExtract,
    post: impl Fn(SearchAuthUserAccountEvent) -> S,
) -> MethodResult<S> {
    let filter = filter.into();

    let search_repository = infra.search_repository();
    let response = search_repository
        .search(filter)
        .await
        .map_err(|err| post(SearchAuthUserAccountEvent::RepositoryError(err)))?;

    Ok(post(SearchAuthUserAccountEvent::Success(response)))
}
