use getto_application::data::MethodResult;

use crate::auth::auth_ticket::_auth::kernel::method::check_nonce;

use crate::auth::password::reset::_auth::request_token::event::destination_not_found;

use crate::auth::password::{
    _auth::kernel::infra::{AuthUserPasswordInfra, AuthUserPasswordRepository},
    reset::_auth::request_token::infra::{
        RequestResetTokenInfra, RequestResetTokenRequestDecoder, ResetTokenDestinationRepository,
        ResetTokenEncoder, ResetTokenGenerator, ResetTokenNotifier,
    },
};

use super::event::RequestResetTokenEvent;

use crate::auth::login_id::_auth::data::LoginId;

pub async fn request_reset_token<S>(
    infra: impl RequestResetTokenInfra,
    post: impl Fn(RequestResetTokenEvent) -> S,
) -> MethodResult<S> {
    let (
        check_nonce_infra,
        clock_infra,
        password_infra,
        request_decoder,
        destination_repository,
        token_generator,
        token_encoder,
        token_notifier,
        config,
    ) = infra.extract();

    check_nonce(check_nonce_infra)
        .await
        .map_err(|err| post(RequestResetTokenEvent::NonceError(err)))?;

    let fields = request_decoder.decode();
    let login_id = LoginId::validate(fields.login_id).map_err(|err| post(err.into()))?;

    let destination = destination_repository
        .get(&login_id)
        .await
        .map_err(|err| post(RequestResetTokenEvent::RepositoryError(err)))?
        .ok_or_else(|| post(destination_not_found()))?;

    let clock = clock_infra.clock;
    let password_repository = password_infra.extract();

    let reset_token = token_generator.generate();

    let requested_at = clock.now();
    let expires = requested_at.clone().expires(&config.token_expires);

    post(RequestResetTokenEvent::TokenExpiresCalculated(
        expires.clone(),
    ));

    password_repository
        .register_reset_token(login_id, reset_token.clone(), expires.clone(), requested_at)
        .await
        .map_err(|err| post(err.into()))?;

    let token_encoded = token_encoder
        .encode(reset_token, expires)
        .map_err(|err| post(RequestResetTokenEvent::EncodeError(err)))?;

    let notify_response = token_notifier
        .notify(destination, token_encoded)
        .await
        .map_err(|err| post(RequestResetTokenEvent::NotifyError(err)))?;

    post(RequestResetTokenEvent::TokenNotified(notify_response));

    Ok(post(RequestResetTokenEvent::Success))
}
