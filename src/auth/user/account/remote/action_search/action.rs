use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::user::account::remote::search::{
    event::SearchAuthUserAccountEvent,
    infra::{SearchAuthUserAccountInfra, SearchAuthUserAccountRequestDecoder},
    method::search_user_account,
};

pub enum SearchAuthUserAccountState {
    Search(SearchAuthUserAccountEvent),
}

impl std::fmt::Display for SearchAuthUserAccountState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Search(event) => write!(f, "{}", event),
        }
    }
}

pub trait SearchAuthUserAccountMaterial {
    type Search: SearchAuthUserAccountInfra;

    fn search(&self) -> &Self::Search;
}

pub struct SearchAuthUserAccountAction<M: SearchAuthUserAccountMaterial> {
    pubsub: ActionStatePubSub<SearchAuthUserAccountState>,
    material: M,
}

impl<M: SearchAuthUserAccountMaterial> SearchAuthUserAccountAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            material,
        }
    }

    pub fn subscribe(&mut self, handler: impl 'static + Fn(&SearchAuthUserAccountState) + Send + Sync) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(
        self,
        request: impl SearchAuthUserAccountRequestDecoder,
    ) -> MethodResult<SearchAuthUserAccountState> {
        let pubsub = self.pubsub;
        let m = self.material;

        let fields = request.decode();

        search_user_account(m.search(), fields, |event| {
            pubsub.post(SearchAuthUserAccountState::Search(event))
        })
        .await
    }
}
