mod detail;

use std::sync::Arc;

use crate::auth::{
    kernel::infra::AuthClock,
    user::password::reset::request_token::infra::{
        RequestResetPasswordTokenFieldsExtract, RequestResetPasswordTokenInfra,
        RequestResetPasswordTokenLogger, RequestResetPasswordTokenRepository,
        ResetPasswordIdGenerator, ResetPasswordTokenEncoder, ResetPasswordTokenNotifier,
    },
};

use crate::auth::user::password::reset::request_token::data::{
    NotifyResetTokenResponse, RequestResetPasswordTokenError,
};

pub struct RequestResetPasswordTokenAction<M: RequestResetPasswordTokenInfra> {
    infra: M,
    logger: Arc<dyn RequestResetPasswordTokenLogger>,
}

impl<M: RequestResetPasswordTokenInfra> RequestResetPasswordTokenAction<M> {
    pub async fn request(
        &self,
        fields: impl RequestResetPasswordTokenFieldsExtract,
    ) -> Result<NotifyResetTokenResponse, RequestResetPasswordTokenError> {
        let fields = fields
            .convert()
            .map_err(|err| self.logger.invalid_request(err))?;

        let (user_id, destination) = self
            .infra
            .repository()
            .lookup_user(&fields.login_id)
            .await
            .map_err(|err| self.logger.failed_to_lookup_user(err))?
            .ok_or_else(|| {
                self.logger
                    .user_not_found(RequestResetPasswordTokenError::NotFound)
            })?;

        let reset_password_id = self.infra.id_generator().generate();
        let requested_at = self.infra.clock().now();

        let expires = self
            .logger
            .calculate_token_expires(requested_at.expires(&self.infra.config().token_expires));

        self.infra
            .repository()
            .register_reset_token(
                reset_password_id.clone(),
                user_id,
                destination.clone(),
                expires.clone(),
                requested_at,
            )
            .await
            .map_err(|err| self.logger.failed_to_register_reset_token(err))?;

        let token_encoded = self
            .infra
            .token_encoder()
            .encode(reset_password_id, expires)
            .map_err(|err| self.logger.failed_to_encode_reset_token(err))?;

        let response = self
            .infra
            .token_notifier()
            .notify(destination, token_encoded)
            .await
            .map_err(|err| self.logger.failed_to_notify_reset_token(err))?;

        Ok(self
            .logger
            .succeed_to_request_reset_password_token(response))
    }
}
