use crate::{
    example::outline::_common::data::OutlineMenuBadge,
    z_details::_common::repository::data::RepositoryError,
};

pub trait GetOutlineMenuBadgeInfra {
    type MenuBadgeRepository: OutlineMenuBadgeRepository;

    fn menu_badge_repository(&self) -> &Self::MenuBadgeRepository;
}

#[async_trait::async_trait]
pub trait OutlineMenuBadgeRepository {
    async fn get_menu_badge(&self) -> Result<OutlineMenuBadge, RepositoryError>;
}
