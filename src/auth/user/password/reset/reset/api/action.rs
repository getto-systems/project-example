mod detail;

use std::sync::Arc;

use crate::auth::{
    kernel::infra::AuthClock,
    user::password::{
        kernel::infra::AuthUserPasswordHasher,
        reset::reset::infra::{
            ResetPasswordFieldsExtract, ResetPasswordInfra, ResetPasswordLogger,
            ResetPasswordNotifier, ResetPasswordRepository, ResetPasswordTokenDecoder,
        },
    },
};

use crate::auth::{
    ticket::kernel::data::AuthenticateSuccess,
    user::{kernel::data::AuthUser, password::reset::reset::data::ResetPasswordError},
};

pub struct ResetPasswordAction<M: ResetPasswordInfra> {
    infra: M,
    logger: Arc<dyn ResetPasswordLogger>,
}

impl<M: ResetPasswordInfra> ResetPasswordAction<M> {
    pub async fn reset(
        &self,
        fields: impl ResetPasswordFieldsExtract,
    ) -> Result<AuthenticateSuccess, ResetPasswordError> {
        self.logger.try_to_reset_password();

        let fields = fields
            .convert()
            .map_err(|err| self.logger.invalid_request(err))?;

        let reset_id = self
            .infra
            .token_decoder()
            .decode(fields.reset_token)
            .map_err(|err| self.logger.failed_to_decode_token(err))?;

        let reset_at = self.infra.clock().now();

        let (user_id, destination, moment) = self
            .infra
            .repository()
            .lookup_reset_token_entry(&reset_id)
            .await
            .map_err(|err| self.logger.failed_to_lookup_reset_token_entry(err))?
            .ok_or_else(|| {
                self.logger
                    .reset_token_not_found(ResetPasswordError::NotFound)
            })?;

        if moment.has_already_reset() {
            return Err(self.logger.already_reset(ResetPasswordError::AlreadyReset));
        }

        if moment.has_expired(&reset_at) {
            return Err(self.logger.expired(ResetPasswordError::ResetTokenExpired));
        }

        let hashed_password = self
            .infra
            .password_hasher(fields.new_password)
            .hash_password()
            .map_err(|err| self.logger.failed_to_hash_password(err))?;

        self.infra
            .repository()
            .consume_reset_id(reset_id, reset_at)
            .await
            .map_err(|err| self.logger.failed_to_consume_reset_id(err))?;

        self.infra
            .repository()
            .update_password(user_id.clone(), hashed_password)
            .await
            .map_err(|err| self.logger.failed_to_update_password(err))?;

        self.logger.succeed_to_notify(
            self.infra
                .reset_notifier()
                .notify(destination)
                .await
                .map_err(|err| self.logger.failed_to_notify(err))?,
        );

        let granted = self
            .infra
            .repository()
            .lookup_permission_granted(&user_id)
            .await
            .map_err(|err| self.logger.failed_to_lookup_permission_granted(err))?;

        Ok(self
            .logger
            .succeed_to_reset_password(AuthenticateSuccess::new(AuthUser {
                user_id,
                granted: granted.unwrap_or_default(),
            })))
    }
}
