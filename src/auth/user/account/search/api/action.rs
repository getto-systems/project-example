mod detail;

use std::{collections::HashMap, sync::Arc};

use crate::auth::user::account::search::infra::{
    SearchAuthUserAccountFilterExtract, SearchAuthUserAccountInfra, SearchAuthUserAccountLogger,
    SearchAuthUserAccountRepository,
};

use crate::{
    auth::{
        ticket::kernel::data::AuthPermissionRequired,
        user::{
            account::{
                kernel::data::{AuthUserAccount, AuthUserAccountAttrs},
                search::data::{AuthUserAccountSearch, SearchAuthUserAccountSortKey},
            },
            login_id::kernel::data::LoginId,
            password::reset::kernel::data::ResetPasswordTokenDestination,
        },
    },
    common::api::{
        repository::data::RepositoryError,
        search::data::{Search, SearchLimit, SearchSorterNormal},
    },
};

pub struct SearchAuthUserAccountAction<M: SearchAuthUserAccountInfra> {
    infra: M,
    logger: Arc<dyn SearchAuthUserAccountLogger>,
}

pub struct SearchAuthUserAccountInfo;

impl SearchAuthUserAccountInfo {
    pub fn required() -> AuthPermissionRequired {
        AuthPermissionRequired::user()
    }
}

impl<M: SearchAuthUserAccountInfra> SearchAuthUserAccountAction<M> {
    pub async fn search(
        &self,
        filter: impl SearchAuthUserAccountFilterExtract,
    ) -> Result<AuthUserAccountSearch, RepositoryError> {
        self.logger.try_to_search_auth_user_account();

        let filter = filter.convert();

        // 業務用アプリケーションなので、ユーザー数はたかだか 1000 程度
        // 全てのデータを取得してフィルタ、ソートする
        let mut destinations: HashMap<LoginId, ResetPasswordTokenDestination> = self
            .infra
            .repository()
            .find_all_reset_token_destination()
            .await?
            .into_iter()
            .collect();

        let users: Vec<AuthUserAccount> = self
            .infra
            .repository()
            .find_all_user()
            .await?
            .into_iter()
            .map(|(login_id, granted, memo)| {
                let destination = destinations.remove(&login_id);
                AuthUserAccount {
                    login_id,
                    attrs: AuthUserAccountAttrs {
                        granted: granted.unwrap_or_default(),
                        memo: memo.unwrap_or_default(),
                    },
                    reset_token_destination: destination.unwrap_or_default(),
                }
            })
            .collect();

        let (users, page) = users.search(
            filter.search,
            SearchLimit::default(),
            |model| filter.props.is_match(model),
            |key| match key {
                SearchAuthUserAccountSortKey::LoginId => (
                    SearchSorterNormal,
                    Box::new(|model| model.login_id.clone().extract()),
                ),
            },
        )?;

        Ok(self
            .logger
            .succeed_to_search_auth_user_account(AuthUserAccountSearch {
                page,
                sort: filter.search.sort,
                users,
            }))
    }
}
