use crate::common::outline::load::infra::OutlineMenuBadgeRepository;

use crate::{
    common::api::repository::data::RepositoryError,
    common::outline::load::data::{OutlineMenuBadge, OutlineMenuBadgeCount},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum OutlineMenuBadgePath {
    Index,
}

impl OutlineMenuBadgePath {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Index => "index",
        }
    }

    pub fn extract(self) -> String {
        self.as_str().to_owned()
    }
}

#[async_trait::async_trait]
pub trait OutlineMenuExampleBadgeRepository {
    async fn index(&self) -> Result<OutlineMenuBadgeCount, RepositoryError>;
}

pub struct ActiveOutlineMenuBadgeRepository {
    example: ActiveOutlineMenuExampleBadgeRepository,
}

impl ActiveOutlineMenuBadgeRepository {
    pub const fn new() -> Self {
        Self {
            example: ActiveOutlineMenuExampleBadgeRepository,
        }
    }
}

#[async_trait::async_trait]
impl OutlineMenuBadgeRepository for ActiveOutlineMenuBadgeRepository {
    async fn load_menu_badge(&self) -> Result<OutlineMenuBadge, RepositoryError> {
        let mut badge = OutlineMenuBadge::new();

        badge.set(OutlineMenuBadgePath::Index, self.example.index().await?);

        Ok(badge)
    }
}

struct ActiveOutlineMenuExampleBadgeRepository;

#[async_trait::async_trait]
impl OutlineMenuExampleBadgeRepository for ActiveOutlineMenuExampleBadgeRepository {
    async fn index(&self) -> Result<OutlineMenuBadgeCount, RepositoryError> {
        Ok(OutlineMenuBadgeCount::restore(0))
    }
}
