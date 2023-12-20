use crate::{
    common::api::repository::data::RepositoryError, common::outline::load::data::OutlineMenuBadge,
};

pub trait LoadOutlineMenuBadgeLogger: Send + Sync {
    fn try_to_load_outline_menu_badge(&self);
    fn failed_to_gather_outline_menu_badge(&self, err: RepositoryError) -> RepositoryError;
    fn succeed_to_load_outline_menu_badge(&self, success: OutlineMenuBadge) -> OutlineMenuBadge;
}
