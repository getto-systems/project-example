use crate::{
    auth::_common::infra::ValidateApiTokenInfra, example::outline::_common::data::OutlineMenuBadge,
    z_details::_common::repository::data::RepositoryError,
};

pub trait GetOutlineMenuBadgeInfra {
    type ValidateInfra: ValidateApiTokenInfra;
    type MenuBadgeRepository: OutlineMenuBadgeRepository;

    fn validate_infra(&self) -> &Self::ValidateInfra;
    fn menu_badge_repository(&self) -> &Self::MenuBadgeRepository;
}

#[async_trait::async_trait]
pub trait OutlineMenuBadgeRepository {
    async fn get_menu_badge(&self) -> Result<OutlineMenuBadge, RepositoryError>;
}
