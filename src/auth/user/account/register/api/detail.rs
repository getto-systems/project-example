use std::sync::Arc;

use uuid::Uuid;

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

use crate::auth::user::account::register::action::RegisterAuthUserAccountInfo;

use crate::auth::user::account::register::infra::{
    AuthUserIdGenerator, RegisterAuthUserAccountInfra, RegisterAuthUserAccountLogger,
    RegisterAuthUserAccountRepository,
};

use crate::{
    auth::{
        ticket::{authorize::data::AuthorizeSuccess, kernel::data::AuthPermissionRequired},
        user::{
            account::{
                kernel::data::{AuthUserAccount, ValidateAuthUserAccountError},
                register::data::{RegisterAuthUserAccountError, RegisterAuthUserAccountSuccess},
            },
            kernel::data::AuthUserId,
            login_id::kernel::data::LoginId,
        },
    },
    common::api::repository::data::RepositoryError,
};

pub struct LiveRegisterAuthUserAccountInfra {
    user_id_generator: UuidAuthUserIdGenerator,
    repository: LiveRegisterAuthUserAccountRepository,
}

impl AsAuthorizedInfra<LiveRegisterAuthUserAccountInfra> for Arc<AuthAppFeature> {
    fn required(&self) -> AuthPermissionRequired {
        RegisterAuthUserAccountInfo::required()
    }
    fn as_authorized_infra(&self, _: &AuthorizeSuccess) -> LiveRegisterAuthUserAccountInfra {
        LiveRegisterAuthUserAccountInfra {
            user_id_generator: UuidAuthUserIdGenerator,
            repository: LiveRegisterAuthUserAccountRepository {
                login_id: self.as_infra(),
                user: self.as_infra(),
            },
        }
    }
}

impl RegisterAuthUserAccountInfra for LiveRegisterAuthUserAccountInfra {
    type UserIdGenerator = UuidAuthUserIdGenerator;
    type Repository = LiveRegisterAuthUserAccountRepository;

    fn user_id_generator(&self) -> &Self::UserIdGenerator {
        &self.user_id_generator
    }
    fn repository(&self) -> &Self::Repository {
        &self.repository
    }
}

pub struct UuidAuthUserIdGenerator;

impl AuthUserIdGenerator for UuidAuthUserIdGenerator {
    fn generate(&self) -> AuthUserId {
        AuthUserId::restore(Uuid::new_v4().to_string())
    }
}

pub struct LiveRegisterAuthUserAccountRepository {
    login_id: ConnectionLoginId,
    user: ConnectionUser,
}

#[async_trait::async_trait]
impl RegisterAuthUserAccountRepository for LiveRegisterAuthUserAccountRepository {
    async fn check_login_id_registered(&self, login_id: &LoginId) -> Result<bool, RepositoryError> {
        Ok(TableLoginId::get_user_id(&self.login_id, login_id.clone())
            .await?
            .is_some())
    }

    async fn register_user(
        &self,
        user_id: AuthUserId,
        fields: AuthUserAccount,
    ) -> Result<(), RepositoryError> {
        // login-id が衝突した場合に rollback する
        let result = TableLoginId::put_new_entry(
            &self.login_id,
            fields.login_id.clone(),
            user_id.clone(),
            fields.reset_token_destination,
        )
        .await;

        if result.is_err() {
            TableUser::delete_entry(&self.user, user_id.clone()).await?;
            return result;
        }

        TableUser::put_new_entry(
            &self.user,
            user_id,
            fields.login_id,
            fields.attrs.granted,
            fields.attrs.memo,
        )
        .await?;

        Ok(())
    }
}

impl RegisterAuthUserAccountLogger for StdoutJsonLogger {
    fn try_to_register_auth_user_account(&self) {
        self.info(format!("try to register auth-user-account"));
    }
    fn invalid_request(&self, err: ValidateAuthUserAccountError) -> ValidateAuthUserAccountError {
        self.fatal(format!(
            "failed to validate register auth-user-account fields; {}",
            err
        ));
        err
    }
    fn failed_to_check_login_id_registered(&self, err: RepositoryError) -> RepositoryError {
        self.fatal(format!("failed to lookup user-id; {}", err));
        err
    }
    fn login_id_already_registered(
        &self,
        err: RegisterAuthUserAccountError,
    ) -> RegisterAuthUserAccountError {
        self.fatal(format!("login-id already registered; {}", err));
        err
    }
    fn failed_to_register_user(&self, err: RepositoryError) -> RepositoryError {
        self.fatal(format!("failed to register user; {}", err));
        err
    }
    fn succeed_to_register_auth_user_account(
        &self,
        success: RegisterAuthUserAccountSuccess,
    ) -> RegisterAuthUserAccountSuccess {
        self.info(format!("succeed to register auth-user-account"));
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

    use crate::auth::user::account::register::infra::{
        AuthUserIdGenerator, RegisterAuthUserAccountInfra, RegisterAuthUserAccountRepository,
    };

    use crate::{
        auth::user::{
            account::kernel::data::AuthUserAccount, kernel::data::AuthUserId,
            login_id::kernel::data::LoginId,
        },
        common::api::repository::data::RepositoryError,
    };

    pub struct MockRegisterAuthUserAccountInfra {
        user_id_generator: MockAuthUserIdGenerator,
        repository: MockModifyAuthUserAccountRepository,
    }

    impl AsInfra<MockRegisterAuthUserAccountInfra>
        for (MockAuthUserIdGenerator, Arc<StoreLoginId>, Arc<StoreUser>)
    {
        fn as_infra(&self) -> MockRegisterAuthUserAccountInfra {
            MockRegisterAuthUserAccountInfra {
                user_id_generator: self.0.clone(),
                repository: MockModifyAuthUserAccountRepository {
                    login_id: Arc::clone(&self.1),
                    user: Arc::clone(&self.2),
                },
            }
        }
    }

    impl<'a> RegisterAuthUserAccountInfra for MockRegisterAuthUserAccountInfra {
        type UserIdGenerator = MockAuthUserIdGenerator;
        type Repository = MockModifyAuthUserAccountRepository;

        fn user_id_generator(&self) -> &Self::UserIdGenerator {
            &self.user_id_generator
        }
        fn repository(&self) -> &Self::Repository {
            &self.repository
        }
    }

    #[derive(Clone)]
    pub struct MockAuthUserIdGenerator {
        user_id: AuthUserId,
    }

    impl MockAuthUserIdGenerator {
        pub const fn new(user_id: AuthUserId) -> Self {
            Self { user_id }
        }
    }

    impl AuthUserIdGenerator for MockAuthUserIdGenerator {
        fn generate(&self) -> AuthUserId {
            self.user_id.clone()
        }
    }

    pub struct MockModifyAuthUserAccountRepository {
        login_id: Arc<StoreLoginId>,
        user: Arc<StoreUser>,
    }

    #[async_trait::async_trait]
    impl RegisterAuthUserAccountRepository for MockModifyAuthUserAccountRepository {
        async fn check_login_id_registered(
            &self,
            login_id: &LoginId,
        ) -> Result<bool, RepositoryError> {
            Ok(MapLoginId::get_user_id(&self.login_id, login_id).is_some())
        }

        async fn register_user(
            &self,
            user_id: AuthUserId,
            fields: AuthUserAccount,
        ) -> Result<(), RepositoryError> {
            MapUser::insert_entry(
                &self.user,
                user_id.clone(),
                (
                    fields.login_id.clone(),
                    Some(fields.attrs.granted),
                    None,
                    Some(fields.attrs.memo),
                ),
            );

            MapLoginId::insert_entry(
                &self.login_id,
                fields.login_id,
                user_id,
                fields.reset_token_destination,
            );

            Ok(())
        }
    }
}
