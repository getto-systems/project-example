use getto_application::data::MethodResult;

use crate::auth::auth_ticket::_api::{
    kernel::infra::{AuthHeaderInfra, AuthNonceHeader, AuthTokenHeader},
    renew::infra::{RenewAuthTicketInfra, RenewAuthTicketService, RenewAuthTicketMessenger},
};

use super::event::RenewAuthTicketEvent;

pub async fn renew<S>(
    infra: &impl RenewAuthTicketInfra,
    post: impl Fn(RenewAuthTicketEvent) -> S,
) -> MethodResult<S> {
    let header_infra = infra.header_infra();
    let nonce_header = header_infra.nonce_header();
    let token_header = header_infra.token_header();
    let renew_service = infra.renew_service();
    let messenger = infra.messenger();

    let nonce = nonce_header
        .nonce()
        .map_err(|err| post(RenewAuthTicketEvent::HeaderError(err)))?;

    let token = token_header
        .token()
        .map_err(|err| post(RenewAuthTicketEvent::HeaderError(err)))?;

    let response = renew_service
        .renew(nonce, token)
        .await
        .map_err(|err| post(RenewAuthTicketEvent::ServiceError(err)))?;

    let message = messenger
        .encode(response)
        .map_err(|err| post(RenewAuthTicketEvent::MessageError(err)))?;

    Ok(post(RenewAuthTicketEvent::Success(message)))
}
