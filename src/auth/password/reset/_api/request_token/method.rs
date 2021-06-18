use getto_application::data::MethodResult;

use super::infra::{
    RequestResetTokenInfra, RequestResetTokenMessenger, ResetTokenDestinationRepository,
    ResetTokenEncoder, ResetTokenNotifier,
};
use crate::auth::{
    auth_ticket::_api::kernel::{infra::AuthClock, method::check_nonce},
    password::reset::_api::kernel::infra::ResetTokenRepository,
};

use super::event::RequestResetTokenEvent;

use super::data::RequestResetTokenResponse;
use crate::auth::login_id::_api::data::LoginId;

pub fn request_reset_token<S>(
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
        .map_err(|err| post(RequestResetTokenEvent::RepositoryError(err)))?;

    match destination {
        None => {
            let message = messenger
                .encode_invalid_reset()
                .map_err(|err| post(RequestResetTokenEvent::MessageError(err)))?;

            Err(post(RequestResetTokenEvent::InvalidReset(
                RequestResetTokenResponse { message },
            )))
        }
        Some(destination) => {
            let config = infra.config();
            let clock = infra.clock();
            let token_repository = infra.token_repository();
            let token_generator = infra.token_generator();
            let token_encoder = infra.token_encoder();
            let token_notifier = infra.token_notifier();

            let expires = clock.now().expires(&config.token_expires);
            post(RequestResetTokenEvent::TokenExpiresCalculated(
                expires.clone(),
            ));

            let token = token_repository
                .register(
                    destination.clone(),
                    token_generator,
                    expires.clone(),
                    clock.now(),
                )
                .map_err(|err| post(RequestResetTokenEvent::RepositoryError(err)))?;

            let token = token_encoder
                .encode(token, expires)
                .map_err(|err| post(RequestResetTokenEvent::EncodeError(err)))?;

            token_notifier
                .notify(destination, token)
                .map_err(|err| post(RequestResetTokenEvent::NotifyError(err)))?;

            let message = messenger
                .encode_success()
                .map_err(|err| post(RequestResetTokenEvent::MessageError(err)))?;

            Ok(post(RequestResetTokenEvent::Success(
                RequestResetTokenResponse { message },
            )))
        }
    }
}
