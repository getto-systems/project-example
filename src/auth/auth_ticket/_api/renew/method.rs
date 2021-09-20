use getto_application::data::MethodResult;

use crate::auth::auth_ticket::{
    _api::{
        kernel::infra::AuthTokenResponseBuilder,
        renew::infra::{
            RenewAuthTicketInfra, RenewAuthTicketResponseEncoder, RenewAuthTicketService,
        },
    },
    _common::kernel::infra::AuthMetadata,
};

use super::event::RenewAuthTicketEvent;

pub async fn renew<S>(
    infra: &impl RenewAuthTicketInfra,
    post: impl Fn(RenewAuthTicketEvent) -> S,
) -> MethodResult<S> {
    let auth_metadata = infra.auth_metadata();
    let renew_service = infra.renew_service();
    let response_encoder = infra.response_encoder();
    let response_builder = infra.response_builder();

    let metadata = auth_metadata
        .metadata()
        .map_err(|err| post(RenewAuthTicketEvent::MetadataError(err)))?;

    let response = renew_service
        .renew(metadata)
        .await
        .map_err(|err| post(RenewAuthTicketEvent::ServiceError(err)))?;

    let message = response_encoder
        .encode(response)
        .map_err(|err| post(RenewAuthTicketEvent::MessageError(err)))?;

    let message = response_builder.build(message);

    Ok(post(RenewAuthTicketEvent::Success(message)))
}
