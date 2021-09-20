use getto_application::data::MethodResult;

use crate::{
    auth::_common::{data::RequireAuthRoles, method::validate_api_token},
    example::outline::_example::get_menu_badge::infra::{
        GetOutlineMenuBadgeInfra, OutlineMenuBadgeRepository,
    },
};

use super::event::GetOutlineMenuBadgeEvent;

pub async fn get_outline_menu_badge<S>(
    infra: &impl GetOutlineMenuBadgeInfra,
    post: impl Fn(GetOutlineMenuBadgeEvent) -> S,
) -> MethodResult<S> {
    validate_api_token(infra.validate_infra(), RequireAuthRoles::Nothing)
        .await
        .map_err(|err| post(GetOutlineMenuBadgeEvent::ValidateError(err)))?;

    let menu_badge_repository = infra.menu_badge_repository();

    let menu_badge = menu_badge_repository
        .get_menu_badge()
        .await
        .map_err(|err| post(GetOutlineMenuBadgeEvent::RepositoryError(err)))?;

    Ok(post(GetOutlineMenuBadgeEvent::Success(menu_badge)))
}
