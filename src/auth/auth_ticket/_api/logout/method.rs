use getto_application::data::MethodResult;

use crate::auth::auth_ticket::{
    _api::logout::infra::{LogoutInfra, LogoutService},
    _common::kernel::infra::{AuthNonceMetadata, AuthTokenMetadata},
};

use super::event::LogoutEvent;

pub async fn logout<S>(
    infra: &impl LogoutInfra,
    post: impl Fn(LogoutEvent) -> S,
) -> MethodResult<S> {
    let nonce_header = infra.nonce_metadata();
    let token_header = infra.token_metadata();
    let logout_service = infra.logout_service();

    let nonce = nonce_header
        .nonce()
        .map_err(|err| post(LogoutEvent::MetadataError(err)))?;

    let token = token_header
        .token()
        .map_err(|err| post(LogoutEvent::MetadataError(err)))?;

    logout_service
        .logout(nonce, token)
        .await
        .map_err(|err| post(LogoutEvent::ServiceError(err)))?;

    Ok(post(LogoutEvent::Success))
}
