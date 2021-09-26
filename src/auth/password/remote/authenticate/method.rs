use crate::auth::auth_ticket::_auth::kernel::method::check_nonce;

use crate::auth::{
    auth_user::_auth::kernel::infra::AuthUserRepository,
    password::remote::{
        authenticate::infra::AuthenticatePasswordInfra,
        kernel::infra::{PlainPassword, VerifyPasswordRepository},
        proxy_authenticate::infra::AuthenticatePasswordFieldsExtract,
    },
};

use super::event::AuthenticatePasswordEvent;

use crate::auth::{auth_user::_common::kernel::data::AuthUser, login_id::_auth::data::LoginId};

pub async fn authenticate_password<S>(
    infra: &impl AuthenticatePasswordInfra,
    fields: AuthenticatePasswordFieldsExtract,
    post: impl Fn(AuthenticatePasswordEvent) -> S,
) -> Result<AuthUser, S> {
    check_nonce(infra.check_nonce_infra())
        .await
        .map_err(|err| post(AuthenticatePasswordEvent::NonceError(err)))?;

    let login_id = LoginId::validate(fields.login_id).map_err(|err| post(err.into()))?;
    let plain_password =
        PlainPassword::validate(fields.password).map_err(|err| post(err.into()))?;

    let password_repository = infra.password_repository();
    let password_matcher = infra.password_matcher(plain_password);

    let user_id = password_repository
        .verify_password(&login_id, password_matcher)
        .await
        .map_err(|err| post(err.into()))?;

    let user_repository = infra.user_repository();
    let user = user_repository
        .get(&user_id)
        .await
        .map_err(|err| post(AuthenticatePasswordEvent::RepositoryError(err)))?
        .ok_or_else(|| post(AuthenticatePasswordEvent::UserNotFound))?;

    // 呼び出し側を簡単にするため、例外的に State ではなく AuthUser を返す
    post(AuthenticatePasswordEvent::Success(user.clone()));
    Ok(user)
}
