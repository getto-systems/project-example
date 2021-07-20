use crate::auth::auth_ticket::_auth::kernel::method::check_nonce;

use crate::auth::{
    auth_user::_auth::kernel::infra::{AuthUserInfra, AuthUserRepository},
    password::_auth::{
        authenticate::infra::{AuthenticatePasswordInfra, AuthenticatePasswordRequestDecoder},
        kernel::infra::{AuthUserPasswordInfra, AuthUserPasswordRepository, PlainPassword},
    },
};

use super::event::AuthenticatePasswordEvent;

use crate::auth::{auth_user::_common::kernel::data::AuthUser, login_id::_auth::data::LoginId};

pub async fn authenticate_password<S>(
    infra: impl AuthenticatePasswordInfra,
    post: impl Fn(AuthenticatePasswordEvent) -> S,
) -> Result<AuthUser, S> {
    let (check_nonce_infra, user_infra, password_infra, request_decoder) = infra.extract();

    check_nonce(&check_nonce_infra)
        .await
        .map_err(|err| post(AuthenticatePasswordEvent::NonceError(err)))?;

    let password_repository = password_infra.password_repository();
    let user_repository = user_infra.user_repository();

    let fields = request_decoder.decode();
    let login_id = LoginId::validate(fields.login_id).map_err(|err| post(err.into()))?;
    let plain_password =
        PlainPassword::validate(fields.password).map_err(|err| post(err.into()))?;

    let matcher = password_infra.password_matcher(plain_password);

    let user_id = password_repository
        .verify_password(&login_id, matcher)
        .await
        .map_err(|err| post(err.into()))?;

    let user = user_repository
        .get(&user_id)
        .await
        .map_err(|err| post(AuthenticatePasswordEvent::RepositoryError(err)))?;

    let user = user.ok_or_else(|| post(AuthenticatePasswordEvent::UserNotFound))?;

    // 呼び出し側を簡単にするため、例外的に State ではなく AuthUser を返す
    post(AuthenticatePasswordEvent::Success(user.clone()));
    Ok(user)
}
