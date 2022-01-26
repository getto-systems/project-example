use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::{
    auth::{
        ticket::remote::validate::method::{
            validate_auth_token, ValidateAuthTokenEvent, ValidateAuthTokenInfra,
        },
        user::{
            account::remote::{
                search::data::SearchAuthUserAccountBasket,
                search::infra::{
                    SearchAuthUserAccountFieldsExtract, SearchAuthUserAccountRepository,
                    SearchAuthUserAccountRequestDecoder,
                },
            },
            remote::kernel::data::RequireAuthRoles,
        },
    },
    z_lib::remote::repository::data::RepositoryError,
};

pub enum SearchAuthUserAccountState {
    Validate(ValidateAuthTokenEvent),
    Search(SearchAuthUserAccountEvent),
}

impl std::fmt::Display for SearchAuthUserAccountState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Validate(event) => event.fmt(f),
            Self::Search(event) => event.fmt(f),
        }
    }
}

pub trait SearchAuthUserAccountMaterial {
    type Validate: ValidateAuthTokenInfra;

    type SearchRepository: SearchAuthUserAccountRepository;

    fn validate(&self) -> &Self::Validate;

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
        let m = self.material;

        let fields = self.request_decoder.decode();

        validate_auth_token(
            m.validate(),
            RequireAuthRoles::Nothing, // TODO RequireAuthRoles::manage_auth_user(),
            |event| pubsub.post(SearchAuthUserAccountState::Validate(event)),
        )
        .await?;

        search_user_account(&m, fields, |event| {
            pubsub.post(SearchAuthUserAccountState::Search(event))
        })
        .await
    }
}

pub enum SearchAuthUserAccountEvent {
    Success(SearchAuthUserAccountBasket),
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
    fields: SearchAuthUserAccountFieldsExtract,
    post: impl Fn(SearchAuthUserAccountEvent) -> S,
) -> MethodResult<S> {
    let fields = fields.into();

    let search_repository = infra.search_repository();
    let response = search_repository
        .search(&fields)
        .await
        .map_err(|err| post(SearchAuthUserAccountEvent::RepositoryError(err)))?;

    Ok(post(SearchAuthUserAccountEvent::Success(response)))
}
