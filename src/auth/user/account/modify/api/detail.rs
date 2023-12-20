use std::sync::Arc;

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::{
    auth::{
        feature::AsAuthorizedInfra,
        user::kernel::detail::repository::dynamodb::{
            login_id::{ConnectionLoginId, TableLoginId},
            user::{ConnectionUser, TableUser},
        },
    },
    common::api::{feature::AsInfra, logger::detail::StdoutJsonLogger},
};

use crate::auth::user::account::modify::action::ModifyAuthUserAccountInfo;

use crate::auth::user::account::modify::infra::{
    ModifyAuthUserAccountInfra, ModifyAuthUserAccountLogger, ModifyAuthUserAccountRepository,
};

use crate::{
    auth::{
        ticket::{authorize::data::AuthorizeSuccess, kernel::data::AuthPermissionRequired},
        user::{
            account::{
                kernel::data::AuthUserAccountAttrs,
                modify::data::{
                    ModifyAuthUserAccountError, ModifyAuthUserAccountSuccess,
                    ValidateModifyAuthUserAccountFieldsError,
                },
            },
            kernel::data::AuthUserId,
            login_id::kernel::data::LoginId,
        },
    },
    common::api::repository::data::RepositoryError,
};

pub struct LiveModifyAuthUserAccountInfra {
    repository: LiveModifyAuthUserAccountRepository,
}

impl AsAuthorizedInfra<LiveModifyAuthUserAccountInfra> for Arc<AuthAppFeature> {
    fn required(&self) -> AuthPermissionRequired {
        ModifyAuthUserAccountInfo::required()
    }
    fn as_authorized_infra(&self, _: &AuthorizeSuccess) -> LiveModifyAuthUserAccountInfra {
        LiveModifyAuthUserAccountInfra {
            repository: LiveModifyAuthUserAccountRepository {
                login_id: self.as_infra(),
                user: self.as_infra(),
            },
        }
    }
}

impl ModifyAuthUserAccountInfra for LiveModifyAuthUserAccountInfra {
    type Repository = LiveModifyAuthUserAccountRepository;

    fn repository(&self) -> &Self::Repository {
        &self.repository
    }
}

pub struct LiveModifyAuthUserAccountRepository {
    login_id: ConnectionLoginId,
    user: ConnectionUser,
}

#[async_trait::async_trait]
impl ModifyAuthUserAccountRepository for LiveModifyAuthUserAccountRepository {
    async fn lookup_user_id(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<AuthUserId>, RepositoryError> {
        TableLoginId::get_user_id(&self.login_id, login_id.clone()).await
    }

    async fn lookup_attrs(
        &self,
        user_id: &AuthUserId,
    ) -> Result<Option<AuthUserAccountAttrs>, RepositoryError> {
        TableUser::get_attrs(&self.user, user_id.clone()).await
    }

    async fn modify_user(
        &self,
        user_id: AuthUserId,
        attrs: AuthUserAccountAttrs,
    ) -> Result<(), RepositoryError> {
        TableUser::update_user(&self.user, user_id, attrs).await
    }
}

impl ModifyAuthUserAccountLogger for StdoutJsonLogger {
    fn try_to_modify_auth_user_account(&self) {
        self.info(format!("try to modify auth-user-account"));
    }
    fn invalid_request(
        &self,
        err: ValidateModifyAuthUserAccountFieldsError,
    ) -> ValidateModifyAuthUserAccountFieldsError {
        self.fatal(format!(
            "failed to validate modify auth-user-account fields; {}",
            err
        ));
        err
    }
    fn failed_to_lookup_user_id(&self, err: RepositoryError) -> RepositoryError {
        self.fatal(format!("failed to lookup user-id; {}", err));
        err
    }
    fn user_id_not_found(&self, err: ModifyAuthUserAccountError) -> ModifyAuthUserAccountError {
        self.fatal(format!("user-id not found; {}", err));
        err
    }
    fn failed_to_lookup_attrs(&self, err: RepositoryError) -> RepositoryError {
        self.fatal(format!("failed to lookup user attrs; {}", err));
        err
    }
    fn user_attrs_not_found(&self, err: ModifyAuthUserAccountError) -> ModifyAuthUserAccountError {
        self.fatal(format!("user attrs not found; {}", err));
        err
    }
    fn conflict(&self, err: ModifyAuthUserAccountError) -> ModifyAuthUserAccountError {
        self.fatal(format!("conflicted; {}", err));
        err
    }
    fn failed_to_modify_attrs(&self, err: RepositoryError) -> RepositoryError {
        self.fatal(format!("failed to modify user attrs; {}", err));
        err
    }
    fn succeed_to_modify_auth_user_account(
        &self,
        success: ModifyAuthUserAccountSuccess,
    ) -> ModifyAuthUserAccountSuccess {
        self.info(format!("succeed to modify auth-user-account"));
        success
    }
}

#[cfg(test)]
pub mod test {
    use std::sync::Arc;

    use crate::{
        auth::user::kernel::detail::repository::memory::{
            login_id::MapLoginId, user::MapUser, StoreLoginId, StoreUser,
        },
        common::api::feature::AsInfra,
    };

    use crate::auth::user::account::modify::infra::{
        ModifyAuthUserAccountInfra, ModifyAuthUserAccountRepository,
    };

    use crate::{
        auth::user::{
            account::kernel::data::AuthUserAccountAttrs, kernel::data::AuthUserId,
            login_id::kernel::data::LoginId,
        },
        common::api::repository::data::RepositoryError,
    };

    pub struct MockModifyAuthUserAccountInfra {
        repository: MockModifyAuthUserAccountRepository,
    }

    impl AsInfra<MockModifyAuthUserAccountInfra> for (Arc<StoreLoginId>, Arc<StoreUser>) {
        fn as_infra(&self) -> MockModifyAuthUserAccountInfra {
            MockModifyAuthUserAccountInfra {
                repository: MockModifyAuthUserAccountRepository {
                    login_id: Arc::clone(&self.0),
                    user: Arc::clone(&self.1),
                },
            }
        }
    }

    impl ModifyAuthUserAccountInfra for MockModifyAuthUserAccountInfra {
        type Repository = MockModifyAuthUserAccountRepository;

        fn repository(&self) -> &Self::Repository {
            &self.repository
        }
    }

    pub struct MockModifyAuthUserAccountRepository {
        login_id: Arc<StoreLoginId>,
        user: Arc<StoreUser>,
    }

    #[async_trait::async_trait]
    impl<'a> ModifyAuthUserAccountRepository for MockModifyAuthUserAccountRepository {
        async fn lookup_user_id(
            &self,
            login_id: &LoginId,
        ) -> Result<Option<AuthUserId>, RepositoryError> {
            Ok(MapLoginId::get_user_id(&self.login_id, login_id))
        }

        async fn lookup_attrs(
            &self,
            user_id: &AuthUserId,
        ) -> Result<Option<AuthUserAccountAttrs>, RepositoryError> {
            Ok(MapUser::get_attrs(&self.user, user_id))
        }

        async fn modify_user(
            &self,
            user_id: AuthUserId,
            attrs: AuthUserAccountAttrs,
        ) -> Result<(), RepositoryError> {
            Ok(MapUser::update_user(&self.user, user_id, attrs))
        }
    }
}
