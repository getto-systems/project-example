mod detail;

use std::sync::Arc;

use crate::x_content::menu::badge::GatherOutlineMenuBadgeAction;

use crate::common::outline::load::infra::LoadOutlineMenuBadgeLogger;

use crate::{
    auth::data::AuthPermissionRequired,
    common::outline::load::data::{LoadOutlineMenuBadgeError, OutlineMenuBadge},
};

pub struct LoadOutlineMenuBadgeAction {
    logger: Arc<dyn LoadOutlineMenuBadgeLogger>,
    action: GatherOutlineMenuBadgeAction,
}

pub struct LoadOutlineMenuBadgeInfo;

impl LoadOutlineMenuBadgeInfo {
    pub fn required() -> AuthPermissionRequired {
        AuthPermissionRequired::Nothing
    }
}

impl LoadOutlineMenuBadgeAction {
    pub async fn load(&self) -> Result<OutlineMenuBadge, LoadOutlineMenuBadgeError> {
        self.logger.try_to_load_outline_menu_badge();

        let badge = self
            .action
            .gather()
            .await
            .map_err(|err| self.logger.failed_to_gather_outline_menu_badge(err))?;

        Ok(self
            .logger
            .succeed_to_load_outline_menu_badge(OutlineMenuBadge::new(badge)))
    }
}
