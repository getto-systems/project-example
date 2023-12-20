mod detail;

use std::sync::Arc;

use crate::auth::user::password::{
    change::infra::{
        ChangePasswordFieldsExtract, ChangePasswordInfra, ChangePasswordLogger,
        ChangePasswordRepository, OverwritePasswordFieldsExtract, OverwritePasswordInfra,
        OverwritePasswordLogger, OverwritePasswordRepository,
    },
    kernel::infra::{AuthUserPasswordHasher, AuthUserPasswordMatcher},
};

use crate::auth::{
    ticket::kernel::data::AuthPermissionRequired,
    user::{
        kernel::data::AuthUserId,
        password::change::data::{
            ChangePasswordError, ChangePasswordSuccess, OverwritePasswordError,
            OverwritePasswordSuccess,
        },
    },
};

pub struct ChangePasswordAction<M: ChangePasswordInfra> {
    infra: M,
    logger: Arc<dyn ChangePasswordLogger>,
}

pub struct ChangePasswordInfo;

impl ChangePasswordInfo {
    pub fn required() -> AuthPermissionRequired {
        AuthPermissionRequired::Nothing
    }
}

impl<M: ChangePasswordInfra> ChangePasswordAction<M> {
    pub async fn change(
        &self,
        user_id: AuthUserId,
        fields: impl ChangePasswordFieldsExtract,
    ) -> Result<ChangePasswordSuccess, ChangePasswordError> {
        self.logger.try_to_change_password();

        let fields = fields
            .convert()
            .map_err(|err| self.logger.invalid_request(err))?;

        let stored_password = self
            .infra
            .repository()
            .lookup_password(&user_id)
            .await
            .map_err(|err| self.logger.failed_to_lookup_password(err))?
            .ok_or_else(|| {
                self.logger
                    .password_not_found(ChangePasswordError::NotFound)
            })?;

        if !self
            .infra
            .password_matcher(fields.current_password)
            .match_password(stored_password)
            .map_err(|err| self.logger.failed_to_match_password(err))?
        {
            return Err(self
                .logger
                .password_not_found(ChangePasswordError::PasswordNotMatched));
        }

        let hashed_password = self
            .infra
            .password_hasher(fields.new_password)
            .hash_password()
            .map_err(|err| self.logger.failed_to_hash_password(err))?;

        self.infra
            .repository()
            .change_password(user_id, hashed_password)
            .await
            .map_err(|err| self.logger.failed_to_change_password(err))?;

        Ok(self
            .logger
            .succeed_to_change_password(ChangePasswordSuccess))
    }
}

pub struct OverwritePasswordAction<M: OverwritePasswordInfra> {
    infra: M,
    logger: Arc<dyn OverwritePasswordLogger>,
}

pub struct OverwritePasswordInfo;

impl OverwritePasswordInfo {
    pub fn required() -> AuthPermissionRequired {
        AuthPermissionRequired::user()
    }
}

impl<M: OverwritePasswordInfra> OverwritePasswordAction<M> {
    pub async fn overwrite(
        &self,
        fields: impl OverwritePasswordFieldsExtract,
    ) -> Result<OverwritePasswordSuccess, OverwritePasswordError> {
        self.logger.try_to_overwrite_password();

        let fields = fields
            .convert()
            .map_err(|err| self.logger.invalid_request(err))?;

        let user_id = self
            .infra
            .repository()
            .lookup_user_id(&fields.login_id)
            .await
            .map_err(|err| self.logger.failed_to_lookup_user_id(err))?
            .ok_or_else(|| {
                self.logger
                    .user_id_not_found(OverwritePasswordError::NotFound)
            })?;

        let hashed_password = self
            .infra
            .password_hasher(fields.new_password)
            .hash_password()
            .map_err(|err| self.logger.failed_to_hash_password(err))?;

        self.infra
            .repository()
            .overwrite_password(user_id, hashed_password)
            .await
            .map_err(|err| self.logger.failed_to_overwrite_password(err))?;

        Ok(self
            .logger
            .succeed_to_overwrite_password(OverwritePasswordSuccess))
    }
}
