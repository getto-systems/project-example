use getto_application::data::MethodResult;

use crate::auth::{
    auth_ticket::_api::kernel::infra::{AuthHeaderInfra, AuthNonceHeader, AuthTokenHeader},
    password::reset::_api::request_token::infra::{
        RequestResetTokenInfra, RequestResetTokenRequestDecoder, RequestResetTokenResponseEncoder,
        RequestResetTokenService,
    },
};

use super::event::RequestResetTokenEvent;

pub async fn request_reset_token<S>(
    infra: &impl RequestResetTokenInfra,
    post: impl Fn(RequestResetTokenEvent) -> S,
) -> MethodResult<S> {
    let header_infra = infra.header_infra();
    let nonce_header = header_infra.nonce_header();
    let token_header = header_infra.token_header();
    let request_token_service = infra.request_token_service();
    let response_encoder = infra.response_encoder();

    let request_decoder = infra.request_decoder();

    let nonce = nonce_header
        .nonce()
        .map_err(|err| post(RequestResetTokenEvent::HeaderError(err)))?;

    let token = token_header
        .token()
        .map_err(|err| post(RequestResetTokenEvent::HeaderError(err)))?;

    let fields = request_decoder
        .decode()
        .map_err(|err| post(RequestResetTokenEvent::MessageError(err)))?;

    let response = request_token_service
        .request_token(nonce, token, fields)
        .await
        .map_err(|err| post(RequestResetTokenEvent::ServiceError(err)))?;

    let message = response_encoder
        .encode(response)
        .map_err(|err| post(RequestResetTokenEvent::MessageError(err)))?;

    Ok(post(RequestResetTokenEvent::Result(message)))
}
