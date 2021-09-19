use getto_application::data::MethodResult;

use crate::auth::{
    auth_ticket::_common::kernel::infra::{AuthNonceMetadata, AuthTokenMetadata},
    password::{
        _api::change::infra::{
            ChangePasswordInfra, ChangePasswordResponseEncoder, ChangePasswordService,
        },
        _common::change::infra::ChangePasswordFieldsExtract,
    },
};

use super::event::ChangePasswordEvent;

pub async fn change_password<S>(
    infra: &impl ChangePasswordInfra,
    fields: ChangePasswordFieldsExtract,
    post: impl Fn(ChangePasswordEvent) -> S,
) -> MethodResult<S> {
    let nonce_metadata = infra.nonce_metadata();
    let token_metadata = infra.token_metadata();
    let change_service = infra.change_service();
    let response_encoder = infra.response_encoder();

    let nonce = nonce_metadata
        .nonce()
        .map_err(|err| post(ChangePasswordEvent::MetadataError(err)))?;

    let token = token_metadata
        .token()
        .map_err(|err| post(ChangePasswordEvent::MetadataError(err)))?;

    let response = change_service
        .change(nonce, token, fields)
        .await
        .map_err(|err| post(ChangePasswordEvent::ServiceError(err)))?;

    let message = response_encoder
        .encode(response)
        .map_err(|err| post(ChangePasswordEvent::MessageError(err)))?;

    Ok(post(ChangePasswordEvent::Result(message)))
}
