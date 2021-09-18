use getto_application::data::MethodResult;

use crate::auth::_common::{data::RequireAuthRoles, method::validate_api_token};

use super::{event::NotifyUnexpectedErrorEvent, infra::NotifyUnexpectedErrorInfra};

pub async fn notify_unexpected_error<S>(
    infra: &impl NotifyUnexpectedErrorInfra,
    err: String,
    post: impl Fn(NotifyUnexpectedErrorEvent) -> S,
) -> MethodResult<S> {
    let user_id = validate_api_token(infra.validate_infra(), RequireAuthRoles::Nothing)
        .await
        .map_err(|err| post(NotifyUnexpectedErrorEvent::ValidateApiTokenError(err)))?;
    post(NotifyUnexpectedErrorEvent::Authorized(user_id));
    Ok(post(NotifyUnexpectedErrorEvent::Notice(err)))
}
