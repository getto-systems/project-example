use getto_application::data::MethodResult;

use crate::auth::auth_ticket::_api::{
    kernel::infra::{AuthHeaderInfra, AuthNonceHeader, AuthTokenHeader},
    logout::infra::{LogoutInfra, LogoutService},
};

use super::event::LogoutEvent;

pub async fn logout<S>(
    infra: &impl LogoutInfra,
    post: impl Fn(LogoutEvent) -> S,
) -> MethodResult<S> {
    let header_infra = infra.header_infra();
    let nonce_header = header_infra.nonce_header();
    let token_header = header_infra.token_header();
    let logout_service = infra.logout_service();

    let nonce = nonce_header
        .nonce()
        .map_err(|err| post(LogoutEvent::HeaderError(err)))?;

    let token = token_header
        .token()
        .map_err(|err| post(LogoutEvent::HeaderError(err)))?;

    logout_service
        .logout(nonce, token)
        .await
        .map_err(|err| post(LogoutEvent::ServiceError(err)))?;

    Ok(post(LogoutEvent::Success))
}
