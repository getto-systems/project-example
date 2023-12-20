mod detail;

use std::sync::Arc;

use crate::auth::user::account::register::infra::{
    AuthUserIdGenerator, RegisterAuthUserAccountFieldsExtract, RegisterAuthUserAccountInfra,
    RegisterAuthUserAccountLogger, RegisterAuthUserAccountRepository,
};

use crate::auth::{
    ticket::kernel::data::AuthPermissionRequired,
    user::account::register::data::{RegisterAuthUserAccountError, RegisterAuthUserAccountSuccess},
};

pub struct RegisterAuthUserAccountAction<M: RegisterAuthUserAccountInfra> {
    infra: M,
    logger: Arc<dyn RegisterAuthUserAccountLogger>,
}

pub struct RegisterAuthUserAccountInfo;

impl RegisterAuthUserAccountInfo {
    pub fn required() -> AuthPermissionRequired {
        AuthPermissionRequired::user()
    }
}

impl<M: RegisterAuthUserAccountInfra> RegisterAuthUserAccountAction<M> {
    pub async fn register(
        &self,
        fields: impl RegisterAuthUserAccountFieldsExtract,
    ) -> Result<RegisterAuthUserAccountSuccess, RegisterAuthUserAccountError> {
        self.logger.try_to_register_auth_user_account();

        let fields = fields
            .convert()
            .map_err(|err| self.logger.invalid_request(err))?;

        if self
            .infra
            .repository()
            .check_login_id_registered(&fields.login_id)
            .await
            .map_err(|err| self.logger.failed_to_check_login_id_registered(err))?
        {
            return Err(self.logger.login_id_already_registered(
                RegisterAuthUserAccountError::LoginIdAlreadyRegistered,
            ));
        }

        let user_id = self.infra.user_id_generator().generate();

        self.infra
            .repository()
            .register_user(user_id, fields)
            .await
            .map_err(|err| self.logger.failed_to_register_user(err))?;

        Ok(self
            .logger
            .succeed_to_register_auth_user_account(RegisterAuthUserAccountSuccess))
    }
}
