use crate::auth::ticket::remote::check_nonce::method::check_auth_nonce;

use crate::auth::user::{
    password::remote::{
        authenticate::infra::{AuthenticatePasswordFieldsExtract, AuthenticatePasswordInfra},
        kernel::infra::{PlainPassword, VerifyPasswordRepository},
    },
    remote::kernel::infra::AuthUserRepository,
};

use super::event::AuthenticatePasswordEvent;

use crate::auth::user::{login_id::remote::data::LoginId, remote::kernel::data::AuthUser};

pub async fn authenticate_password<S>(
    infra: &impl AuthenticatePasswordInfra,
    fields: AuthenticatePasswordFieldsExtract,
    post: impl Fn(AuthenticatePasswordEvent) -> S,
) -> Result<AuthUser, S> {
    check_auth_nonce(infra.check_nonce_infra())
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
