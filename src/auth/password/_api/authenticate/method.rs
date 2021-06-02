use std::convert::TryInto;

use crate::auth::auth_ticket::_api::kernel::part::check_nonce;

use super::infra::{
    AuthUserPasswordHash, AuthUserPasswordRepository, AuthenticateMessenger,
    AuthenticatePasswordInfra, MatchPasswordError,
};
use crate::auth::auth_user::_api::kernel::infra::AuthUserRepository;

use super::event::AuthenticatePasswordEvent;

use super::data::AuthenticatePasswordError;
use crate::auth::auth_user::_api::kernel::data::AuthUser;

pub fn authenticate_password<S>(
    infra: &impl AuthenticatePasswordInfra,
    post: impl Fn(AuthenticatePasswordEvent) -> S,
) -> Result<AuthUser, S> {
    check_nonce(
        infra.nonce_config(),
        infra.clock(),
        infra.nonce_header(),
        infra.nonce_repository(),
    )
    .map_err(|err| post(AuthenticatePasswordEvent::NonceError(err)))?;

    let password_hash = infra.password_hash();
    let password_repository = infra.password_repository();
    let messenger = infra.messenger();

    let fields = messenger
        .decode()
        .map_err(|err| post(AuthenticatePasswordEvent::MessageError(err)))?;

    let login_id = fields
        .login_id
        .try_into()
        .map_err(|err| post(AuthenticatePasswordEvent::ConvertLoginIdError(err)))?;

    let plain_password = fields
        .password
        .try_into()
        .map_err(|err| post(AuthenticatePasswordEvent::ConvertPasswordError(err)))?;

    let user_id = password_repository
        .match_password(&login_id, |hashed_password| {
            password_hash
                .verify(&plain_password, hashed_password)
                .map_err(|err| err.into())
        })
        .map_err(|err| match err {
            MatchPasswordError::PasswordHashError(err) => {
                post(AuthenticatePasswordEvent::PasswordHashError(err))
            }
            MatchPasswordError::RepositoryError(err) => {
                post(AuthenticatePasswordEvent::RepositoryError(err))
            }
        })?;

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
