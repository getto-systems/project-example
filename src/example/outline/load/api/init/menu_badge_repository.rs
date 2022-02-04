use crate::example::outline::load::api::infra::OutlineMenuBadgeRepository;

use crate::{
    example::outline::load::api::data::{OutlineMenuBadge, OutlineMenuBadgeCount},
    z_lib::api::repository::data::RepositoryError,
};

pub struct UnitedOutlineMenuBadgeRepository;

#[async_trait::async_trait]
impl OutlineMenuBadgeRepository for UnitedOutlineMenuBadgeRepository {
    async fn load_menu_badge(&self) -> Result<OutlineMenuBadge, RepositoryError> {
        // TODO ユーザー数を返すとかにするか
        Ok(OutlineMenuBadge {
            index: OutlineMenuBadgeCount::restore(4649),
        })
    }
}

#[cfg(test)]
pub mod test {
    use crate::example::outline::load::api::infra::OutlineMenuBadgeRepository;

    use crate::{
        example::outline::load::api::data::{OutlineMenuBadge, OutlineMenuBadgeCount},
        z_lib::api::repository::data::RepositoryError,
    };

    pub struct StaticOutlineMenuBadgeRepository;

    #[async_trait::async_trait]
    impl OutlineMenuBadgeRepository for StaticOutlineMenuBadgeRepository {
        async fn load_menu_badge(&self) -> Result<OutlineMenuBadge, RepositoryError> {
            Ok(OutlineMenuBadge {
                index: OutlineMenuBadgeCount::restore(0),
            })
        }
    }
}
