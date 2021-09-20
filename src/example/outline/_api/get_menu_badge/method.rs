use getto_application::data::MethodResult;

use crate::{
    auth::_common::infra::AuthServiceMetadata,
    example::outline::_api::get_menu_badge::infra::{
        GetOutlineMenuBadgeInfra, GetOutlineMenuBadgeResponseEncoder, GetOutlineMenuBadgeService,
    },
};

use super::event::GetOutlineMenuBadgeEvent;

pub async fn get_outline_menu_badge<S>(
    infra: &impl GetOutlineMenuBadgeInfra,
    post: impl Fn(GetOutlineMenuBadgeEvent) -> S,
) -> MethodResult<S> {
    let service_metadata = infra.service_metadata();
    let get_menu_service = infra.get_menu_service();
    let response_encoder = infra.response_encoder();

    let metadata = service_metadata
        .metadata()
        .map_err(|err| post(GetOutlineMenuBadgeEvent::MetadataError(err)))?;

    let menu = get_menu_service
        .get_menu(metadata)
        .await
        .map_err(|err| post(GetOutlineMenuBadgeEvent::ServiceError(err)))?;

    let response = response_encoder
        .encode(menu)
        .map_err(|err| post(GetOutlineMenuBadgeEvent::MessageError(err)))?;

    Ok(post(GetOutlineMenuBadgeEvent::Success(response)))
}
