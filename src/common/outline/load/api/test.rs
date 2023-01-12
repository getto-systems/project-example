use getto_application_test::ApplicationActionStateHolder;
use pretty_assertions::assert_eq;

use crate::{
    auth::init::test::{StaticAuthorizeInfra, StaticAuthorizeToken},
    common::outline::load::init::test::{
        StaticLoadOutlineMenuBadgeMaterial, StaticOutlineMenuBadgeRepository,
    },
};

use crate::common::outline::load::action::LoadOutlineMenuBadgeAction;

#[tokio::test]
async fn info() {
    let material = StaticLoadOutlineMenuBadgeMaterial {
        authorize: StaticAuthorizeInfra::standard(),
        menu_badge_repository: StaticOutlineMenuBadgeRepository,
    };

    let action = LoadOutlineMenuBadgeAction::with_material(material);

    let (name, required) = action.info.params();
    assert_eq!(
        format!("{}; {}", name, required),
        "common.outline.load; require: nothing",
    );
}

#[tokio::test]
async fn success_load_menu_badge() {
    let holder = ApplicationActionStateHolder::new();

    let material = StaticLoadOutlineMenuBadgeMaterial {
        authorize: StaticAuthorizeInfra::standard(),
        menu_badge_repository: StaticOutlineMenuBadgeRepository,
    };

    let mut action = LoadOutlineMenuBadgeAction::with_material(material);
    action.subscribe(holder.handler());

    let result = action.ignite(StaticAuthorizeToken).await;
    assert_eq!(
        holder.extract(),
        vec![
            "try to proxy call: auth.ticket.authorize.clarify(require: nothing)",
            "proxy call success",
            "load menu badge success",
        ],
    );
    assert!(result.is_ok());
}
