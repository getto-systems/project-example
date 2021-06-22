use getto_application::data::MethodResult;

use crate::z_details::_api::repository::helper::register_attempt;

use super::infra::{
    RequestResetTokenInfra, RequestResetTokenMessenger, ResetTokenDestinationRepository,
    ResetTokenEncoder, ResetTokenNotifier,
};
use crate::auth::{
    auth_ticket::_api::kernel::{
        data::{AuthDateTime, ExpireDateTime},
        infra::AuthClock,
        method::check_nonce,
    },
    password::_api::kernel::{
        data::ResetToken,
        infra::{AuthUserPasswordRepository, RegisterResetTokenError, ResetTokenGenerator},
    },
};

use super::event::RequestResetTokenEvent;

use crate::auth::login_id::_api::data::LoginId;

pub async fn request_reset_token<S>(
    infra: &impl RequestResetTokenInfra,
    post: impl Fn(RequestResetTokenEvent) -> S,
) -> MethodResult<S> {
    check_nonce(infra.check_nonce_infra())
        .map_err(|err| post(RequestResetTokenEvent::NonceError(err)))?;

    let destination_repository = infra.destination_repository();
    let messenger = infra.messenger();

    let fields = messenger
        .decode()
        .map_err(|err| post(RequestResetTokenEvent::MessageError(err)))?;

    let login_id = LoginId::validate(fields.login_id)
        .map_err(|err| post(RequestResetTokenEvent::ValidateLoginIdError(err)))?;

    let destination = destination_repository
        .get(&login_id)
        .map_err(|err| post(RequestResetTokenEvent::RepositoryError(err)))?
        .ok_or_else(|| post(messenger.encode_destination_not_found().into()))?;

    let config = infra.config();
    let clock = infra.clock();
    let token_encoder = infra.token_encoder();
    let token_notifier = infra.token_notifier();

    let expires = clock.now().expires(&config.token_expires);
    post(RequestResetTokenEvent::TokenExpiresCalculated(
        expires.clone(),
    ));

    let registered_at = clock.now();

    let token = register_reset_token(infra, &login_id, &expires, &registered_at)
        .map_err(|err| post(err.into_request_reset_token_event(messenger)))?;

    let token = token_encoder
        .encode(token, expires)
        .map_err(|err| post(RequestResetTokenEvent::EncodeError(err)))?;

    let response = token_notifier
        .notify(destination, &token)
        .await
        .map_err(|err| post(RequestResetTokenEvent::NotifyError(err)))?;

    post(RequestResetTokenEvent::TokenNotified(response));

    let response = messenger
        .encode_success()
        .map_err(|err| post(RequestResetTokenEvent::MessageError(err)))?;

    Ok(post(RequestResetTokenEvent::Success(response)))
}

fn register_reset_token(
    infra: &impl RequestResetTokenInfra,
    login_id: &LoginId,
    expires: &ExpireDateTime,
    registered_at: &AuthDateTime,
) -> Result<ResetToken, RegisterResetTokenError> {
    let password_repository = infra.password_repository();
    let token_generator = infra.token_generator();

    register_attempt(
        || {
            let token = token_generator.generate();
            password_repository.register_reset_token(token, login_id, expires, registered_at)
        },
        RegisterResetTokenError::RepositoryError,
    )
}
