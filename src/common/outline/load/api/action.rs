use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::method::proxy::{authorize, AuthorizeEvent, AuthorizeInfra};

use crate::common::outline::load::infra::OutlineMenuBadgeRepository;

use crate::common::proxy::action::CoreProxyParams;
use crate::{
    auth::data::{AuthPermissionRequired, AuthorizeTokenExtract},
    common::api::repository::data::RepositoryError,
    common::outline::load::data::OutlineMenuBadge,
};

pub enum LoadOutlineMenuBadgeState {
    Authorize(AuthorizeEvent),
    LoadMenuBadge(LoadOutlineMenuBadgeEvent),
}

impl std::fmt::Display for LoadOutlineMenuBadgeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authorize(event) => event.fmt(f),
            Self::LoadMenuBadge(event) => event.fmt(f),
        }
    }
}

pub trait LoadOutlineMenuBadgeMaterial {
    type Authorize: AuthorizeInfra;
    type MenuBadgeRepository: OutlineMenuBadgeRepository;

    fn authorize(&self) -> &Self::Authorize;
    fn menu_badge_repository(&self) -> &Self::MenuBadgeRepository;
}

pub struct LoadOutlineMenuBadgeAction<M: LoadOutlineMenuBadgeMaterial> {
    pub info: LoadOutlineMenuBadgeActionInfo,
    pubsub: ActionStatePubSub<LoadOutlineMenuBadgeState>,
    material: M,
}

pub struct LoadOutlineMenuBadgeActionInfo;

impl LoadOutlineMenuBadgeActionInfo {
    pub const fn name(&self) -> &'static str {
        "common.outline.load"
    }

    pub fn required(&self) -> AuthPermissionRequired {
        AuthPermissionRequired::Nothing
    }

    pub fn params(&self) -> CoreProxyParams {
        (self.name(), self.required())
    }
}

impl<M: LoadOutlineMenuBadgeMaterial> LoadOutlineMenuBadgeAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            info: LoadOutlineMenuBadgeActionInfo,
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

    pub async fn ignite(
        self,
        token: impl AuthorizeTokenExtract,
    ) -> MethodResult<LoadOutlineMenuBadgeState> {
        authorize(
            self.material.authorize(),
            (token, self.info.required()),
            |event| {
                self.pubsub
                    .post(LoadOutlineMenuBadgeState::Authorize(event))
            },
        )
        .await?;

        load_menu_badge(&self.material, |event| {
            self.pubsub
                .post(LoadOutlineMenuBadgeState::LoadMenuBadge(event))
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
    let menu_badge = infra
        .menu_badge_repository()
        .load_menu_badge()
        .await
        .map_err(|err| post(LoadOutlineMenuBadgeEvent::RepositoryError(err)))?;

    Ok(post(LoadOutlineMenuBadgeEvent::Success(menu_badge)))
}
