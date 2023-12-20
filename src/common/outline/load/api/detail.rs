use crate::common::{
    api::{logger::detail::StdoutJsonLogger, repository::data::RepositoryError},
    outline::load::{data::OutlineMenuBadge, infra::LoadOutlineMenuBadgeLogger},
};

impl LoadOutlineMenuBadgeLogger for StdoutJsonLogger {
    fn try_to_load_outline_menu_badge(&self) {
        self.info(format!("try to load outline-menu-badge"));
    }
    fn failed_to_gather_outline_menu_badge(&self, err: RepositoryError) -> RepositoryError {
        self.fatal(format!("failed to collect outline-menu-badge; {}", err));
        err
    }
    fn succeed_to_load_outline_menu_badge(&self, success: OutlineMenuBadge) -> OutlineMenuBadge {
        self.info(format!("succeed to load outline-menu-badge; {}", success));
        success
    }
}
