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

use crate::auth::user::login_id::change::action::OverwriteLoginIdInfo;

use crate::auth::user::login_id::change::infra::{
    OverwriteLoginIdInfra, OverwriteLoginIdLogger, OverwriteLoginIdRepository,
};

use crate::{
    auth::{
        ticket::{authorize::data::AuthorizeSuccess, kernel::data::AuthPermissionRequired},
        user::{
            kernel::data::AuthUserId,
            login_id::{
                change::data::{
                    OverwriteLoginIdError, OverwriteLoginIdSuccess,
                    ValidateOverwriteLoginIdFieldsError,
                },
                kernel::data::LoginId,
            },
            password::reset::kernel::data::ResetPasswordTokenDestination,
        },
    },
    common::api::repository::data::RepositoryError,
};

pub struct LiveOverwriteLoginIdInfra {
    repository: LiveOverwriteLoginIdRepository,
}

impl AsAuthorizedInfra<LiveOverwriteLoginIdInfra> for Arc<AuthAppFeature> {
    fn required(&self) -> AuthPermissionRequired {
        OverwriteLoginIdInfo::required()
    }
    fn as_authorized_infra(&self, _: &AuthorizeSuccess) -> LiveOverwriteLoginIdInfra {
        LiveOverwriteLoginIdInfra {
            repository: LiveOverwriteLoginIdRepository {
                login_id: self.as_infra(),
                user: self.as_infra(),
            },
        }
    }
}

impl OverwriteLoginIdInfra for LiveOverwriteLoginIdInfra {
    type Repository = LiveOverwriteLoginIdRepository;

    fn repository(&self) -> &Self::Repository {
        &self.repository
    }
}

pub struct LiveOverwriteLoginIdRepository {
    login_id: ConnectionLoginId,
    user: ConnectionUser,
}

#[async_trait::async_trait]
impl OverwriteLoginIdRepository for LiveOverwriteLoginIdRepository {
    async fn check_login_id_registered(&self, login_id: &LoginId) -> Result<bool, RepositoryError> {
        Ok(TableLoginId::get_user_id(&self.login_id, login_id.clone())
            .await?
            .is_some())
    }

    async fn drop_login_id_entry(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<(AuthUserId, ResetPasswordTokenDestination)>, RepositoryError> {
        TableLoginId::delete_entry(&self.login_id, login_id.clone()).await
    }
    async fn insert_login_id_entry(
        &self,
        login_id: LoginId,
        user_id: AuthUserId,
        reset_token_destination: ResetPasswordTokenDestination,
    ) -> Result<(), RepositoryError> {
        TableLoginId::put_overwrite_entry(
            &self.login_id,
            login_id,
            user_id,
            reset_token_destination,
        )
        .await
    }
    async fn update_login_id(
        &self,
        user_id: AuthUserId,
        login_id: LoginId,
    ) -> Result<(), RepositoryError> {
        TableUser::update_login_id(&self.user, user_id, login_id).await
    }
}

impl OverwriteLoginIdLogger for StdoutJsonLogger {
    fn try_to_overwrite_login_id(&self) {
        self.info(format!("try to overwrite login-id"));
    }
    fn invalid_request(
        &self,
        err: ValidateOverwriteLoginIdFieldsError,
    ) -> ValidateOverwriteLoginIdFieldsError {
        self.fatal(format!(
            "failed to validate overwrite login-id fields; {}",
            err
        ));
        err
    }
    fn failed_to_check_login_id_registered(&self, err: RepositoryError) -> RepositoryError {
        self.fatal(format!("failed to check login-id registered; {}", err));
        err
    }
    fn login_id_already_registered(&self, err: OverwriteLoginIdError) -> OverwriteLoginIdError {
        self.fatal(format!("login-id already registered; {}", err));
        err
    }
    fn failed_to_drop_login_id_entry(&self, err: RepositoryError) -> RepositoryError {
        self.fatal(format!("failed to drop login-id entry; {}", err));
        err
    }
    fn login_id_entry_not_found(&self, err: OverwriteLoginIdError) -> OverwriteLoginIdError {
        self.fatal(format!("login-id entry not found; {}", err));
        err
    }
    fn failed_to_insert_login_id_entry(&self, err: RepositoryError) -> RepositoryError {
        self.fatal(format!("failed to insert login-id entry; {}", err));
        err
    }
    fn failed_to_update_login_id(&self, err: RepositoryError) -> RepositoryError {
        self.fatal(format!("failed to update login-id; {}", err));
        err
    }
    fn succeed_to_overwrite_login_id(
        &self,
        auth: OverwriteLoginIdSuccess,
    ) -> OverwriteLoginIdSuccess {
        self.audit(format!("succeed to overwrite login-id"));
        auth
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

    use crate::auth::user::login_id::change::infra::{
        OverwriteLoginIdInfra, OverwriteLoginIdRepository,
    };

    use crate::{
        auth::user::{
            kernel::data::AuthUserId, login_id::kernel::data::LoginId,
            password::reset::kernel::data::ResetPasswordTokenDestination,
        },
        common::api::repository::data::RepositoryError,
    };

    pub struct MockOverwriteLoginIdInfra {
        repository: MockOverwriteLoginIdRepository,
    }

    impl AsInfra<MockOverwriteLoginIdInfra> for (Arc<StoreLoginId>, Arc<StoreUser>) {
        fn as_infra(&self) -> MockOverwriteLoginIdInfra {
            MockOverwriteLoginIdInfra {
                repository: MockOverwriteLoginIdRepository {
                    login_id: Arc::clone(&self.0),
                    user: Arc::clone(&self.1),
                },
            }
        }
    }

    impl OverwriteLoginIdInfra for MockOverwriteLoginIdInfra {
        type Repository = MockOverwriteLoginIdRepository;

        fn repository(&self) -> &Self::Repository {
            &self.repository
        }
    }

    pub struct MockOverwriteLoginIdRepository {
        login_id: Arc<StoreLoginId>,
        user: Arc<StoreUser>,
    }

    #[async_trait::async_trait]
    impl OverwriteLoginIdRepository for MockOverwriteLoginIdRepository {
        async fn check_login_id_registered(
            &self,
            login_id: &LoginId,
        ) -> Result<bool, RepositoryError> {
            Ok(MapLoginId::get_user_id(&self.login_id, login_id).is_some())
        }

        async fn drop_login_id_entry(
            &self,
            login_id: &LoginId,
        ) -> Result<Option<(AuthUserId, ResetPasswordTokenDestination)>, RepositoryError> {
            Ok(MapLoginId::remove_entry(&self.login_id, login_id))
        }
        async fn insert_login_id_entry(
            &self,
            login_id: LoginId,
            user_id: AuthUserId,
            reset_token_destination: ResetPasswordTokenDestination,
        ) -> Result<(), RepositoryError> {
            Ok(MapLoginId::insert_entry(
                &self.login_id,
                login_id,
                user_id,
                reset_token_destination,
            ))
        }
        async fn update_login_id(
            &self,
            user_id: AuthUserId,
            login_id: LoginId,
        ) -> Result<(), RepositoryError> {
            Ok(MapUser::update_login_id(&self.user, user_id, login_id))
        }
    }
}
