use getto_application::data::MethodResult;

use crate::auth::{
    auth_ticket::_api::kernel::infra::{
        AuthHeaderInfra, AuthNonceHeader, AuthTokenHeader, AuthTokenInfra, AuthTokenMessenger,
    },
    password::_api::authenticate::infra::{
        AuthenticatePasswordInfra, AuthenticatePasswordRequestDecoder,
        AuthenticatePasswordResponseEncoder, AuthenticatePasswordService,
    },
};

use super::event::AuthenticatePasswordEvent;

pub async fn authenticate_password<S>(
    infra: &impl AuthenticatePasswordInfra,
    post: impl Fn(AuthenticatePasswordEvent) -> S,
) -> MethodResult<S> {
    let header_infra = infra.header_infra();
    let nonce_header = header_infra.nonce_header();
    let token_header = header_infra.token_header();
    let authenticate_service = infra.authenticate_service();
    let token_infra = infra.token_infra();
    let token_messenger = token_infra.token_messenger();
    let response_encoder = infra.response_encoder();

    let request_decoder = infra.request_decoder();

    let nonce = nonce_header
        .nonce()
        .map_err(|err| post(AuthenticatePasswordEvent::HeaderError(err)))?;

    let token = token_header
        .token()
        .map_err(|err| post(AuthenticatePasswordEvent::HeaderError(err)))?;

    let fields = request_decoder
        .decode()
        .map_err(|err| post(AuthenticatePasswordEvent::MessageError(err)))?;

    let response = authenticate_service
        .authenticate(nonce, token, fields)
        .await
        .map_err(|err| post(AuthenticatePasswordEvent::ServiceError(err)))?;

    let message = response_encoder
        .encode(response)
        .map_err(|err| post(AuthenticatePasswordEvent::MessageError(err)))?;

    let message = message.map(|message| token_messenger.to_message(message));

    Ok(post(AuthenticatePasswordEvent::Result(message)))
}
