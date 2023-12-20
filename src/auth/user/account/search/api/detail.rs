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

use crate::auth::user::account::search::action::SearchAuthUserAccountInfo;

use crate::auth::user::account::search::infra::{
    SearchAuthUserAccountInfra, SearchAuthUserAccountLogger, SearchAuthUserAccountRepository,
};

use crate::{
    auth::{
        ticket::{
            authorize::data::AuthorizeSuccess,
            kernel::data::{AuthPermissionGranted, AuthPermissionRequired},
        },
        user::{
            account::{kernel::data::AuthUserMemo, search::data::AuthUserAccountSearch},
            login_id::kernel::data::LoginId,
            password::reset::kernel::data::ResetPasswordTokenDestination,
        },
    },
    common::api::repository::data::RepositoryError,
};

pub struct LiveSearchAuthUserAccountInfra {
    repository: LiveSearchAuthUserAccountRepository,
}

impl AsAuthorizedInfra<LiveSearchAuthUserAccountInfra> for Arc<AuthAppFeature> {
    fn required(&self) -> AuthPermissionRequired {
        SearchAuthUserAccountInfo::required()
    }
    fn as_authorized_infra(&self, _: &AuthorizeSuccess) -> LiveSearchAuthUserAccountInfra {
        LiveSearchAuthUserAccountInfra {
            repository: LiveSearchAuthUserAccountRepository {
                login_id: self.as_infra(),
                user: self.as_infra(),
            },
        }
    }
}

impl SearchAuthUserAccountInfra for LiveSearchAuthUserAccountInfra {
    type Repository = LiveSearchAuthUserAccountRepository;

    fn repository(&self) -> &Self::Repository {
        &self.repository
    }
}

pub struct LiveSearchAuthUserAccountRepository {
    login_id: ConnectionLoginId,
    user: ConnectionUser,
}

#[async_trait::async_trait]
impl SearchAuthUserAccountRepository for LiveSearchAuthUserAccountRepository {
    async fn find_all_reset_token_destination(
        &self,
    ) -> Result<Vec<(LoginId, ResetPasswordTokenDestination)>, RepositoryError> {
        TableLoginId::scan_reset_token_destination(&self.login_id).await
    }
    async fn find_all_user(
        &self,
    ) -> Result<Vec<(LoginId, Option<AuthPermissionGranted>, Option<AuthUserMemo>)>, RepositoryError>
    {
        TableUser::scan_user(&self.user).await
    }
}

impl SearchAuthUserAccountLogger for StdoutJsonLogger {
    fn try_to_search_auth_user_account(&self) {
        self.info(format!("try to search auth-user-account"));
    }
    fn failed_to_search_user_account(&self, err: RepositoryError) -> RepositoryError {
        self.fatal(format!("failed to search auth-user-account; {}", err));
        err
    }
    fn succeed_to_search_auth_user_account(
        &self,
        success: AuthUserAccountSearch,
    ) -> AuthUserAccountSearch {
        self.info(format!("succeed to search auth-user-account"));
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

    use crate::auth::user::account::search::infra::{
        SearchAuthUserAccountInfra, SearchAuthUserAccountRepository,
    };

    use crate::{
        auth::{
            ticket::kernel::data::AuthPermissionGranted,
            user::{
                account::kernel::data::AuthUserMemo, login_id::kernel::data::LoginId,
                password::reset::kernel::data::ResetPasswordTokenDestination,
            },
        },
        common::api::repository::data::RepositoryError,
    };

    pub struct MockSearchAuthUserAccountInfra {
        repository: MockModifyAuthUserAccountRepository,
    }

    impl AsInfra<MockSearchAuthUserAccountInfra> for (Arc<StoreLoginId>, Arc<StoreUser>) {
        fn as_infra(&self) -> MockSearchAuthUserAccountInfra {
            MockSearchAuthUserAccountInfra {
                repository: MockModifyAuthUserAccountRepository {
                    login_id: Arc::clone(&self.0),
                    user: Arc::clone(&self.1),
                },
            }
        }
    }

    impl<'a> SearchAuthUserAccountInfra for MockSearchAuthUserAccountInfra {
        type Repository = MockModifyAuthUserAccountRepository;

        fn repository(&self) -> &Self::Repository {
            &self.repository
        }
    }

    pub struct MockModifyAuthUserAccountRepository {
        login_id: Arc<StoreLoginId>,
        user: Arc<StoreUser>,
    }

    #[async_trait::async_trait]
    impl SearchAuthUserAccountRepository for MockModifyAuthUserAccountRepository {
        async fn find_all_reset_token_destination(
            &self,
        ) -> Result<Vec<(LoginId, ResetPasswordTokenDestination)>, RepositoryError> {
            Ok(MapLoginId::find_all(&self.login_id))
        }
        async fn find_all_user(
            &self,
        ) -> Result<
            Vec<(LoginId, Option<AuthPermissionGranted>, Option<AuthUserMemo>)>,
            RepositoryError,
        > {
            Ok(MapUser::find_all(&self.user))
        }
    }
}
