mod detail;

use std::sync::Arc;

use crate::auth::user::password::reset::token_destination::change::infra::{
    ChangeResetTokenDestinationFieldsExtract, ChangeResetTokenDestinationInfra,
    ChangeResetTokenDestinationLogger, ChangeResetTokenDestinationRepository,
};

use crate::auth::{
    ticket::kernel::data::AuthPermissionRequired,
    user::password::reset::token_destination::change::data::{
        ChangeResetTokenDestinationError, ChangeResetTokenDestinationSuccess,
    },
};

pub struct ChangeResetTokenDestinationAction<M: ChangeResetTokenDestinationInfra> {
    infra: M,
    logger: Arc<dyn ChangeResetTokenDestinationLogger>,
}

pub struct ChangeResetTokenDestinationInfo;

impl ChangeResetTokenDestinationInfo {
    pub fn required() -> AuthPermissionRequired {
        AuthPermissionRequired::user()
    }
}

impl<M: ChangeResetTokenDestinationInfra> ChangeResetTokenDestinationAction<M> {
    pub async fn change(
        &self,
        fields: impl ChangeResetTokenDestinationFieldsExtract,
    ) -> Result<ChangeResetTokenDestinationSuccess, ChangeResetTokenDestinationError> {
        self.logger.try_to_change_destination();

        let fields = fields
            .convert()
            .map_err(|err| self.logger.invalid_request(err))?;

        let stored_destination = self
            .infra
            .repository()
            .lookup_destination(&fields.login_id)
            .await
            .map_err(|err| self.logger.failed_to_lookup_destination(err))?
            .ok_or_else(|| {
                self.logger
                    .user_not_found(ChangeResetTokenDestinationError::NotFound)
            })?;

        if stored_destination != fields.from {
            return Err(self
                .logger
                .conflict(ChangeResetTokenDestinationError::Conflict));
        }

        self.infra
            .repository()
            .change_destination(fields.login_id, fields.to)
            .await
            .map_err(|err| self.logger.failed_to_change_destination(err))?;

        Ok(self
            .logger
            .succeed_to_change_destination(ChangeResetTokenDestinationSuccess))
    }
}
