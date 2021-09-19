use getto_application::data::MethodResult;

use crate::auth::{
    auth_ticket::{
        _api::kernel::infra::AuthTokenResponseBuilder,
        _common::kernel::infra::{AuthNonceMetadata, AuthTokenMetadata},
    },
    password::reset::{
        _api::reset::infra::{
            ResetPasswordInfra, ResetPasswordResponseEncoder, ResetPasswordService,
        },
        _common::reset::infra::ResetPasswordFieldsExtract,
    },
};

use super::event::ResetPasswordEvent;

pub async fn reset_password<S>(
    infra: &impl ResetPasswordInfra,
    fields: ResetPasswordFieldsExtract,
    post: impl Fn(ResetPasswordEvent) -> S,
) -> MethodResult<S> {
    let nonce_metadata = infra.nonce_metadata();
    let token_metadata = infra.token_metadata();
    let reset_service = infra.reset_service();
    let response_encoder = infra.response_encoder();
    let response_builder = infra.response_builder();

    let nonce = nonce_metadata
        .nonce()
        .map_err(|err| post(ResetPasswordEvent::MetadataError(err)))?;

    let token = token_metadata
        .token()
        .map_err(|err| post(ResetPasswordEvent::MetadataError(err)))?;

    let response = reset_service
        .reset(nonce, token, fields)
        .await
        .map_err(|err| post(ResetPasswordEvent::ServiceError(err)))?;

    let message = response_encoder
        .encode(response)
        .map_err(|err| post(ResetPasswordEvent::MessageError(err)))?;

    let message = message.map(|message| response_builder.build(message));

    Ok(post(ResetPasswordEvent::Result(message)))
}
