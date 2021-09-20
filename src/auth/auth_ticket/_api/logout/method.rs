use getto_application::data::MethodResult;

use crate::auth::auth_ticket::{
    _api::logout::infra::{LogoutInfra, LogoutService},
    _common::kernel::infra::AuthServiceMetadata,
};

use super::event::LogoutEvent;

pub async fn logout<S>(
    infra: &impl LogoutInfra,
    post: impl Fn(LogoutEvent) -> S,
) -> MethodResult<S> {
    let service_metadata = infra.service_metadata();
    let logout_service = infra.logout_service();

    let metadata = service_metadata
        .metadata()
        .map_err(|err| post(LogoutEvent::MetadataError(err)))?;

    logout_service
        .logout(metadata)
        .await
        .map_err(|err| post(LogoutEvent::ServiceError(err)))?;

    Ok(post(LogoutEvent::Success))
}
