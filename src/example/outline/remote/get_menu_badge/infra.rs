use crate::{
    auth::remote::infra::ValidateApiTokenInfra,
    example::outline::remote::get_menu_badge::data::OutlineMenuBadge,
    z_lib::remote::repository::data::RepositoryError,
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
