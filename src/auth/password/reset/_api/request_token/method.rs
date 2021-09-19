use getto_application::data::MethodResult;

use crate::auth::{
    auth_ticket::_common::kernel::infra::AuthServiceMetadata,
    password::reset::{
        _api::request_token::infra::{
            RequestResetTokenInfra, RequestResetTokenResponseEncoder, RequestResetTokenService,
        },
        _common::request_token::infra::RequestResetTokenFieldsExtract,
    },
};

use super::event::RequestResetTokenEvent;

pub async fn request_reset_token<S>(
    infra: &impl RequestResetTokenInfra,
    fields: RequestResetTokenFieldsExtract,
    post: impl Fn(RequestResetTokenEvent) -> S,
) -> MethodResult<S> {
    let service_metadata = infra.service_metadata();
    let request_token_service = infra.request_token_service();
    let response_encoder = infra.response_encoder();

    let metadata = service_metadata
        .metadata()
        .map_err(|err| post(RequestResetTokenEvent::MetadataError(err)))?;

    let response = request_token_service
        .request_token(metadata, fields)
        .await
        .map_err(|err| post(RequestResetTokenEvent::ServiceError(err)))?;

    let message = response_encoder
        .encode(response)
        .map_err(|err| post(RequestResetTokenEvent::MessageError(err)))?;

    Ok(post(RequestResetTokenEvent::Result(message)))
}
