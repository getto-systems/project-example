use crate::{
    common::outline::load::data::OutlineMenuBadge,
    z_lib::repository::data::RepositoryError,
};

#[async_trait::async_trait]
pub trait OutlineMenuBadgeRepository {
    async fn load_menu_badge(&self) -> Result<OutlineMenuBadge, RepositoryError>;
}
