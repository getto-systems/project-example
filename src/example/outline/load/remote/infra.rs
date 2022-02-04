use crate::{
    example::outline::load::remote::data::OutlineMenuBadge,
    z_lib::remote::repository::data::RepositoryError,
};

#[async_trait::async_trait]
pub trait OutlineMenuBadgeRepository {
    async fn load_menu_badge(&self) -> Result<OutlineMenuBadge, RepositoryError>;
}
