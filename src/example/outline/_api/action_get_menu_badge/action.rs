use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::example::outline::_api::get_menu_badge::{
    event::GetOutlineMenuBadgeEvent, infra::GetOutlineMenuBadgeInfra,
    method::get_outline_menu_badge,
};

pub enum GetOutlineMenuBadgeState {
    GetMenuBadge(GetOutlineMenuBadgeEvent),
}

impl std::fmt::Display for GetOutlineMenuBadgeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GetMenuBadge(event) => write!(f, "{}", event),
        }
    }
}

pub trait GetOutlineMenuBadgeMaterial {
    type GetMenuBadge: GetOutlineMenuBadgeInfra;

    fn get_menu_badge(&self) -> &Self::GetMenuBadge;
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

        get_outline_menu_badge(m.get_menu_badge(), |event| {
            pubsub.post(GetOutlineMenuBadgeState::GetMenuBadge(event))
        })
        .await
    }
}
