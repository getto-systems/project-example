mod detail;

use std::sync::Arc;

use crate::auth::user::account::unregister::infra::{
    UnregisterAuthUserAccountFieldsExtract, UnregisterAuthUserAccountInfra,
    UnregisterAuthUserAccountLogger, UnregisterAuthUserAccountRepository,
};

use crate::auth::{
    ticket::kernel::data::AuthPermissionRequired,
    user::account::unregister::data::{
        UnregisterAuthUserAccountError, UnregisterAuthUserAccountSuccess,
    },
};

pub struct UnregisterAuthUserAccountAction<M: UnregisterAuthUserAccountInfra> {
    infra: M,
    logger: Arc<dyn UnregisterAuthUserAccountLogger>,
}

pub struct UnregisterAuthUserAccountInfo;

impl UnregisterAuthUserAccountInfo {
    pub fn required() -> AuthPermissionRequired {
        AuthPermissionRequired::user()
    }
}

impl<M: UnregisterAuthUserAccountInfra> UnregisterAuthUserAccountAction<M> {
    pub async fn unregister(
        &self,
        fields: impl UnregisterAuthUserAccountFieldsExtract,
    ) -> Result<UnregisterAuthUserAccountSuccess, UnregisterAuthUserAccountError> {
        self.logger.try_to_unregister_auth_user_account();

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
                    .user_id_not_found(UnregisterAuthUserAccountError::NotFound)
            })?;

        self.infra
            .repository()
            .unregister_user(&user_id, &fields.login_id)
            .await
            .map_err(|err| self.logger.failed_to_unregister_user(err))?;

        self.infra
            .repository()
            .discard_all_ticket(&user_id)
            .await
            .map_err(|err| self.logger.failed_to_discard_all_ticket(err))?;

        Ok(self
            .logger
            .succeed_to_unregister_auth_user_account(UnregisterAuthUserAccountSuccess))
    }
}
