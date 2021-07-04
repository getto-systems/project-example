use getto_application::data::MethodResult;

use crate::auth::auth_ticket::_api::kernel::method::check_nonce;

use crate::auth::{
    auth_ticket::_api::kernel::infra::{AuthClock, CheckAuthNonceInfra},
    password::{
        _api::kernel::infra::{AuthUserPasswordInfra, AuthUserPasswordRepository},
        reset::_api::request_token::infra::{
            RequestResetTokenInfra, RequestResetTokenMessenger, ResetTokenDestinationRepository,
            ResetTokenEncoder, ResetTokenGenerator, ResetTokenNotifier,
        },
    },
};

use super::event::RequestResetTokenEvent;

use crate::auth::login_id::_api::data::LoginId;

pub async fn request_reset_token<S>(
    infra: &impl RequestResetTokenInfra,
    post: impl Fn(RequestResetTokenEvent) -> S,
) -> MethodResult<S> {
    check_nonce(infra.check_nonce_infra())
        .await
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

    let clock = infra.check_nonce_infra().clock();
    let password_repository = infra.password_infra().password_repository();
    let token_generator = infra.token_generator();
    let token_encoder = infra.token_encoder();
    let token_notifier = infra.token_notifier();
    let config = infra.config();

    let reset_token = token_generator.generate();

    let requested_at = clock.now();
    let expires = requested_at.clone().expires(&config.token_expires);

    post(RequestResetTokenEvent::TokenExpiresCalculated(
        expires.clone(),
    ));

    password_repository
        .request_reset_token(reset_token.clone(), login_id, expires.clone(), requested_at)
        .map_err(|err| post(err.into_request_reset_token_event(messenger)))?;

    let token_encoded = token_encoder
        .encode(reset_token, expires)
        .map_err(|err| post(RequestResetTokenEvent::EncodeError(err)))?;

    let notify_response = token_notifier
        .notify(destination, token_encoded)
        .await
        .map_err(|err| post(RequestResetTokenEvent::NotifyError(err)))?;

    post(RequestResetTokenEvent::TokenNotified(notify_response));

    let response = messenger
        .encode_success()
        .map_err(|err| post(RequestResetTokenEvent::MessageError(err)))?;

    Ok(post(RequestResetTokenEvent::Success(response)))
}
