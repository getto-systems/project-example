use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::{
    auth::{
        data::RequireAuthRoles,
        method::{check_permission, CheckPermissionEvent, CheckPermissionInfra},
    },
    example::outline::load::api::{data::OutlineMenuBadge, infra::OutlineMenuBadgeRepository},
    z_lib::api::repository::data::RepositoryError,
};

pub enum LoadOutlineMenuBadgeState {
    Validate(CheckPermissionEvent),
    LoadMenuBadge(LoadOutlineMenuBadgeEvent),
}

impl std::fmt::Display for LoadOutlineMenuBadgeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Validate(event) => write!(f, "{}", event),
            Self::LoadMenuBadge(event) => write!(f, "{}", event),
        }
    }
}

pub trait LoadOutlineMenuBadgeMaterial {
    type CheckPermission: CheckPermissionInfra;
    type MenuBadgeRepository: OutlineMenuBadgeRepository;

    fn check_permission(&self) -> &Self::CheckPermission;
    fn menu_badge_repository(&self) -> &Self::MenuBadgeRepository;
}

pub struct LoadOutlineMenuBadgeAction<M: LoadOutlineMenuBadgeMaterial> {
    pubsub: ActionStatePubSub<LoadOutlineMenuBadgeState>,
    material: M,
}

impl<M: LoadOutlineMenuBadgeMaterial> LoadOutlineMenuBadgeAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            material,
        }
    }

    pub fn subscribe(
        &mut self,
        handler: impl 'static + Fn(&LoadOutlineMenuBadgeState) + Send + Sync,
    ) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(self) -> MethodResult<LoadOutlineMenuBadgeState> {
        let pubsub = self.pubsub;
        let m = self.material;

        check_permission(m.check_permission(), RequireAuthRoles::Nothing, |event| {
            pubsub.post(LoadOutlineMenuBadgeState::Validate(event))
        })
        .await?;

        load_menu_badge(&m, |event| {
            pubsub.post(LoadOutlineMenuBadgeState::LoadMenuBadge(event))
        })
        .await
    }
}

pub enum LoadOutlineMenuBadgeEvent {
    Success(OutlineMenuBadge),
    RepositoryError(RepositoryError),
}

const SUCCESS: &'static str = "load menu badge success";
const ERROR: &'static str = "load menu badge error";

impl std::fmt::Display for LoadOutlineMenuBadgeEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success(_) => write!(f, "{}", SUCCESS),
            Self::RepositoryError(err) => write!(f, "{}: {}", ERROR, err),
        }
    }
}

async fn load_menu_badge<S>(
    infra: &impl LoadOutlineMenuBadgeMaterial,
    post: impl Fn(LoadOutlineMenuBadgeEvent) -> S,
) -> MethodResult<S> {
    let menu_badge_repository = infra.menu_badge_repository();

    let menu_badge = menu_badge_repository
        .load_menu_badge()
        .await
        .map_err(|err| post(LoadOutlineMenuBadgeEvent::RepositoryError(err)))?;

    Ok(post(LoadOutlineMenuBadgeEvent::Success(menu_badge)))
}
