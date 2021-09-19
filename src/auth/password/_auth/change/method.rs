use crate::auth::{
    _common::data::RequireAuthRoles,
    auth_ticket::_auth::validate::method::validate_api_token,
    password::{
        _auth::{
            change::infra::ChangePasswordInfra,
            kernel::infra::{ChangePasswordRepository, PlainPassword},
        },
        _common::change::infra::ChangePasswordFieldsExtract,
    },
};

use super::event::{ChangePasswordEvent, ChangePasswordKind};

use getto_application::data::MethodResult;

pub async fn change_password<S>(
    infra: &impl ChangePasswordInfra,
    fields: ChangePasswordFieldsExtract,
    post: impl Fn(ChangePasswordEvent) -> S,
) -> MethodResult<S> {
    let ticket = validate_api_token(infra.validate_infra(), RequireAuthRoles::Nothing, |event| {
        post(ChangePasswordEvent::Validate(event))
    })
    .await?;

    let current_password = PlainPassword::validate(fields.current_password)
        .map_err(|err| post((err, ChangePasswordKind::Current).into()))?;
    let new_password = PlainPassword::validate(fields.new_password)
        .map_err(|err| post((err, ChangePasswordKind::New).into()))?;

    let password_repository = infra.password_repository();
    let password_matcher = infra.password_matcher(current_password);
    let password_hasher = infra.password_hasher(new_password);

    let user_id = ticket.into_user().into_user_id();

    password_repository
        .change_password(&user_id, password_matcher, password_hasher)
        .await
        .map_err(|err| post(err.into()))?;

    Ok(post(ChangePasswordEvent::Success))
}
