use std::sync::Arc;

use crate::auth::user::password::change::api::detail::{
    LiveChangePasswordInfra, LiveOverwritePasswordInfra,
};

use crate::auth::user::password::change::action::{ChangePasswordAction, OverwritePasswordAction};

use crate::auth::user::password::change::infra::{
    ChangePasswordInfra, ChangePasswordLogger, OverwritePasswordInfra, OverwritePasswordLogger,
};

use crate::{
    auth::user::password::{
        change::data::{
            ChangePasswordError, ChangePasswordSuccess, OverwritePasswordError,
            OverwritePasswordSuccess, ValidateChangePasswordFieldsError,
            ValidateOverwritePasswordFieldsError,
        },
        kernel::data::PasswordHashError,
    },
    common::api::repository::data::RepositoryError,
};

impl<M: ChangePasswordInfra> ChangePasswordAction<M> {
    fn new(infra: M) -> Self {
        Self {
            logger: Arc::new(ChangePasswordNoopLogger),
            infra,
        }
    }

    pub fn with_logger(self, logger: Arc<dyn ChangePasswordLogger>) -> Self {
        Self { logger, ..self }
    }
}

impl ChangePasswordAction<LiveChangePasswordInfra> {
    pub fn live(infra: LiveChangePasswordInfra) -> Self {
        Self::new(infra)
    }
}

struct ChangePasswordNoopLogger;

impl ChangePasswordLogger for ChangePasswordNoopLogger {
    fn try_to_change_password(&self) {}
    fn invalid_request(
        &self,
        err: ValidateChangePasswordFieldsError,
    ) -> ValidateChangePasswordFieldsError {
        err
    }
    fn failed_to_lookup_password(&self, err: RepositoryError) -> RepositoryError {
        err
    }
    fn password_not_found(&self, err: ChangePasswordError) -> ChangePasswordError {
        err
    }
    fn failed_to_match_password(&self, err: PasswordHashError) -> PasswordHashError {
        err
    }
    fn password_not_matched(&self, err: ChangePasswordError) -> ChangePasswordError {
        err
    }
    fn failed_to_hash_password(&self, err: PasswordHashError) -> PasswordHashError {
        err
    }
    fn failed_to_change_password(&self, err: RepositoryError) -> RepositoryError {
        err
    }
    fn succeed_to_change_password(&self, auth: ChangePasswordSuccess) -> ChangePasswordSuccess {
        auth
    }
}

impl<M: OverwritePasswordInfra> OverwritePasswordAction<M> {
    fn new(infra: M) -> Self {
        Self {
            logger: Arc::new(OverwritePasswordNoopLogger),
            infra,
        }
    }

    pub fn with_logger(self, logger: Arc<dyn OverwritePasswordLogger>) -> Self {
        Self { logger, ..self }
    }
}

impl OverwritePasswordAction<LiveOverwritePasswordInfra> {
    pub fn live(infra: LiveOverwritePasswordInfra) -> Self {
        Self::new(infra)
    }
}

struct OverwritePasswordNoopLogger;

impl OverwritePasswordLogger for OverwritePasswordNoopLogger {
    fn try_to_overwrite_password(&self) {}
    fn invalid_request(
        &self,
        err: ValidateOverwritePasswordFieldsError,
    ) -> ValidateOverwritePasswordFieldsError {
        err
    }
    fn failed_to_lookup_user_id(&self, err: RepositoryError) -> RepositoryError {
        err
    }
    fn user_id_not_found(&self, err: OverwritePasswordError) -> OverwritePasswordError {
        err
    }
    fn failed_to_hash_password(&self, err: PasswordHashError) -> PasswordHashError {
        err
    }
    fn failed_to_overwrite_password(&self, err: RepositoryError) -> RepositoryError {
        err
    }
    fn succeed_to_overwrite_password(
        &self,
        auth: OverwritePasswordSuccess,
    ) -> OverwritePasswordSuccess {
        auth
    }
}

#[cfg(test)]
mod test {
    use crate::auth::user::password::change::api::detail::test::{
        MockChangePasswordInfra, MockOverwritePasswordInfra,
    };

    use crate::auth::user::password::change::action::{
        ChangePasswordAction, OverwritePasswordAction,
    };

    impl ChangePasswordAction<MockChangePasswordInfra> {
        pub fn mock(infra: MockChangePasswordInfra) -> Self {
            Self::new(infra)
        }
    }

    impl OverwritePasswordAction<MockOverwritePasswordInfra> {
        pub fn mock(infra: MockOverwritePasswordInfra) -> Self {
            Self::new(infra)
        }
    }
}
