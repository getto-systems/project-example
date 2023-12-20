use std::sync::Arc;

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::{
    auth::user::{
        kernel::detail::repository::dynamodb::{
            login_id::{ConnectionLoginId, TableLoginId},
            user::{ConnectionUser, TableUser},
        },
        password::kernel::detail::password_matcher::Argon2PasswordMatcher,
    },
    common::api::{feature::AsInfra, logger::detail::StdoutJsonLogger},
};

use crate::auth::user::password::{
    authenticate::infra::{
        AuthenticateWithPasswordInfra, AuthenticateWithPasswordLogger,
        AuthenticateWithPasswordRepository,
    },
    kernel::infra::HashedPassword,
};

use crate::{
    auth::{
        ticket::kernel::data::{AuthPermissionGranted, AuthenticateSuccess},
        user::{
            kernel::data::AuthUserId,
            login_id::kernel::data::LoginId,
            password::{
                authenticate::data::{
                    AuthenticateWithPasswordError, ValidateAuthenticateWithPasswordFieldsError,
                },
                kernel::data::PasswordHashError,
            },
        },
    },
    common::api::repository::data::RepositoryError,
};

pub struct LiveAuthenticateWithPasswordInfra {
    repository: LiveAuthenticatePasswordRepository,
}

impl AsInfra<LiveAuthenticateWithPasswordInfra> for Arc<AuthAppFeature> {
    fn as_infra(&self) -> LiveAuthenticateWithPasswordInfra {
        LiveAuthenticateWithPasswordInfra {
            repository: LiveAuthenticatePasswordRepository {
                login_id: self.as_infra(),
                user: self.as_infra(),
            },
        }
    }
}

impl AuthenticateWithPasswordInfra for LiveAuthenticateWithPasswordInfra {
    type Repository = LiveAuthenticatePasswordRepository;
    type PasswordMatcher = Argon2PasswordMatcher;

    fn repository(&self) -> &Self::Repository {
        &self.repository
    }
}

pub struct LiveAuthenticatePasswordRepository {
    login_id: ConnectionLoginId,
    user: ConnectionUser,
}

#[async_trait::async_trait]
impl AuthenticateWithPasswordRepository for LiveAuthenticatePasswordRepository {
    async fn lookup_user_id(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<AuthUserId>, RepositoryError> {
        TableLoginId::get_user_id(&self.login_id, login_id.clone()).await
    }
    async fn lookup_password_and_granted(
        &self,
        user_id: &AuthUserId,
    ) -> Result<Option<(HashedPassword, Option<AuthPermissionGranted>)>, RepositoryError> {
        TableUser::get_password_and_granted(&self.user, user_id.clone()).await
    }
}

impl AuthenticateWithPasswordLogger for StdoutJsonLogger {
    fn try_to_authenticate_with_password(&self) {
        self.info(format!("try to authenticate with password"));
    }
    fn invalid_request(
        &self,
        err: ValidateAuthenticateWithPasswordFieldsError,
    ) -> ValidateAuthenticateWithPasswordFieldsError {
        self.fatal(format!(
            "invalid authenticate with password request; {}",
            err
        ));
        err
    }
    fn failed_to_lookup_user(&self, err: RepositoryError) -> RepositoryError {
        self.fatal(format!("failed to lookup user; {}", err));
        err
    }
    fn failed_to_lookup_password_and_granted(&self, err: RepositoryError) -> RepositoryError {
        self.fatal(format!("failed to lookup password and granted; {}", err));
        err
    }
    fn user_not_found(&self, err: AuthenticateWithPasswordError) -> AuthenticateWithPasswordError {
        self.incident(format!("user not found; {}", err));
        err
    }
    fn failed_to_match_password(&self, err: PasswordHashError) -> PasswordHashError {
        self.fatal(format!("failed to match password; {}", err));
        err
    }
    fn password_not_matched(
        &self,
        err: AuthenticateWithPasswordError,
    ) -> AuthenticateWithPasswordError {
        self.incident(format!("password not matched; {}", err));
        err
    }
    fn authenticated(&self, auth: AuthenticateSuccess) -> AuthenticateSuccess {
        self.info(format!("authenticated; {}", auth));
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
            password::kernel::detail::password_matcher::test::PlainPasswordMatcher,
        },
        common::api::feature::AsInfra,
    };

    use crate::auth::user::password::{
        authenticate::infra::{AuthenticateWithPasswordInfra, AuthenticateWithPasswordRepository},
        kernel::infra::HashedPassword,
    };

    use crate::{
        auth::{
            ticket::kernel::data::AuthPermissionGranted,
            user::{kernel::data::AuthUserId, login_id::kernel::data::LoginId},
        },
        common::api::repository::data::RepositoryError,
    };

    pub struct MockAuthenticateWithPasswordInfra {
        repository: MockAuthenticatePasswordRepository,
    }

    impl AsInfra<MockAuthenticateWithPasswordInfra> for (Arc<StoreLoginId>, Arc<StoreUser>) {
        fn as_infra(&self) -> MockAuthenticateWithPasswordInfra {
            MockAuthenticateWithPasswordInfra {
                repository: MockAuthenticatePasswordRepository {
                    login_id: Arc::clone(&self.0),
                    user: Arc::clone(&self.1),
                },
            }
        }
    }

    impl AuthenticateWithPasswordInfra for MockAuthenticateWithPasswordInfra {
        type Repository = MockAuthenticatePasswordRepository;
        type PasswordMatcher = PlainPasswordMatcher;

        fn repository(&self) -> &Self::Repository {
            &self.repository
        }
    }

    pub struct MockAuthenticatePasswordRepository {
        login_id: Arc<StoreLoginId>,
        user: Arc<StoreUser>,
    }

    #[async_trait::async_trait]
    impl AuthenticateWithPasswordRepository for MockAuthenticatePasswordRepository {
        async fn lookup_user_id(
            &self,
            login_id: &LoginId,
        ) -> Result<Option<AuthUserId>, RepositoryError> {
            Ok(MapLoginId::get_user_id(&self.login_id, login_id))
        }
        async fn lookup_password_and_granted(
            &self,
            user_id: &AuthUserId,
        ) -> Result<Option<(HashedPassword, Option<AuthPermissionGranted>)>, RepositoryError>
        {
            Ok(MapUser::get_password_and_granted(&self.user, user_id))
        }
    }
}
