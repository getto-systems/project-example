use crate::auth::ticket::remote::check_nonce::method::check_auth_nonce;

use crate::auth::user::password::remote::kernel::data::ResetTokenDestination;
use crate::auth::{
    ticket::remote::kernel::infra::AuthClock,
    user::{
        password::{
            remote::kernel::infra::{PlainPassword, ResetPasswordRepository, ResetTokenEntry},
            reset::remote::{
                proxy_reset::infra::ResetPasswordFieldsExtract,
                reset::infra::{ResetPasswordInfra, ResetTokenDecoder, ResetPasswordNotifier},
            },
        },
        remote::kernel::infra::AuthUserRepository,
    },
};

use super::event::ResetPasswordEvent;

use crate::auth::{
    ticket::remote::kernel::data::AuthDateTime,
    user::{
        login_id::remote::data::LoginId,
        password::{
            remote::kernel::data::VerifyResetTokenEntryError,
            reset::remote::kernel::data::ResetTokenEncoded,
        },
        remote::kernel::data::AuthUser,
    },
};

pub async fn reset_password<S>(
    infra: &impl ResetPasswordInfra,
    fields: ResetPasswordFieldsExtract,
    post: impl Fn(ResetPasswordEvent) -> S,
) -> Result<AuthUser, S> {
    check_auth_nonce(infra.check_nonce_infra())
        .await
        .map_err(|err| post(ResetPasswordEvent::NonceError(err)))?;

    let login_id = LoginId::validate(fields.login_id).map_err(|err| post(err.into()))?;
    let plain_password =
        PlainPassword::validate(fields.password).map_err(|err| post(err.into()))?;
    let reset_token =
        ResetTokenEncoded::validate(fields.reset_token).map_err(|err| post(err.into()))?;

    let token_decoder = infra.token_decoder();
    let reset_notifier = infra.reset_notifier();

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

    let destination =
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

    let notify_response = reset_notifier
        .notify(destination)
        .await
        .map_err(|err| post(ResetPasswordEvent::NotifyError(err)))?;

    post(ResetPasswordEvent::ResetNotified(notify_response));

    // 呼び出し側を簡単にするため、例外的に State ではなく AuthUser を返す
    post(ResetPasswordEvent::Success(user.clone()));
    Ok(user)
}

fn verify_reset_token_entry(
    entry: Option<ResetTokenEntry>,
    reset_at: &AuthDateTime,
    login_id: &LoginId,
) -> Result<ResetTokenDestination, VerifyResetTokenEntryError> {
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
    Ok(entry.into_destination())
}
