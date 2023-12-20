use std::sync::Arc;

use crate::auth::user::password::reset::request_token::api::detail::LiveRequestResetPasswordTokenInfra;

use crate::auth::user::password::reset::request_token::action::RequestResetPasswordTokenAction;

use crate::auth::user::password::reset::request_token::infra::{
    RequestResetPasswordTokenInfra, RequestResetPasswordTokenLogger,
};

use crate::{
    auth::{
        kernel::data::ExpireDateTime,
        user::{
            login_id::kernel::data::ValidateLoginIdError,
            password::reset::request_token::data::{
                EncodeResetTokenError, NotifyResetTokenError, NotifyResetTokenResponse,
                RequestResetPasswordTokenError,
            },
        },
    },
    common::api::repository::data::RepositoryError,
};

impl<M: RequestResetPasswordTokenInfra> RequestResetPasswordTokenAction<M> {
    fn new(infra: M) -> Self {
        Self {
            logger: Arc::new(NoopLogger),
            infra,
        }
    }

    pub fn with_logger(self, logger: Arc<dyn RequestResetPasswordTokenLogger>) -> Self {
        Self { logger, ..self }
    }
}

impl RequestResetPasswordTokenAction<LiveRequestResetPasswordTokenInfra> {
    pub fn live(infra: LiveRequestResetPasswordTokenInfra) -> Self {
        Self::new(infra)
    }
}

struct NoopLogger;

impl RequestResetPasswordTokenLogger for NoopLogger {
    fn try_to_request_reset_password_token(&self) {}
    fn invalid_request(&self, err: ValidateLoginIdError) -> ValidateLoginIdError {
        err
    }
    fn failed_to_lookup_user(&self, err: RepositoryError) -> RepositoryError {
        err
    }
    fn user_not_found(
        &self,
        err: RequestResetPasswordTokenError,
    ) -> RequestResetPasswordTokenError {
        err
    }
    fn calculate_token_expires(&self, expires: ExpireDateTime) -> ExpireDateTime {
        expires
    }
    fn failed_to_register_reset_token(&self, err: RepositoryError) -> RepositoryError {
        err
    }
    fn failed_to_encode_reset_token(&self, err: EncodeResetTokenError) -> EncodeResetTokenError {
        err
    }
    fn failed_to_notify_reset_token(&self, err: NotifyResetTokenError) -> NotifyResetTokenError {
        err
    }
    fn succeed_to_request_reset_password_token(
        &self,
        response: NotifyResetTokenResponse,
    ) -> NotifyResetTokenResponse {
        response
    }
}

#[cfg(test)]
mod test {
    use crate::auth::user::password::reset::request_token::api::detail::test::MockRequestResetPasswordTokenInfra;

    use crate::auth::user::password::reset::request_token::action::RequestResetPasswordTokenAction;

    impl RequestResetPasswordTokenAction<MockRequestResetPasswordTokenInfra> {
        pub fn mock(infra: MockRequestResetPasswordTokenInfra) -> Self {
            Self::new(infra)
        }
    }
}
