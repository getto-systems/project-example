#[cfg(test)]
pub mod test {
    use crate::common::outline::load::infra::OutlineMenuBadgeRepository;

    use crate::{
        common::api::repository::data::RepositoryError,
        common::outline::load::data::OutlineMenuBadge,
    };

    pub struct StaticOutlineMenuBadgeRepository;

    #[async_trait::async_trait]
    impl OutlineMenuBadgeRepository for StaticOutlineMenuBadgeRepository {
        async fn load_menu_badge(&self) -> Result<OutlineMenuBadge, RepositoryError> {
            Ok(OutlineMenuBadge::new())
        }
    }
}
