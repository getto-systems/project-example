use std::collections::HashMap;

use crate::x_content::menu::badge::OutlineMenuBadgePath;

use crate::common::outline::load::infra::OutlineMenuBadgeRepository;

use crate::{
    common::outline::load::data::{OutlineMenuBadge, OutlineMenuBadgeCount},
    z_lib::repository::data::RepositoryError,
};

pub struct UnitedOutlineMenuBadgeRepository(
    HashMap<OutlineMenuBadgePath, Box<dyn OutlineMenuBadgeCountProvider + Sync + Send>>,
);

#[async_trait::async_trait]
pub trait OutlineMenuBadgeCountProvider {
    async fn count(&self) -> Result<OutlineMenuBadgeCount, RepositoryError>;
}

impl UnitedOutlineMenuBadgeRepository {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn register(
        &mut self,
        path: OutlineMenuBadgePath,
        provider: Box<dyn OutlineMenuBadgeCountProvider + Sync + Send>,
    ) {
        self.0.insert(path, provider);
    }
}

#[async_trait::async_trait]
impl OutlineMenuBadgeRepository for UnitedOutlineMenuBadgeRepository {
    async fn load_menu_badge(&self) -> Result<OutlineMenuBadge, RepositoryError> {
        let mut badge = OutlineMenuBadge::new();

        for (path, provider) in self.0.iter() {
            badge.set(path.clone(), provider.count().await?.clone());
        }

        Ok(badge)
    }
}

#[cfg(test)]
pub mod test {
    use crate::common::outline::load::infra::OutlineMenuBadgeRepository;

    use crate::{
        common::outline::load::data::OutlineMenuBadge, z_lib::repository::data::RepositoryError,
    };

    pub struct StaticOutlineMenuBadgeRepository;

    #[async_trait::async_trait]
    impl OutlineMenuBadgeRepository for StaticOutlineMenuBadgeRepository {
        async fn load_menu_badge(&self) -> Result<OutlineMenuBadge, RepositoryError> {
            Ok(OutlineMenuBadge::new())
        }
    }
}
