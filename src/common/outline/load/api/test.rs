use pretty_assertions::assert_eq;

use crate::x_content::menu::badge::GatherOutlineMenuBadgeAction;

use crate::common::outline::load::action::{LoadOutlineMenuBadgeAction, LoadOutlineMenuBadgeInfo};

use crate::{
    auth::data::AuthPermissionRequired, common::outline::load::data::LoadOutlineMenuBadgeError,
};

#[tokio::test]
async fn info() {
    assert_eq!(
        LoadOutlineMenuBadgeInfo::required(),
        AuthPermissionRequired::Nothing,
    );
}

#[tokio::test]
async fn success() -> Result<(), LoadOutlineMenuBadgeError> {
    let action = LoadOutlineMenuBadgeAction::new(GatherOutlineMenuBadgeAction::mock());

    action.load().await?;

    Ok(())
}
