use getto_application::{data::MethodResult, infra::ActionStatePubSub};

use crate::auth::user::account::remote::search::{
    event::SearchUserAccountEvent,
    infra::{SearchUserAccountInfra, SearchUserAccountRequestDecoder},
    method::search_user_account,
};

pub enum SearchUserAccountState {
    Search(SearchUserAccountEvent),
}

impl std::fmt::Display for SearchUserAccountState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Search(event) => write!(f, "{}", event),
        }
    }
}

pub trait SearchUserAccountMaterial {
    type Search: SearchUserAccountInfra;

    fn search(&self) -> &Self::Search;
}

pub struct SearchUserAccountAction<M: SearchUserAccountMaterial> {
    pubsub: ActionStatePubSub<SearchUserAccountState>,
    material: M,
}

impl<M: SearchUserAccountMaterial> SearchUserAccountAction<M> {
    pub fn with_material(material: M) -> Self {
        Self {
            pubsub: ActionStatePubSub::new(),
            material,
        }
    }

    pub fn subscribe(&mut self, handler: impl 'static + Fn(&SearchUserAccountState) + Send + Sync) {
        self.pubsub.subscribe(handler);
    }

    pub async fn ignite(
        self,
        request: impl SearchUserAccountRequestDecoder,
    ) -> MethodResult<SearchUserAccountState> {
        let pubsub = self.pubsub;
        let m = self.material;

        let fields = request.decode();

        search_user_account(m.search(), fields, |event| {
            pubsub.post(SearchUserAccountState::Search(event))
        })
        .await
    }
}
