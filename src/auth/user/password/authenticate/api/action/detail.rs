use std::sync::Arc;

use crate::auth::user::password::authenticate::api::detail::LiveAuthenticateWithPasswordInfra;

use crate::auth::user::password::authenticate::action::AuthenticateWithPasswordAction;

use crate::auth::user::password::authenticate::infra::{
    AuthenticateWithPasswordInfra, AuthenticateWithPasswordLogger,
};

use crate::{
    auth::{
        ticket::kernel::data::AuthenticateSuccess,
        user::password::{
            authenticate::data::{
                AuthenticateWithPasswordError, ValidateAuthenticateWithPasswordFieldsError,
            },
            kernel::data::PasswordHashError,
        },
    },
    common::api::repository::data::RepositoryError,
};

impl<M: AuthenticateWithPasswordInfra> AuthenticateWithPasswordAction<M> {
    fn new(infra: M) -> Self {
        Self {
            logger: Arc::new(NoopLogger),
            infra,
        }
    }

    pub fn with_logger(self, logger: Arc<dyn AuthenticateWithPasswordLogger>) -> Self {
        Self { logger, ..self }
    }
}

impl AuthenticateWithPasswordAction<LiveAuthenticateWithPasswordInfra> {
    pub fn live(infra: LiveAuthenticateWithPasswordInfra) -> Self {
        Self::new(infra)
    }
}

struct NoopLogger;

impl AuthenticateWithPasswordLogger for NoopLogger {
    fn try_to_authenticate_with_password(&self) {}
    fn invalid_request(
        &self,
        err: ValidateAuthenticateWithPasswordFieldsError,
    ) -> ValidateAuthenticateWithPasswordFieldsError {
        err
    }
    fn failed_to_lookup_user(&self, err: RepositoryError) -> RepositoryError {
        err
    }
    fn failed_to_lookup_password_and_granted(&self, err: RepositoryError) -> RepositoryError {
        err
    }
    fn user_not_found(&self, err: AuthenticateWithPasswordError) -> AuthenticateWithPasswordError {
        err
    }
    fn failed_to_match_password(&self, err: PasswordHashError) -> PasswordHashError {
        err
    }
    fn password_not_matched(
        &self,
        err: AuthenticateWithPasswordError,
    ) -> AuthenticateWithPasswordError {
        err
    }
    fn authenticated(&self, auth: AuthenticateSuccess) -> AuthenticateSuccess {
        auth
    }
}

#[cfg(test)]
mod test {
    use crate::auth::user::password::authenticate::api::detail::test::MockAuthenticateWithPasswordInfra;

    use crate::auth::user::password::authenticate::action::AuthenticateWithPasswordAction;

    impl AuthenticateWithPasswordAction<MockAuthenticateWithPasswordInfra> {
        pub fn mock(infra: MockAuthenticateWithPasswordInfra) -> Self {
            Self::new(infra)
        }
    }
}
