mod detail;

use std::sync::Arc;

use crate::auth::user::account::modify::infra::{
    ModifyAuthUserAccountFieldsExtract, ModifyAuthUserAccountInfra, ModifyAuthUserAccountLogger,
    ModifyAuthUserAccountRepository,
};

use crate::auth::{
    ticket::kernel::data::AuthPermissionRequired,
    user::account::modify::data::{ModifyAuthUserAccountError, ModifyAuthUserAccountSuccess},
};

pub struct ModifyAuthUserAccountAction<M: ModifyAuthUserAccountInfra> {
    infra: M,
    logger: Arc<dyn ModifyAuthUserAccountLogger>,
}

pub struct ModifyAuthUserAccountInfo;

impl ModifyAuthUserAccountInfo {
    pub fn required() -> AuthPermissionRequired {
        AuthPermissionRequired::user()
    }
}

impl<M: ModifyAuthUserAccountInfra> ModifyAuthUserAccountAction<M> {
    pub async fn modify(
        &self,
        fields: impl ModifyAuthUserAccountFieldsExtract,
    ) -> Result<ModifyAuthUserAccountSuccess, ModifyAuthUserAccountError> {
        self.logger.try_to_modify_auth_user_account();

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
                    .user_id_not_found(ModifyAuthUserAccountError::NotFound)
            })?;

        let stored_user = self
            .infra
            .repository()
            .lookup_attrs(&user_id)
            .await
            .map_err(|err| self.logger.failed_to_lookup_attrs(err))?
            .ok_or_else(|| {
                self.logger
                    .user_attrs_not_found(ModifyAuthUserAccountError::NotFound)
            })?;

        if stored_user != fields.from {
            return Err(self.logger.conflict(ModifyAuthUserAccountError::Conflict));
        }

        self.infra
            .repository()
            .modify_user(user_id, fields.to)
            .await
            .map_err(|err| self.logger.failed_to_modify_attrs(err))?;

        Ok(self
            .logger
            .succeed_to_modify_auth_user_account(ModifyAuthUserAccountSuccess))
    }
}
