use crate::{
    core::outline::load::api::data::OutlineMenuBadge,
    z_lib::api::repository::data::RepositoryError,
};

#[async_trait::async_trait]
pub trait OutlineMenuBadgeRepository {
    async fn load_menu_badge(&self) -> Result<OutlineMenuBadge, RepositoryError>;
}
