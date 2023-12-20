use crate::{
    auth::{
        ticket::kernel::data::AuthPermissionGranted,
        user::{
            account::{
                kernel::data::AuthUserMemo,
                search::data::{AuthUserAccountSearch, SearchAuthUserAccountFilter},
            },
            login_id::kernel::data::LoginId,
            password::reset::kernel::data::ResetPasswordTokenDestination,
        },
    },
    common::api::repository::data::RepositoryError,
};

pub trait SearchAuthUserAccountFilterExtract {
    fn convert(self) -> SearchAuthUserAccountFilter;
}

impl SearchAuthUserAccountFilterExtract for SearchAuthUserAccountFilter {
    fn convert(self) -> SearchAuthUserAccountFilter {
        self
    }
}

pub trait SearchAuthUserAccountInfra {
    type Repository: SearchAuthUserAccountRepository;

    fn repository(&self) -> &Self::Repository;
}

#[async_trait::async_trait]
pub trait SearchAuthUserAccountRepository {
    async fn find_all_reset_token_destination(
        &self,
    ) -> Result<Vec<(LoginId, ResetPasswordTokenDestination)>, RepositoryError>;
    async fn find_all_user(
        &self,
    ) -> Result<Vec<(LoginId, Option<AuthPermissionGranted>, Option<AuthUserMemo>)>, RepositoryError>;
}

pub trait SearchAuthUserAccountLogger: Send + Sync {
    fn try_to_search_auth_user_account(&self);
    fn failed_to_search_user_account(&self, err: RepositoryError) -> RepositoryError;
    fn succeed_to_search_auth_user_account(
        &self,
        success: AuthUserAccountSearch,
    ) -> AuthUserAccountSearch;
}
