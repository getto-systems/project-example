use crate::auth::auth_ticket::_api::kernel::method::check_nonce;

use super::infra::PlainPassword;
use super::infra::{
    AuthUserPasswordHash, AuthUserPasswordRepository, AuthenticateMessenger,
    AuthenticatePasswordInfra,
};
use crate::auth::auth_user::_api::kernel::infra::AuthUserRepository;

use super::event::AuthenticatePasswordEvent;

use super::data::AuthenticatePasswordError;
use crate::auth::{auth_user::_api::kernel::data::AuthUser, login_id::_api::data::LoginId};

pub fn authenticate_password<S>(
    infra: &impl AuthenticatePasswordInfra,
    post: impl Fn(AuthenticatePasswordEvent) -> S,
) -> Result<AuthUser, S> {
    check_nonce(infra.check_nonce_infra())
        .map_err(|err| post(AuthenticatePasswordEvent::NonceError(err)))?;

    let password_hash = infra.password_hash();
    let password_repository = infra.password_repository();
    let messenger = infra.messenger();

    let fields = messenger
        .decode()
        .map_err(|err| post(AuthenticatePasswordEvent::MessageError(err)))?;

    let login_id = LoginId::validate(fields.login_id)
        .map_err(|err| post(AuthenticatePasswordEvent::ConvertLoginIdError(err)))?;

    let plain_password = PlainPassword::validate(fields.password)
        .map_err(|err| post(AuthenticatePasswordEvent::ConvertPasswordError(err)))?;

    let user_id = password_repository
        .match_password(&login_id, |hashed_password| {
            password_hash
                .verify(&plain_password, hashed_password)
                .map_err(|err| err.into())
        })
        .map_err(|err| post(err.into()))?;

    match user_id {
        None => {
            let message = messenger
                .encode_invalid_password()
                .map_err(|err| post(AuthenticatePasswordEvent::MessageError(err)))?;

            Err(post(AuthenticatePasswordEvent::InvalidPassword(
                AuthenticatePasswordError { message },
            )))
        }
        Some(user_id) => {
            let user_repository = infra.user_repository();

            let user = user_repository
                .get(&user_id)
                .map_err(|err| post(AuthenticatePasswordEvent::RepositoryError(err)))?;

            let user = user.ok_or_else(|| post(AuthenticatePasswordEvent::UserNotFound))?;

            // 呼び出し側を簡単にするため、例外的に State ではなく AuthUser を返す
            post(AuthenticatePasswordEvent::Success(user.clone()));
            Ok(user)
        }
    }
}
