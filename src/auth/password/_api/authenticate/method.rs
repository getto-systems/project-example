use getto_application::data::MethodResult;

use crate::auth::{
    auth_ticket::{
        _api::kernel::infra::AuthTokenResponseBuilder,
        _common::kernel::infra::{AuthNonceMetadata, AuthTokenMetadata},
    },
    password::{
        _api::authenticate::infra::{
            AuthenticatePasswordInfra, AuthenticatePasswordResponseEncoder,
            AuthenticatePasswordService,
        },
        _common::authenticate::infra::AuthenticatePasswordFieldsExtract,
    },
};

use super::event::AuthenticatePasswordEvent;

pub async fn authenticate_password<S>(
    infra: &impl AuthenticatePasswordInfra,
    fields: AuthenticatePasswordFieldsExtract,
    post: impl Fn(AuthenticatePasswordEvent) -> S,
) -> MethodResult<S> {
    let nonce_metadata = infra.nonce_metadata();
    let token_metadata = infra.token_metadata();
    let authenticate_service = infra.authenticate_service();
    let response_encoder = infra.response_encoder();
    let response_builder = infra.response_builder();

    let nonce = nonce_metadata
        .nonce()
        .map_err(|err| post(AuthenticatePasswordEvent::MetadataError(err)))?;

    let token = token_metadata
        .token()
        .map_err(|err| post(AuthenticatePasswordEvent::MetadataError(err)))?;

    let response = authenticate_service
        .authenticate(nonce, token, fields)
        .await
        .map_err(|err| post(AuthenticatePasswordEvent::ServiceError(err)))?;

    let message = response_encoder
        .encode(response)
        .map_err(|err| post(AuthenticatePasswordEvent::MessageError(err)))?;

    let message = message.map(|message| response_builder.build(message));

    Ok(post(AuthenticatePasswordEvent::Result(message)))
}
