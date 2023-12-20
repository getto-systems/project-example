use std::sync::Arc;

use crate::x_content::menu::badge::GatherOutlineMenuBadgeAction;

use crate::common::outline::load::action::LoadOutlineMenuBadgeAction;

use crate::common::outline::load::infra::LoadOutlineMenuBadgeLogger;

use crate::common::{
    api::repository::data::RepositoryError, outline::load::data::OutlineMenuBadge,
};

impl LoadOutlineMenuBadgeAction {
    pub fn new(action: GatherOutlineMenuBadgeAction) -> Self {
        Self {
            logger: Arc::new(NoopLogger),
            action,
        }
    }

    pub fn with_logger(self, logger: Arc<dyn LoadOutlineMenuBadgeLogger>) -> Self {
        Self { logger, ..self }
    }
}

struct NoopLogger;

impl LoadOutlineMenuBadgeLogger for NoopLogger {
    fn try_to_load_outline_menu_badge(&self) {}
    fn failed_to_gather_outline_menu_badge(&self, err: RepositoryError) -> RepositoryError {
        err
    }
    fn succeed_to_load_outline_menu_badge(&self, success: OutlineMenuBadge) -> OutlineMenuBadge {
        success
    }
}
