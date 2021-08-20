use getto_application::data::MethodResult;

use crate::auth::_api::common::{data::RequireAuthRoles, method::validate_api_token};

use super::{
    event::GetOutlineMenuBadgeEvent,
    infra::{
        GetOutlineMenuBadgeInfra, GetOutlineMenuBadgeResponseEncoder, GetOutlineMenuBadgeService,
    },
};

pub async fn get_outline_menu_badge<S>(
    infra: &impl GetOutlineMenuBadgeInfra,
    post: impl Fn(GetOutlineMenuBadgeEvent) -> S,
) -> MethodResult<S> {
    let get_menu_service = infra.get_menu_service();
    let response_encoder = infra.response_encoder();

    let user_id = validate_api_token(infra.validate_infra(), RequireAuthRoles::Nothing)
        .await
        .map_err(|err| post(GetOutlineMenuBadgeEvent::ValidateApiTokenError(err)))?;
    post(GetOutlineMenuBadgeEvent::Authorized(user_id));

    let menu = get_menu_service
        .get_menu()
        .await
        .map_err(|err| post(GetOutlineMenuBadgeEvent::ServiceError(err)))?;

    let response = response_encoder
        .encode(menu)
        .map_err(|err| post(GetOutlineMenuBadgeEvent::MessageError(err)))?;

    Ok(post(GetOutlineMenuBadgeEvent::Success(response)))
}
