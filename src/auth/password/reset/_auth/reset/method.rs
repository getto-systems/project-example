use crate::auth::auth_ticket::_auth::kernel::method::check_nonce;

use crate::auth::{
    auth_ticket::_auth::kernel::infra::AuthClock,
    auth_user::_auth::kernel::infra::AuthUserRepository,
    password::{
        _auth::kernel::infra::{AuthUserPasswordRepository, PlainPassword, ResetTokenEntry},
        reset::{
            _auth::reset::infra::{ResetPasswordInfra, ResetTokenDecoder},
            _common::reset::infra::ResetPasswordFieldsExtract,
        },
    },
};

use super::event::ResetPasswordEvent;

use crate::auth::{
    auth_ticket::_common::kernel::data::AuthDateTime,
    auth_user::_common::kernel::data::AuthUser,
    login_id::_auth::data::LoginId,
    password::{
        _auth::kernel::data::VerifyResetTokenEntryError,
        reset::_auth::kernel::data::ResetTokenEncoded,
    },
};

pub async fn reset_password<S>(
    infra: &impl ResetPasswordInfra,
    fields: ResetPasswordFieldsExtract,
    post: impl Fn(ResetPasswordEvent) -> S,
) -> Result<AuthUser, S> {
    let check_nonce_infra = infra.check_nonce_infra();
    let token_decoder = infra.token_decoder();

    check_nonce(check_nonce_infra)
        .await
        .map_err(|err| post(ResetPasswordEvent::NonceError(err)))?;

    let login_id = LoginId::validate(fields.login_id).map_err(|err| post(err.into()))?;
    let plain_password =
        PlainPassword::validate(fields.password).map_err(|err| post(err.into()))?;
    let reset_token =
        ResetTokenEncoded::validate(fields.reset_token).map_err(|err| post(err.into()))?;

    let reset_token = token_decoder
        .decode(&reset_token)
        .map_err(|err| post(ResetPasswordEvent::DecodeError(err)))?;

    let password_repository = infra.password_repository();
    let password_hasher = infra.password_hasher(plain_password);
    let clock = infra.clock();

    let reset_at = clock.now();

    let entry = password_repository
        .reset_token_entry(&reset_token)
        .await
        .map_err(|err| post(ResetPasswordEvent::RepositoryError(err)))?;

    verify_reset_token_entry(entry, &reset_at, &login_id).map_err(|err| post(err.into()))?;

    let user_id = password_repository
        .reset_password(&reset_token, password_hasher, reset_at)
        .await
        .map_err(|err| post(err.into()))?;

    let user_repository = infra.user_repository();
    let user = user_repository
        .get(&user_id)
        .await
        .map_err(|err| post(ResetPasswordEvent::RepositoryError(err)))?
        .ok_or_else(|| post(ResetPasswordEvent::UserNotFound))?;

    // 呼び出し側を簡単にするため、例外的に State ではなく AuthUser を返す
    post(ResetPasswordEvent::Success(user.clone()));
    Ok(user)
}

fn verify_reset_token_entry(
    entry: Option<ResetTokenEntry>,
    reset_at: &AuthDateTime,
    login_id: &LoginId,
) -> Result<(), VerifyResetTokenEntryError> {
    let entry = entry.ok_or(VerifyResetTokenEntryError::ResetTokenEntryNotFound)?;
    if entry.has_already_reset() {
        return Err(VerifyResetTokenEntryError::AlreadyReset);
    }
    if entry.has_expired(reset_at) {
        return Err(VerifyResetTokenEntryError::Expired);
    }
    if !entry.verify_login_id(login_id) {
        return Err(VerifyResetTokenEntryError::LoginIdNotMatched);
    }
    Ok(())
}
