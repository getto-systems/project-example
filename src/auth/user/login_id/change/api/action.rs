mod detail;

use std::sync::Arc;

use crate::auth::user::login_id::change::infra::{
    OverwriteLoginIdFieldsExtract, OverwriteLoginIdInfra, OverwriteLoginIdLogger,
    OverwriteLoginIdRepository,
};

use crate::auth::{
    ticket::kernel::data::AuthPermissionRequired,
    user::login_id::change::data::{OverwriteLoginIdError, OverwriteLoginIdSuccess},
};

pub struct OverwriteLoginIdAction<M: OverwriteLoginIdInfra> {
    infra: M,
    logger: Arc<dyn OverwriteLoginIdLogger>,
}

pub struct OverwriteLoginIdInfo;

impl OverwriteLoginIdInfo {
    pub fn required() -> AuthPermissionRequired {
        AuthPermissionRequired::Nothing
    }
}

impl<M: OverwriteLoginIdInfra> OverwriteLoginIdAction<M> {
    pub async fn overwrite(
        &self,
        fields: impl OverwriteLoginIdFieldsExtract,
    ) -> Result<OverwriteLoginIdSuccess, OverwriteLoginIdError> {
        let fields = fields
            .convert()
            .map_err(|err| self.logger.invalid_request(err))?;

        if self
            .infra
            .repository()
            .check_login_id_registered(&fields.new_login_id)
            .await
            .map_err(|err| self.logger.failed_to_check_login_id_registered(err))?
        {
            return Err(self
                .logger
                .login_id_already_registered(OverwriteLoginIdError::AlreadyRegistered));
        }

        let (user_id, reset_token_destination) = self
            .infra
            .repository()
            .drop_login_id_entry(&fields.login_id)
            .await
            .map_err(|err| self.logger.failed_to_drop_login_id_entry(err))?
            .ok_or_else(|| {
                self.logger
                    .login_id_entry_not_found(OverwriteLoginIdError::NotFound)
            })?;

        self.infra
            .repository()
            .update_login_id(user_id.clone(), fields.new_login_id.clone())
            .await
            .map_err(|err| self.logger.failed_to_update_login_id(err))?;

        self.infra
            .repository()
            .insert_login_id_entry(fields.new_login_id, user_id, reset_token_destination)
            .await
            .map_err(|err| self.logger.failed_to_insert_login_id_entry(err))?;

        Ok(self
            .logger
            .succeed_to_overwrite_login_id(OverwriteLoginIdSuccess))
    }
}
