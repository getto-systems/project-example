use crate::auth::auth_ticket::_auth::kernel::method::check_nonce;

use crate::auth::password::{
    _auth::{
        change::infra::ChangePasswordInfra,
        kernel::infra::{ChangePasswordRepository, PlainPassword},
    },
    _common::change::infra::ChangePasswordFieldsExtract,
};

use super::event::{ChangePasswordEvent, ChangePasswordKind};

use crate::auth::auth_user::_common::kernel::data::AuthUserId;
use getto_application::data::MethodResult;

pub async fn change_password<S>(
    infra: &impl ChangePasswordInfra,
    user_id: AuthUserId,
    fields: ChangePasswordFieldsExtract,
    post: impl Fn(ChangePasswordEvent) -> S,
) -> MethodResult<S> {
    check_nonce(infra.check_nonce_infra())
        .await
        .map_err(|err| post(ChangePasswordEvent::NonceError(err)))?;

    let current_password = PlainPassword::validate(fields.current_password)
        .map_err(|err| post((err, ChangePasswordKind::Current).into()))?;
    let new_password = PlainPassword::validate(fields.new_password)
        .map_err(|err| post((err, ChangePasswordKind::New).into()))?;

    let password_repository = infra.password_repository();
    let password_matcher = infra.password_matcher(current_password);
    let password_hasher = infra.password_hasher(new_password);

    password_repository
        .change_password(&user_id, password_matcher, password_hasher)
        .await
        .map_err(|err| post(err.into()))?;

    Ok(post(ChangePasswordEvent::Success))
}
