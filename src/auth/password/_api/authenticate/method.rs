use crate::auth::auth_ticket::_api::kernel::method::check_nonce;

use crate::auth::{
    auth_user::_api::kernel::infra::{AuthUserInfra, AuthUserRepository},
    password::_api::{
        authenticate::infra::{AuthenticatePasswordInfra, AuthenticatePasswordMessenger},
        kernel::infra::{AuthUserPasswordInfra, AuthUserPasswordRepository, PlainPassword},
    },
};

use super::event::AuthenticatePasswordEvent;

use crate::auth::{auth_user::_api::kernel::data::AuthUser, login_id::_api::data::LoginId};

pub fn authenticate_password<S>(
    infra: &impl AuthenticatePasswordInfra,
    post: impl Fn(AuthenticatePasswordEvent) -> S,
) -> Result<AuthUser, S> {
    check_nonce(infra.check_nonce_infra())
        .map_err(|err| post(AuthenticatePasswordEvent::NonceError(err)))?;

    let password_infra = infra.password_infra();
    let password_repository = password_infra.password_repository();
    let messenger = infra.messenger();

    let fields = messenger
        .decode()
        .map_err(|err| post(AuthenticatePasswordEvent::MessageError(err)))?;

    let login_id = LoginId::validate(fields.login_id)
        .map_err(|err| post(AuthenticatePasswordEvent::ValidateLoginIdError(err)))?;

    let plain_password = PlainPassword::validate(fields.password)
        .map_err(|err| post(AuthenticatePasswordEvent::ValidatePasswordError(err)))?;

    let matcher = password_infra.password_matcher(plain_password);

    let user_id = password_repository
        .verify_password(&login_id, &matcher)
        .map_err(|err| post(err.into_authenticate_password_event(messenger)))?;

    let user_repository = infra.user_infra().user_repository();

    let user = user_repository
        .get(&user_id)
        .map_err(|err| post(AuthenticatePasswordEvent::RepositoryError(err)))?;

    let user = user.ok_or_else(|| post(AuthenticatePasswordEvent::UserNotFound))?;

    // 呼び出し側を簡単にするため、例外的に State ではなく AuthUser を返す
    post(AuthenticatePasswordEvent::Success(user.clone()));
    Ok(user)
}
