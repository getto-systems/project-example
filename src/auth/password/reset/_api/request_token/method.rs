use getto_application::data::MethodResult;

use crate::auth::{
    auth_ticket::_api::kernel::infra::{AuthNonceHeader, AuthTokenHeader},
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
    let nonce_header = infra.nonce_header();
    let token_header = infra.token_header();
    let request_token_service = infra.request_token_service();
    let response_encoder = infra.response_encoder();

    let nonce = nonce_header
        .nonce()
        .map_err(|err| post(RequestResetTokenEvent::HeaderError(err)))?;

    let token = token_header
        .token()
        .map_err(|err| post(RequestResetTokenEvent::HeaderError(err)))?;

    let response = request_token_service
        .request_token(nonce, token, fields)
        .await
        .map_err(|err| post(RequestResetTokenEvent::ServiceError(err)))?;

    let message = response_encoder
        .encode(response)
        .map_err(|err| post(RequestResetTokenEvent::MessageError(err)))?;

    Ok(post(RequestResetTokenEvent::Result(message)))
}
