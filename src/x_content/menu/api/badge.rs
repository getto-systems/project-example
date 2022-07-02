use crate::common::outline::load::init::menu_badge_repository::{
    OutlineMenuBadgeCountProvider, UnitedOutlineMenuBadgeRepository,
};

use crate::{
    common::outline::load::data::OutlineMenuBadgeCount, z_lib::repository::data::RepositoryError,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum OutlineMenuBadgePath {
    Index,
}

impl ToString for OutlineMenuBadgePath {
    fn to_string(&self) -> String {
        match self {
            Self::Index => "index".to_owned(),
        }
    }
}

impl UnitedOutlineMenuBadgeRepository {
    pub fn build(mut self) -> Self {
        self.register(OutlineMenuBadgePath::Index, ExampleProvider::boxed());
        self
    }
}

struct ExampleProvider;

impl ExampleProvider {
    fn boxed() -> Box<dyn OutlineMenuBadgeCountProvider + Sync + Send> {
        Box::new(Self)
    }
}

#[async_trait::async_trait]
impl OutlineMenuBadgeCountProvider for ExampleProvider {
    async fn count(&self) -> Result<OutlineMenuBadgeCount, RepositoryError> {
        Ok(OutlineMenuBadgeCount::restore(4649))
    }
}
