use getto_application::data::MethodResult;

use crate::auth::{
    auth_ticket::_common::kernel::infra::AuthServiceMetadata,
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
    let service_metadata = infra.service_metadata();
    let change_service = infra.change_service();
    let response_encoder = infra.response_encoder();

    let metadata = service_metadata
        .metadata()
        .map_err(|err| post(ChangePasswordEvent::MetadataError(err)))?;

    let response = change_service
        .change(metadata, fields)
        .await
        .map_err(|err| post(ChangePasswordEvent::ServiceError(err)))?;

    let message = response_encoder
        .encode(response)
        .map_err(|err| post(ChangePasswordEvent::MessageError(err)))?;

    Ok(post(ChangePasswordEvent::Result(message)))
}
