use crate::{
    common::api::repository::data::RepositoryError, common::outline::load::data::OutlineMenuBadge,
};

#[async_trait::async_trait]
pub trait OutlineMenuBadgeRepository {
    async fn load_menu_badge(&self) -> Result<OutlineMenuBadge, RepositoryError>;
}
