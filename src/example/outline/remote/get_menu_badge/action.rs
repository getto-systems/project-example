use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::{
    auth::remote::{
        data::{RequireAuthRoles, ValidateApiTokenError},
        infra::ValidateApiTokenInfra,
        method::validate_api_token,
    },
    example::outline::remote::get_menu_badge::{data::OutlineMenuBadge, infra::OutlineMenuBadgeRepository},
    z_lib::remote::repository::data::RepositoryError,
};

pub enum GetOutlineMenuBadgeState {
    Validate(ValidateApiTokenError),
    GetMenuBadge(GetOutlineMenuBadgeEvent),
}

impl std::fmt::Display for GetOutlineMenuBadgeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Validate(err) => write!(f, "{}", err),
            Self::GetMenuBadge(event) => write!(f, "{}", event),
        }
    }
}

pub trait GetOutlineMenuBadgeMaterial {
    type Validate: ValidateApiTokenInfra;

    type MenuBadgeRepository: OutlineMenuBadgeRepository;

    fn validate(&self) -> &Self::Validate;

    fn menu_badge_repository(&self) -> &Self::MenuBadgeRepository;
}

pub struct GetOutlineMenuBadgeAction<M: GetOutlineMenuBadgeMaterial> {
    pubsub: ActionStatePubSub<GetOutlineMenuBadgeState>,
    material: M,
}

impl<M: GetOutlineMenuBadgeMaterial> GetOutlineMenuBadgeAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            material,
        }
    }

    pub fn subscribe(
        &mut self,
        handler: impl 'static + Fn(&GetOutlineMenuBadgeState) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(self) -> MethodResult<GetOutlineMenuBadgeState> {
        let pubsub = self.pubsub;
        let m = self.material;

        validate_api_token(m.validate(), RequireAuthRoles::Nothing)
            .await
            .map_err(|err| pubsub.post(GetOutlineMenuBadgeState::Validate(err)))?;

        get_outline_menu_badge(&m, |event| {
            pubsub.post(GetOutlineMenuBadgeState::GetMenuBadge(event))
        })
        .await
    }
}

pub enum GetOutlineMenuBadgeEvent {
    Success(OutlineMenuBadge),
    RepositoryError(RepositoryError),
}

const SUCCESS: &'static str = "get menu badge success";
const ERROR: &'static str = "get menu badge error";

impl std::fmt::Display for GetOutlineMenuBadgeEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success(_) => write!(f, "{}", SUCCESS),
            Self::RepositoryError(err) => write!(f, "{}: {}", ERROR, err),
        }
    }
}

async fn get_outline_menu_badge<S>(
    infra: &impl GetOutlineMenuBadgeMaterial,
    post: impl Fn(GetOutlineMenuBadgeEvent) -> S,
) -> MethodResult<S> {
    let menu_badge_repository = infra.menu_badge_repository();

    let menu_badge = menu_badge_repository
        .get_menu_badge()
        .await
        .map_err(|err| post(GetOutlineMenuBadgeEvent::RepositoryError(err)))?;

    Ok(post(GetOutlineMenuBadgeEvent::Success(menu_badge)))
}