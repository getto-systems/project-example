use std::sync::Arc;

use crate::auth::user::password::reset::reset::api::detail::LiveResetPasswordInfra;

use crate::auth::user::password::reset::reset::action::ResetPasswordAction;

use crate::auth::user::password::reset::reset::infra::{ResetPasswordInfra, ResetPasswordLogger};

use crate::{
    auth::{
        ticket::kernel::data::AuthenticateSuccess,
        user::password::{
            kernel::data::PasswordHashError,
            reset::reset::data::{
                DecodeResetTokenError, NotifyResetPasswordError, NotifyResetPasswordResponse,
                ResetPasswordError, ValidateResetPasswordFieldsError,
            },
        },
    },
    common::api::repository::data::RepositoryError,
};

impl<M: ResetPasswordInfra> ResetPasswordAction<M> {
    fn new(infra: M) -> Self {
        Self {
            logger: Arc::new(NoopLogger),
            infra,
        }
    }

    pub fn with_logger(self, logger: Arc<dyn ResetPasswordLogger>) -> Self {
        Self { logger, ..self }
    }
}

impl ResetPasswordAction<LiveResetPasswordInfra> {
    pub fn live(infra: LiveResetPasswordInfra) -> Self {
        Self::new(infra)
    }
}

struct NoopLogger;

impl ResetPasswordLogger for NoopLogger {
    fn try_to_reset_password(&self) {}
    fn invalid_request(
        &self,
        err: ValidateResetPasswordFieldsError,
    ) -> ValidateResetPasswordFieldsError {
        err
    }
    fn failed_to_decode_token(&self, err: DecodeResetTokenError) -> DecodeResetTokenError {
        err
    }
    fn failed_to_lookup_reset_token_entry(&self, err: RepositoryError) -> RepositoryError {
        err
    }
    fn reset_token_not_found(&self, err: ResetPasswordError) -> ResetPasswordError {
        err
    }
    fn failed_to_lookup_permission_granted(&self, err: RepositoryError) -> RepositoryError {
        err
    }
    fn already_reset(&self, err: ResetPasswordError) -> ResetPasswordError {
        err
    }
    fn expired(&self, err: ResetPasswordError) -> ResetPasswordError {
        err
    }
    fn failed_to_hash_password(&self, err: PasswordHashError) -> PasswordHashError {
        err
    }
    fn failed_to_consume_reset_id(&self, err: RepositoryError) -> RepositoryError {
        err
    }
    fn failed_to_update_password(&self, err: RepositoryError) -> RepositoryError {
        err
    }
    fn failed_to_notify(&self, err: NotifyResetPasswordError) -> NotifyResetPasswordError {
        err
    }
    fn succeed_to_notify(
        &self,
        response: NotifyResetPasswordResponse,
    ) -> NotifyResetPasswordResponse {
        response
    }
    fn succeed_to_reset_password(&self, auth: AuthenticateSuccess) -> AuthenticateSuccess {
        auth
    }
}

#[cfg(test)]
mod test {
    use crate::auth::user::password::reset::reset::api::detail::test::MockResetPasswordInfra;

    use crate::auth::user::password::reset::reset::action::ResetPasswordAction;

    impl ResetPasswordAction<MockResetPasswordInfra> {
        pub fn mock(infra: MockResetPasswordInfra) -> Self {
            Self::new(infra)
        }
    }
}
