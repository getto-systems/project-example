use crate::auth::auth_ticket::_api::kernel::method::check_nonce;

use crate::auth::{
    auth_ticket::_api::kernel::infra::{AuthClock, CheckAuthNonceInfra},
    auth_user::_api::kernel::infra::AuthUserRepository,
    password::{
        _api::kernel::infra::{AuthUserPasswordRepository, PlainPassword},
        reset::_api::reset::infra::{
            ResetPasswordInfra, ResetPasswordMessenger, ResetTokenDecoder,
        },
    },
};

use super::event::ResetPasswordEvent;

use crate::auth::{
    auth_user::_api::kernel::data::AuthUser, login_id::_api::data::LoginId,
    password::reset::_api::kernel::data::ResetTokenEncoded,
};

pub fn reset_password<S>(
    infra: &impl ResetPasswordInfra,
    post: impl Fn(ResetPasswordEvent) -> S,
) -> Result<AuthUser, S> {
    check_nonce(infra.check_nonce_infra())
        .map_err(|err| post(ResetPasswordEvent::NonceError(err)))?;

    let clock = infra.check_nonce_infra().clock();
    let password_repository = infra.password_repository();
    let token_decoder = infra.token_decoder();
    let messenger = infra.messenger();

    let fields = messenger
        .decode()
        .map_err(|err| post(ResetPasswordEvent::MessageError(err)))?;

    let login_id = LoginId::validate(fields.login_id)
        .map_err(|err| post(ResetPasswordEvent::ValidateLoginIdError(err)))?;

    let plain_password = PlainPassword::validate(fields.password)
        .map_err(|err| post(ResetPasswordEvent::ValidatePasswordError(err)))?;

    let reset_token = ResetTokenEncoded::validate(fields.reset_token)
        .map_err(|err| post(ResetPasswordEvent::ValidateResetTokenError(err)))?;

    let token = token_decoder
        .decode(&reset_token)
        .map_err(|err| post(ResetPasswordEvent::DecodeError(err)))?;

    let hasher = infra.password_hasher(plain_password);
    let reset_at = clock.now();

    let user_id = password_repository
        .reset_password(&token, &login_id, &hasher, &reset_at)
        .map_err(|err| post(err.into_reset_password_event(messenger)))?;

    let user_repository = infra.user_repository();

    let user = user_repository
        .get(&user_id)
        .map_err(|err| post(ResetPasswordEvent::RepositoryError(err)))?;

    let user = user.ok_or_else(|| post(ResetPasswordEvent::UserNotFound))?;

    // 呼び出し側を簡単にするため、例外的に State ではなく AuthUser を返す
    post(ResetPasswordEvent::Success(user.clone()));
    Ok(user)
}
