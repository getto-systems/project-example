use std::sync::Arc;

use crate::auth::{
    feature::{AsAuthorizedInfra, AsCheckedInfra},
    ticket::authorize::detail::{LiveAuthorizeInfra, LiveCheckAuthorizeTokenInfra},
};

use crate::auth::ticket::authorize::action::{AuthorizeAction, CheckAuthorizeTokenAction};

use crate::auth::ticket::authorize::infra::{
    AuthorizeInfra, AuthorizeLogger, CheckAuthorizeTokenInfra, CheckAuthorizeTokenLogger,
};

use crate::{
    auth::ticket::{
        authorize::data::{
            AuthorizeError, AuthorizeSuccess, CheckAuthorizeTokenError,
            ValidateAuthorizeFieldsError,
        },
        kernel::data::{
            AuthPermissionError, AuthPermissionGranted, AuthorizeTokenExtract,
            DecodeAuthorizeTokenError, ValidateAuthorizeTokenError,
        },
    },
    common::api::repository::data::RepositoryError,
};

impl<M: AuthorizeInfra> AuthorizeAction<M> {
    pub async fn pick_authorized_infra<T>(
        &self,
        app: &impl AsAuthorizedInfra<T>,
        token: impl AuthorizeTokenExtract,
    ) -> Result<(T, AuthorizeSuccess), AuthorizeError> {
        let auth = self.authorize((token, app.required())).await?;
        Ok((app.as_authorized_infra(&auth), auth))
    }
}

impl<M: CheckAuthorizeTokenInfra> CheckAuthorizeTokenAction<M> {
    pub async fn pick_authorized_infra<T>(
        &self,
        app: &impl AsCheckedInfra<T>,
        token: impl AuthorizeTokenExtract,
    ) -> Result<T, CheckAuthorizeTokenError> {
        let auth = self.check(token, app.required()).await?;
        Ok(app.as_authorized_infra(&auth))
    }

    fn new(infra: M) -> Self {
        Self {
            logger: Arc::new(NoopCheckAuthorizeTokenLogger),
            infra,
        }
    }

    pub fn with_logger(self, logger: Arc<dyn CheckAuthorizeTokenLogger>) -> Self {
        Self { logger, ..self }
    }
}

impl CheckAuthorizeTokenAction<LiveCheckAuthorizeTokenInfra> {
    pub fn live(infra: LiveCheckAuthorizeTokenInfra) -> Self {
        Self::new(infra)
    }
}

struct NoopCheckAuthorizeTokenLogger;

impl CheckAuthorizeTokenLogger for NoopCheckAuthorizeTokenLogger {
    fn try_to_check_authorize_token(&self) {}
    fn invalid_request(&self, err: ValidateAuthorizeTokenError) -> ValidateAuthorizeTokenError {
        err
    }
    fn invalid_token(&self, err: DecodeAuthorizeTokenError) -> DecodeAuthorizeTokenError {
        err
    }
    fn forbidden(&self, err: AuthPermissionError) -> AuthPermissionError {
        err
    }
    fn succeed_to_check_authorize_token(
        &self,
        granted: AuthPermissionGranted,
    ) -> AuthPermissionGranted {
        granted
    }
}

impl<M: AuthorizeInfra> AuthorizeAction<M> {
    fn new(infra: M) -> Self {
        Self {
            logger: Arc::new(NoopAuthorizeLogger),
            infra,
        }
    }

    pub fn with_logger(self, logger: Arc<dyn AuthorizeLogger>) -> Self {
        Self { logger, ..self }
    }
}

impl AuthorizeAction<LiveAuthorizeInfra> {
    pub fn live(infra: LiveAuthorizeInfra) -> Self {
        Self::new(infra)
    }
}

struct NoopAuthorizeLogger;

impl AuthorizeLogger for NoopAuthorizeLogger {
    fn try_to_authorize(&self) {}
    fn invalid_request(&self, err: ValidateAuthorizeFieldsError) -> ValidateAuthorizeFieldsError {
        err
    }
    fn invalid_token(&self, err: DecodeAuthorizeTokenError) -> DecodeAuthorizeTokenError {
        err
    }
    fn forbidden(&self, err: AuthPermissionError) -> AuthPermissionError {
        err
    }
    fn failed_to_lookup_expansion_limit(&self, err: RepositoryError) -> RepositoryError {
        err
    }
    fn failed_to_lookup_permission_granted(&self, err: RepositoryError) -> RepositoryError {
        err
    }
    fn expansion_limit_not_found(&self, err: AuthorizeError) -> AuthorizeError {
        err
    }
    fn ticket_has_expired(&self, err: AuthorizeError) -> AuthorizeError {
        err
    }
    fn permission_granted_not_found(&self, err: AuthorizeError) -> AuthorizeError {
        err
    }
    fn authorized(&self, auth: AuthorizeSuccess) -> AuthorizeSuccess {
        auth
    }
}

#[cfg(test)]
mod test {
    use crate::auth::ticket::authorize::detail::test::{
        MockAuthorizeInfra, MockCheckAuthorizeTokenInfra,
    };

    use crate::auth::ticket::authorize::action::{AuthorizeAction, CheckAuthorizeTokenAction};

    impl CheckAuthorizeTokenAction<MockCheckAuthorizeTokenInfra> {
        pub fn mock(infra: MockCheckAuthorizeTokenInfra) -> Self {
            Self::new(infra)
        }
    }

    impl AuthorizeAction<MockAuthorizeInfra> {
        pub fn mock(infra: MockAuthorizeInfra) -> Self {
            Self::new(infra)
        }
    }
}
