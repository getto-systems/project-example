use getto_application::data::MethodResult;

use crate::auth::{
    auth_ticket::{
        _api::kernel::infra::AuthTokenResponseBuilder, _common::kernel::infra::AuthMetadata,
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
    let auth_metadata = infra.auth_metadata();
    let authenticate_service = infra.authenticate_service();
    let response_encoder = infra.response_encoder();
    let response_builder = infra.response_builder();

    let metadata = auth_metadata
        .metadata()
        .map_err(|err| post(AuthenticatePasswordEvent::MetadataError(err)))?;

    let response = authenticate_service
        .authenticate(metadata, fields)
        .await
        .map_err(|err| post(AuthenticatePasswordEvent::ServiceError(err)))?;

    let message = response_encoder
        .encode(response)
        .map_err(|err| post(AuthenticatePasswordEvent::MessageError(err)))?;

    let message = message.map(|message| response_builder.build(message));

    Ok(post(AuthenticatePasswordEvent::Result(message)))
}
