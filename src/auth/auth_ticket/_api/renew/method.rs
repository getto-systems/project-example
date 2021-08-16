use getto_application::data::MethodResult;

use crate::auth::auth_ticket::_api::{
    kernel::infra::{AuthNonceHeader, AuthTokenHeader, AuthTokenResponseBuilder},
    renew::infra::{RenewAuthTicketInfra, RenewAuthTicketResponseEncoder, RenewAuthTicketService},
};

use super::event::RenewAuthTicketEvent;

pub async fn renew<S>(
    infra: &impl RenewAuthTicketInfra,
    post: impl Fn(RenewAuthTicketEvent) -> S,
) -> MethodResult<S> {
    let nonce_header = infra.nonce_header();
    let token_header = infra.token_header();
    let renew_service = infra.renew_service();
    let response_encoder = infra.response_encoder();
    let response_builder = infra.response_builder();

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

    let message = response_builder.build(message);

    Ok(post(RenewAuthTicketEvent::Success(message)))
}
