use getto_application::data::MethodResult;

use crate::auth::auth_ticket::{
    _api::{
        kernel::infra::AuthTokenResponseBuilder,
        renew::infra::{
            RenewAuthTicketInfra, RenewAuthTicketResponseEncoder, RenewAuthTicketService,
        },
    },
    _common::kernel::infra::{AuthNonceMetadata, AuthTokenMetadata},
};

use super::event::RenewAuthTicketEvent;

pub async fn renew<S>(
    infra: &impl RenewAuthTicketInfra,
    post: impl Fn(RenewAuthTicketEvent) -> S,
) -> MethodResult<S> {
    let nonce_metadata = infra.nonce_metadata();
    let token_metadata = infra.token_metadata();
    let renew_service = infra.renew_service();
    let response_encoder = infra.response_encoder();
    let response_builder = infra.response_builder();

    let nonce = nonce_metadata
        .nonce()
        .map_err(|err| post(RenewAuthTicketEvent::MetadataError(err)))?;

    let token = token_metadata
        .token()
        .map_err(|err| post(RenewAuthTicketEvent::MetadataError(err)))?;

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
