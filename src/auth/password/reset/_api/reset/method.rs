use getto_application::data::MethodResult;

use crate::auth::{
    auth_ticket::_api::kernel::infra::{
        AuthHeaderInfra, AuthNonceHeader, AuthTokenHeader, AuthTokenInfra, AuthTokenResponseBuilder,
    },
    password::reset::_api::reset::infra::{
        ResetPasswordInfra, ResetPasswordRequestDecoder,
        ResetPasswordResponseEncoder, ResetPasswordService,
    },
};

use super::event::ResetPasswordEvent;

pub async fn reset_password<S>(
    infra: &impl ResetPasswordInfra,
    post: impl Fn(ResetPasswordEvent) -> S,
) -> MethodResult<S> {
    let header_infra = infra.header_infra();
    let nonce_header = header_infra.nonce_header();
    let token_header = header_infra.token_header();
    let reset_service = infra.reset_service();
    let token_infra = infra.token_infra();
    let token_messenger = token_infra.response_builder();
    let response_encoder = infra.response_encoder();

    let request_decoder = infra.request_decoder();

    let nonce = nonce_header
        .nonce()
        .map_err(|err| post(ResetPasswordEvent::HeaderError(err)))?;

    let token = token_header
        .token()
        .map_err(|err| post(ResetPasswordEvent::HeaderError(err)))?;

    let fields = request_decoder
        .decode()
        .map_err(|err| post(ResetPasswordEvent::MessageError(err)))?;

    let response = reset_service
        .reset(nonce, token, fields)
        .await
        .map_err(|err| post(ResetPasswordEvent::ServiceError(err)))?;

    let message = response_encoder
        .encode(response)
        .map_err(|err| post(ResetPasswordEvent::MessageError(err)))?;

    let message = message.map(|message| token_messenger.build(message));

    Ok(post(ResetPasswordEvent::Result(message)))
}
