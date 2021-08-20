use crate::outline::_example::get_menu_badge::infra::OutlineMenuBadgeRepository;

use crate::{
    outline::_common::data::{OutlineMenuBadge, OutlineMenuBadgeCount},
    z_details::_common::repository::data::RepositoryError,
};

pub struct UnitedOutlineMenuBadgeRepository;

#[async_trait::async_trait]
impl OutlineMenuBadgeRepository for UnitedOutlineMenuBadgeRepository {
    async fn get_menu_badge(&self) -> Result<OutlineMenuBadge, RepositoryError> {
        // unite した Repository 実装から、badge count を取得する
        Ok(OutlineMenuBadge {
            index: OutlineMenuBadgeCount::restore(4649),
        })
    }
}

#[cfg(test)]
pub mod test {
    use crate::outline::_example::get_menu_badge::infra::OutlineMenuBadgeRepository;

    use crate::{
        outline::_common::data::{OutlineMenuBadge, OutlineMenuBadgeCount},
        z_details::_common::repository::data::RepositoryError,
    };

    pub struct StaticOutlineMenuBadgeRepository;

    #[async_trait::async_trait]
    impl OutlineMenuBadgeRepository for StaticOutlineMenuBadgeRepository {
        async fn get_menu_badge(&self) -> Result<OutlineMenuBadge, RepositoryError> {
            Ok(OutlineMenuBadge {
                index: OutlineMenuBadgeCount::restore(0),
            })
        }
    }
}
