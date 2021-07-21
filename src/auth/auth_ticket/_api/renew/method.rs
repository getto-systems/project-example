use getto_application::data::MethodResult;

use crate::auth::auth_ticket::_api::{
    kernel::infra::{
        AuthHeaderInfra, AuthNonceHeader, AuthTokenHeader, AuthTokenInfra, AuthTokenMessenger,
    },
    renew::infra::{RenewAuthTicketInfra, RenewAuthTicketResponseEncoder, RenewAuthTicketService},
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
    let token_infra = infra.token_infra();
    let token_messenger = token_infra.token_messenger();
    let response_encoder = infra.response_encoder();

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

    let message = response_encoder
        .encode(response)
        .map_err(|err| post(RenewAuthTicketEvent::MessageError(err)))?;

    let message = token_messenger.to_message(message);

    Ok(post(RenewAuthTicketEvent::Success(message)))
}
