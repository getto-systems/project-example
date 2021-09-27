use getto_application::data::MethodResult;

use crate::auth::remote::{data::RequireAuthRoles, method::validate_api_token};

use crate::avail::unexpected_error::{
    _common::notify::infra::NotifyUnexpectedErrorFieldsExtract,
    _example::notify::infra::NotifyUnexpectedErrorInfra,
};

use super::event::NotifyUnexpectedErrorEvent;

pub async fn notify_unexpected_error<S>(
    infra: &impl NotifyUnexpectedErrorInfra,
    fields: NotifyUnexpectedErrorFieldsExtract,
    post: impl Fn(NotifyUnexpectedErrorEvent) -> S,
) -> MethodResult<S> {
    validate_api_token(infra.validate_infra(), RequireAuthRoles::Nothing)
        .await
        .map_err(|err| post(NotifyUnexpectedErrorEvent::ValidateError(err)))?;

    // TODO おそらくここで slack に通知とかする
    Ok(post(NotifyUnexpectedErrorEvent::Error(fields.err)))
}
