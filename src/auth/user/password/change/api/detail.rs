use std::sync::Arc;

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::{
    auth::{
        feature::AsAuthorizedInfra,
        user::{
            kernel::detail::repository::dynamodb::{
                login_id::{ConnectionLoginId, TableLoginId},
                user::{ConnectionUser, TableUser},
            },
            password::kernel::detail::{
                password_hasher::Argon2PasswordHasher, password_matcher::Argon2PasswordMatcher,
            },
        },
    },
    common::api::{feature::AsInfra, logger::detail::StdoutJsonLogger},
};

use crate::auth::user::password::change::action::{ChangePasswordInfo, OverwritePasswordInfo};

use crate::auth::user::password::{
    change::infra::{
        ChangePasswordInfra, ChangePasswordLogger, ChangePasswordRepository,
        OverwritePasswordInfra, OverwritePasswordLogger, OverwritePasswordRepository,
    },
    kernel::infra::HashedPassword,
};

use crate::{
    auth::{
        ticket::{authorize::data::AuthorizeSuccess, kernel::data::AuthPermissionRequired},
        user::{
            kernel::data::AuthUserId,
            login_id::kernel::data::LoginId,
            password::{
                change::data::{
                    ChangePasswordError, ChangePasswordSuccess, OverwritePasswordError,
                    OverwritePasswordSuccess, ValidateChangePasswordFieldsError,
                    ValidateOverwritePasswordFieldsError,
                },
                kernel::data::PasswordHashError,
            },
        },
    },
    common::api::repository::data::RepositoryError,
};

pub struct LiveChangePasswordInfra {
    repository: LiveChangePasswordRepository,
}

impl AsAuthorizedInfra<LiveChangePasswordInfra> for Arc<AuthAppFeature> {
    fn required(&self) -> AuthPermissionRequired {
        ChangePasswordInfo::required()
    }
    fn as_authorized_infra(&self, _: &AuthorizeSuccess) -> LiveChangePasswordInfra {
        LiveChangePasswordInfra {
            repository: LiveChangePasswordRepository {
                user: self.as_infra(),
            },
        }
    }
}

impl ChangePasswordInfra for LiveChangePasswordInfra {
    type Repository = LiveChangePasswordRepository;
    type PasswordMatcher = Argon2PasswordMatcher;
    type PasswordHasher = Argon2PasswordHasher;

    fn repository(&self) -> &Self::Repository {
        &self.repository
    }
}

pub struct LiveChangePasswordRepository {
    user: ConnectionUser,
}

#[async_trait::async_trait]
impl ChangePasswordRepository for LiveChangePasswordRepository {
    async fn lookup_password(
        &self,
        user_id: &AuthUserId,
    ) -> Result<Option<HashedPassword>, RepositoryError> {
        TableUser::get_password(&self.user, user_id.clone()).await
    }

    async fn change_password(
        &self,
        user_id: AuthUserId,
        new_password: HashedPassword,
    ) -> Result<(), RepositoryError> {
        TableUser::update_password(&self.user, user_id, new_password).await
    }
}

impl ChangePasswordLogger for StdoutJsonLogger {
    fn try_to_change_password(&self) {
        self.info(format!("try to change password"));
    }
    fn invalid_request(
        &self,
        err: ValidateChangePasswordFieldsError,
    ) -> ValidateChangePasswordFieldsError {
        self.incident(format!(
            "failed to validate change password fields; {}",
            err
        ));
        err
    }
    fn failed_to_lookup_password(&self, err: RepositoryError) -> RepositoryError {
        self.fatal(format!("failed to lookup password; {}", err));
        err
    }
    fn password_not_found(&self, err: ChangePasswordError) -> ChangePasswordError {
        self.incident(format!("password not found; {}", err));
        err
    }
    fn failed_to_match_password(&self, err: PasswordHashError) -> PasswordHashError {
        self.fatal(format!("failed to match password; {}", err));
        err
    }
    fn password_not_matched(&self, err: ChangePasswordError) -> ChangePasswordError {
        self.incident(format!("password not matched; {}", err));
        err
    }
    fn failed_to_hash_password(&self, err: PasswordHashError) -> PasswordHashError {
        self.fatal(format!("failed to hash password; {}", err));
        err
    }
    fn failed_to_change_password(&self, err: RepositoryError) -> RepositoryError {
        self.fatal(format!("failed to change password; {}", err));
        err
    }
    fn succeed_to_change_password(&self, auth: ChangePasswordSuccess) -> ChangePasswordSuccess {
        self.info(format!("succeed to change password"));
        auth
    }
}

pub struct LiveOverwritePasswordInfra {
    repository: LiveOverwritePasswordRepository,
}

impl AsAuthorizedInfra<LiveOverwritePasswordInfra> for Arc<AuthAppFeature> {
    fn required(&self) -> AuthPermissionRequired {
        OverwritePasswordInfo::required()
    }
    fn as_authorized_infra(&self, _: &AuthorizeSuccess) -> LiveOverwritePasswordInfra {
        LiveOverwritePasswordInfra {
            repository: LiveOverwritePasswordRepository {
                login_id: self.as_infra(),
                user: self.as_infra(),
            },
        }
    }
}

impl OverwritePasswordInfra for LiveOverwritePasswordInfra {
    type Repository = LiveOverwritePasswordRepository;
    type PasswordHasher = Argon2PasswordHasher;

    fn repository(&self) -> &Self::Repository {
        &self.repository
    }
}

pub struct LiveOverwritePasswordRepository {
    login_id: ConnectionLoginId,
    user: ConnectionUser,
}

#[async_trait::async_trait]
impl OverwritePasswordRepository for LiveOverwritePasswordRepository {
    async fn lookup_user_id(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<AuthUserId>, RepositoryError> {
        TableLoginId::get_user_id(&self.login_id, login_id.clone()).await
    }

    async fn overwrite_password(
        &self,
        user_id: AuthUserId,
        new_password: HashedPassword,
    ) -> Result<(), RepositoryError> {
        TableUser::update_password(&self.user, user_id, new_password).await
    }
}

impl OverwritePasswordLogger for StdoutJsonLogger {
    fn try_to_overwrite_password(&self) {
        self.info(format!("try to overwrite password"));
    }
    fn invalid_request(
        &self,
        err: ValidateOverwritePasswordFieldsError,
    ) -> ValidateOverwritePasswordFieldsError {
        self.incident(format!(
            "failed to validate overwrite password fields; {}",
            err
        ));
        err
    }
    fn failed_to_lookup_user_id(&self, err: RepositoryError) -> RepositoryError {
        self.fatal(format!("failed to lookup user-id; {}", err));
        err
    }
    fn user_id_not_found(&self, err: OverwritePasswordError) -> OverwritePasswordError {
        self.incident(format!("user-id not found; {}", err));
        err
    }
    fn failed_to_hash_password(&self, err: PasswordHashError) -> PasswordHashError {
        self.fatal(format!("failed to hash password; {}", err));
        err
    }
    fn failed_to_overwrite_password(&self, err: RepositoryError) -> RepositoryError {
        self.fatal(format!("failed to overwrite password; {}", err));
        err
    }
    fn succeed_to_overwrite_password(
        &self,
        auth: OverwritePasswordSuccess,
    ) -> OverwritePasswordSuccess {
        self.info(format!("succeed to overwrite password"));
        auth
    }
}

#[cfg(test)]
pub mod test {
    use std::sync::Arc;

    use crate::{
        auth::user::{
            kernel::detail::repository::memory::{
                login_id::MapLoginId, user::MapUser, StoreLoginId, StoreUser,
            },
            login_id::kernel::data::LoginId,
            password::{
                change::infra::{OverwritePasswordInfra, OverwritePasswordRepository},
                kernel::detail::{
                    password_hasher::test::PlainPasswordHasher,
                    password_matcher::test::PlainPasswordMatcher,
                },
            },
        },
        common::api::feature::AsInfra,
    };

    use crate::auth::user::password::{
        change::infra::{ChangePasswordInfra, ChangePasswordRepository},
        kernel::infra::HashedPassword,
    };

    use crate::{
        auth::user::kernel::data::AuthUserId, common::api::repository::data::RepositoryError,
    };

    pub struct MockChangePasswordInfra {
        repository: MockChangePasswordRepository,
    }

    impl AsInfra<MockChangePasswordInfra> for Arc<StoreUser> {
        fn as_infra(&self) -> MockChangePasswordInfra {
            MockChangePasswordInfra {
                repository: MockChangePasswordRepository {
                    user: Arc::clone(self),
                },
            }
        }
    }

    impl ChangePasswordInfra for MockChangePasswordInfra {
        type Repository = MockChangePasswordRepository;
        type PasswordMatcher = PlainPasswordMatcher;
        type PasswordHasher = PlainPasswordHasher;

        fn repository(&self) -> &Self::Repository {
            &self.repository
        }
    }

    pub struct MockChangePasswordRepository {
        user: Arc<StoreUser>,
    }

    #[async_trait::async_trait]
    impl ChangePasswordRepository for MockChangePasswordRepository {
        async fn lookup_password(
            &self,
            user_id: &AuthUserId,
        ) -> Result<Option<HashedPassword>, RepositoryError> {
            Ok(MapUser::get_password(&self.user, user_id))
        }

        async fn change_password(
            &self,
            user_id: AuthUserId,
            new_password: HashedPassword,
        ) -> Result<(), RepositoryError> {
            Ok(MapUser::update_password(&self.user, user_id, new_password))
        }
    }

    pub struct MockOverwritePasswordInfra {
        repository: MockOverwritePasswordRepository,
    }

    impl AsInfra<MockOverwritePasswordInfra> for (Arc<StoreLoginId>, Arc<StoreUser>) {
        fn as_infra(&self) -> MockOverwritePasswordInfra {
            MockOverwritePasswordInfra {
                repository: MockOverwritePasswordRepository {
                    login_id: Arc::clone(&self.0),
                    user: Arc::clone(&self.1),
                },
            }
        }
    }

    impl OverwritePasswordInfra for MockOverwritePasswordInfra {
        type Repository = MockOverwritePasswordRepository;
        type PasswordHasher = PlainPasswordHasher;

        fn repository(&self) -> &Self::Repository {
            &self.repository
        }
    }

    pub struct MockOverwritePasswordRepository {
        login_id: Arc<StoreLoginId>,
        user: Arc<StoreUser>,
    }

    #[async_trait::async_trait]
    impl OverwritePasswordRepository for MockOverwritePasswordRepository {
        async fn lookup_user_id(
            &self,
            login_id: &LoginId,
        ) -> Result<Option<AuthUserId>, RepositoryError> {
            Ok(MapLoginId::get_user_id(&self.login_id, login_id))
        }

        async fn overwrite_password(
            &self,
            user_id: AuthUserId,
            new_password: HashedPassword,
        ) -> Result<(), RepositoryError> {
            Ok(MapUser::update_password(&self.user, user_id, new_password))
        }
    }
}
