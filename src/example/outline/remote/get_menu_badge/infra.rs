use crate::{
    example::outline::remote::get_menu_badge::data::OutlineMenuBadge,
    z_lib::remote::repository::data::RepositoryError,
};

#[async_trait::async_trait]
pub trait OutlineMenuBadgeRepository {
    async fn get_menu_badge(&self) -> Result<OutlineMenuBadge, RepositoryError>;
}
