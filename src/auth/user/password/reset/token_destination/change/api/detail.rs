use std::sync::Arc;

use crate::x_outside_feature::auth::feature::AuthAppFeature;

use crate::{
    auth::{
        feature::AsAuthorizedInfra,
        user::kernel::detail::repository::dynamodb::login_id::{ConnectionLoginId, TableLoginId},
    },
    common::api::{feature::AsInfra, logger::detail::StdoutJsonLogger},
};

use crate::auth::user::password::reset::token_destination::change::action::ChangeResetTokenDestinationInfo;

use crate::auth::user::password::reset::token_destination::change::infra::{
    ChangeResetTokenDestinationInfra, ChangeResetTokenDestinationLogger,
    ChangeResetTokenDestinationRepository,
};

use crate::{
    auth::{
        ticket::{authorize::data::AuthorizeSuccess, kernel::data::AuthPermissionRequired},
        user::{
            login_id::kernel::data::LoginId,
            password::reset::{
                kernel::data::ResetPasswordTokenDestination,
                token_destination::change::data::{
                    ChangeResetTokenDestinationError, ChangeResetTokenDestinationSuccess,
                    ValidateChangeResetTokenDestinationFieldsError,
                },
            },
        },
    },
    common::api::repository::data::RepositoryError,
};

pub struct LiveChangeResetTokenDestinationInfra {
    repository: LiveChangeResetTokenDestinationRepository,
}

impl AsAuthorizedInfra<LiveChangeResetTokenDestinationInfra> for Arc<AuthAppFeature> {
    fn required(&self) -> AuthPermissionRequired {
        ChangeResetTokenDestinationInfo::required()
    }
    fn as_authorized_infra(&self, _: &AuthorizeSuccess) -> LiveChangeResetTokenDestinationInfra {
        LiveChangeResetTokenDestinationInfra {
            repository: LiveChangeResetTokenDestinationRepository {
                login_id: self.as_infra(),
            },
        }
    }
}

impl ChangeResetTokenDestinationInfra for LiveChangeResetTokenDestinationInfra {
    type Repository = LiveChangeResetTokenDestinationRepository;

    fn repository(&self) -> &Self::Repository {
        &self.repository
    }
}

pub struct LiveChangeResetTokenDestinationRepository {
    login_id: ConnectionLoginId,
}

#[async_trait::async_trait]
impl ChangeResetTokenDestinationRepository for LiveChangeResetTokenDestinationRepository {
    async fn lookup_destination(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<ResetPasswordTokenDestination>, RepositoryError> {
        TableLoginId::get_reset_token_destination(&self.login_id, login_id.clone()).await
    }

    async fn change_destination(
        &self,
        login_id: LoginId,
        new_destination: ResetPasswordTokenDestination,
    ) -> Result<(), RepositoryError> {
        TableLoginId::update_reset_token_destination(&self.login_id, login_id, new_destination)
            .await
    }
}

impl ChangeResetTokenDestinationLogger for StdoutJsonLogger {
    fn try_to_change_destination(&self) {
        self.info(format!("try to change reset-token destination"));
    }
    fn invalid_request(
        &self,
        err: ValidateChangeResetTokenDestinationFieldsError,
    ) -> ValidateChangeResetTokenDestinationFieldsError {
        self.fatal(format!(
            "failed to validate change reset-token destination fields; {}",
            err
        ));
        err
    }
    fn failed_to_lookup_destination(&self, err: RepositoryError) -> RepositoryError {
        self.fatal(format!("failed to lookup reset-token destination; {}", err));
        err
    }
    fn user_not_found(
        &self,
        err: ChangeResetTokenDestinationError,
    ) -> ChangeResetTokenDestinationError {
        self.fatal(format!("user not found; {}", err));
        err
    }
    fn conflict(&self, err: ChangeResetTokenDestinationError) -> ChangeResetTokenDestinationError {
        self.info(format!("reset-token destination conflicted; {}", err));
        err
    }
    fn failed_to_change_destination(&self, err: RepositoryError) -> RepositoryError {
        self.fatal(format!("failed to change reset-token destination; {}", err));
        err
    }
    fn succeed_to_change_destination(
        &self,
        auth: ChangeResetTokenDestinationSuccess,
    ) -> ChangeResetTokenDestinationSuccess {
        self.audit(format!("succeed to change reset-token destination"));
        auth
    }
}

#[cfg(test)]
pub mod test {
    use std::sync::Arc;

    use crate::{
        auth::user::kernel::detail::repository::memory::{login_id::MapLoginId, StoreLoginId},
        common::api::feature::AsInfra,
    };

    use crate::auth::user::password::reset::token_destination::change::infra::{
        ChangeResetTokenDestinationInfra, ChangeResetTokenDestinationRepository,
    };

    use crate::{
        auth::user::{
            login_id::kernel::data::LoginId,
            password::reset::kernel::data::ResetPasswordTokenDestination,
        },
        common::api::repository::data::RepositoryError,
    };

    pub struct MockChangeResetTokenDestinationInfra {
        repository: MockChangeResetTokenDestinationRepository,
    }

    impl AsInfra<MockChangeResetTokenDestinationInfra> for Arc<StoreLoginId> {
        fn as_infra(&self) -> MockChangeResetTokenDestinationInfra {
            MockChangeResetTokenDestinationInfra {
                repository: MockChangeResetTokenDestinationRepository {
                    login_id: Arc::clone(&self),
                },
            }
        }
    }

    impl ChangeResetTokenDestinationInfra for MockChangeResetTokenDestinationInfra {
        type Repository = MockChangeResetTokenDestinationRepository;

        fn repository(&self) -> &Self::Repository {
            &self.repository
        }
    }

    pub struct MockChangeResetTokenDestinationRepository {
        login_id: Arc<StoreLoginId>,
    }

    #[async_trait::async_trait]
    impl ChangeResetTokenDestinationRepository for MockChangeResetTokenDestinationRepository {
        async fn lookup_destination(
            &self,
            login_id: &LoginId,
        ) -> Result<Option<ResetPasswordTokenDestination>, RepositoryError> {
            Ok(MapLoginId::get_reset_token_destination(
                &self.login_id,
                login_id,
            ))
        }

        async fn change_destination(
            &self,
            login_id: LoginId,
            new_destination: ResetPasswordTokenDestination,
        ) -> Result<(), RepositoryError> {
            Ok(MapLoginId::update_reset_token_destination(
                &self.login_id,
                login_id,
                new_destination,
            ))
        }
    }
}
