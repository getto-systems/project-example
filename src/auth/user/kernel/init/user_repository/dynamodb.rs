mod login_id;
mod reset_token;
mod user;

use std::{collections::HashMap, convert::TryInto};

use crate::auth::x_outside_feature::feature::AuthOutsideStore;

use crate::auth::user::kernel::init::user_repository::dynamodb::{
    login_id::TableLoginId, reset_token::TableResetToken, user::TableUser,
};

use crate::z_lib::repository::helper::infra_error;

use crate::auth::user::{
    account::{
        modify::infra::ModifyAuthUserAccountRepository,
        search::infra::SearchAuthUserAccountRepository,
    },
    login_id::change::infra::{OverrideLoginIdEntry, OverrideLoginIdRepository},
    password::{
        authenticate::infra::AuthenticatePasswordRepository,
        change::infra::{ChangePasswordRepository, OverridePasswordRepository},
        kernel::infra::HashedPassword,
        reset::{
            request_token::infra::RegisterResetTokenRepository,
            reset::infra::{ResetPasswordRepository, ResetTokenMoment},
            token_destination::change::infra::ChangeResetTokenDestinationRepository,
        },
    },
};

use crate::{
    auth::{
        ticket::kernel::data::{AuthDateTime, ExpireDateTime},
        user::{
            account::{
                kernel::data::AuthUserAccount,
                modify::data::ModifyAuthUserAccountChanges,
                search::data::{
                    AuthUserAccountSearch, SearchAuthUserAccountFilter,
                    SearchAuthUserAccountSortKey,
                },
            },
            kernel::data::{AuthUserId, GrantedAuthRoles},
            login_id::kernel::data::LoginId,
            password::reset::kernel::data::{ResetToken, ResetTokenDestination},
        },
    },
    z_lib::{
        repository::data::RepositoryError,
        search::data::{SearchOffset, SearchPage, SearchSortOrder},
    },
};

pub struct DynamoDbAuthUserRepository<'a> {
    user: TableUser<'a>,
    login_id: TableLoginId<'a>,
    reset_token: TableResetToken<'a>,
}

impl<'a> DynamoDbAuthUserRepository<'a> {
    pub const fn new(feature: &'a AuthOutsideStore) -> Self {
        Self {
            user: TableUser::new(feature),
            login_id: TableLoginId::new(feature),
            reset_token: TableResetToken::new(feature),
        }
    }
}

#[async_trait::async_trait]
impl<'client> OverrideLoginIdRepository for DynamoDbAuthUserRepository<'client> {
    async fn lookup_user<'a>(
        &self,
        login_id: &'a LoginId,
    ) -> Result<Option<OverrideLoginIdEntry>, RepositoryError> {
        self.login_id.lookup_override_entry(login_id.clone()).await
    }

    async fn check_login_id_registered<'a>(
        &self,
        login_id: &'a LoginId,
    ) -> Result<bool, RepositoryError> {
        Ok(self
            .login_id
            .lookup_user_id(login_id.clone())
            .await?
            .is_some())
    }

    async fn override_login_id<'a>(
        &self,
        user: OverrideLoginIdEntry,
        new_login_id: LoginId,
    ) -> Result<(), RepositoryError> {
        // TODO 引数がわかりにくいのをなんとかしたい
        self.user
            .update_login_id(user.user_id.clone(), new_login_id.clone())
            .await?;

        self.login_id.delete_login_id(user.login_id.clone()).await?;
        self.login_id.put_login_id(new_login_id, user).await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl<'client> AuthenticatePasswordRepository for DynamoDbAuthUserRepository<'client> {
    async fn lookup_user_id<'a>(
        &self,
        login_id: &'a LoginId,
    ) -> Result<Option<AuthUserId>, RepositoryError> {
        self.login_id.lookup_user_id(login_id.clone()).await
    }

    async fn lookup_granted_roles<'a>(
        &self,
        user_id: &'a AuthUserId,
    ) -> Result<Option<GrantedAuthRoles>, RepositoryError> {
        self.user.lookup_granted_roles(user_id.clone()).await
    }

    async fn lookup_password<'a>(
        &self,
        user_id: &'a AuthUserId,
    ) -> Result<Option<HashedPassword>, RepositoryError> {
        self.user.lookup_password(user_id.clone()).await
    }
}

#[async_trait::async_trait]
impl<'client> ChangePasswordRepository for DynamoDbAuthUserRepository<'client> {
    async fn lookup_password<'a>(
        &self,
        user_id: &'a AuthUserId,
    ) -> Result<Option<HashedPassword>, RepositoryError> {
        self.user.lookup_password(user_id.clone()).await
    }

    async fn change_password<'a>(
        &self,
        user_id: &'a AuthUserId,
        new_password: HashedPassword,
    ) -> Result<(), RepositoryError> {
        self.user
            .update_password(user_id.clone(), new_password)
            .await
    }
}

#[async_trait::async_trait]
impl<'client> OverridePasswordRepository for DynamoDbAuthUserRepository<'client> {
    async fn lookup_user_id<'a>(
        &self,
        login_id: &'a LoginId,
    ) -> Result<Option<AuthUserId>, RepositoryError> {
        self.login_id.lookup_user_id(login_id.clone()).await
    }

    async fn override_password<'a>(
        &self,
        user_id: &'a AuthUserId,
        new_password: HashedPassword,
    ) -> Result<(), RepositoryError> {
        self.user
            .update_password(user_id.clone(), new_password)
            .await
    }
}

#[async_trait::async_trait]
impl<'client> ModifyAuthUserAccountRepository for DynamoDbAuthUserRepository<'client> {
    async fn lookup_user_id(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<AuthUserId>, RepositoryError> {
        self.login_id.lookup_user_id(login_id.clone()).await
    }

    async fn lookup_changes(
        &self,
        user_id: &AuthUserId,
    ) -> Result<Option<ModifyAuthUserAccountChanges>, RepositoryError> {
        self.user.lookup_modify_changes(user_id.clone()).await
    }

    async fn modify_user(
        &self,
        user_id: &AuthUserId,
        changes: ModifyAuthUserAccountChanges,
    ) -> Result<(), RepositoryError> {
        self.user.update_user(user_id.clone(), changes).await
    }
}

#[async_trait::async_trait]
impl<'client> ChangeResetTokenDestinationRepository for DynamoDbAuthUserRepository<'client> {
    async fn lookup_destination(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<ResetTokenDestination>, RepositoryError> {
        self.login_id
            .lookup_reset_token_entry(login_id.clone())
            .await
            .map(|user| user.map(|(_user_id, destination)| destination))
    }

    async fn change_destination(
        &self,
        login_id: &LoginId,
        new_destination: ResetTokenDestination,
    ) -> Result<(), RepositoryError> {
        self.login_id
            .update_reset_token_destination(login_id.clone(), new_destination)
            .await
    }
}

#[async_trait::async_trait]
impl<'client> RegisterResetTokenRepository for DynamoDbAuthUserRepository<'client> {
    async fn lookup_user(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<(AuthUserId, ResetTokenDestination)>, RepositoryError> {
        self.login_id
            .lookup_reset_token_entry(login_id.clone())
            .await
    }

    async fn register_reset_token(
        &self,
        reset_token: ResetToken,
        user_id: AuthUserId,
        login_id: LoginId,
        destination: ResetTokenDestination,
        expires: ExpireDateTime,
        requested_at: AuthDateTime,
    ) -> Result<(), RepositoryError> {
        self.reset_token
            .put_reset_token(
                reset_token,
                user_id,
                login_id,
                destination,
                expires,
                requested_at,
            )
            .await
    }
}

#[async_trait::async_trait]
impl<'client> ResetPasswordRepository for DynamoDbAuthUserRepository<'client> {
    async fn lookup_reset_token_entry(
        &self,
        reset_token: &ResetToken,
    ) -> Result<
        Option<(AuthUserId, LoginId, ResetTokenDestination, ResetTokenMoment)>,
        RepositoryError,
    > {
        self.reset_token
            .lookup_reset_token_entry(reset_token.clone())
            .await
    }

    async fn lookup_granted_roles<'a>(
        &self,
        user_id: &'a AuthUserId,
    ) -> Result<Option<GrantedAuthRoles>, RepositoryError> {
        self.user.lookup_granted_roles(user_id.clone()).await
    }

    async fn reset_password(
        &self,
        reset_token: &ResetToken,
        user_id: &AuthUserId,
        new_password: HashedPassword,
        reset_at: AuthDateTime,
    ) -> Result<(), RepositoryError> {
        reset_password(self, reset_token, user_id, new_password, reset_at).await
    }
}
async fn reset_password<'client, 'a>(
    repository: &DynamoDbAuthUserRepository<'client>,
    reset_token: &ResetToken,
    user_id: &AuthUserId,
    new_password: HashedPassword,
    reset_at: AuthDateTime,
) -> Result<(), RepositoryError> {
    repository
        .reset_token
        .update_reset_at(reset_token.clone(), reset_at)
        .await?;

    repository
        .user
        .update_password(user_id.clone(), new_password)
        .await?;

    Ok(())
}

#[async_trait::async_trait]
impl<'client> SearchAuthUserAccountRepository for DynamoDbAuthUserRepository<'client> {
    async fn search(
        &self,
        filter: SearchAuthUserAccountFilter,
    ) -> Result<AuthUserAccountSearch, RepositoryError> {
        search_user_account(&self, filter).await
    }
}
async fn search_user_account<'client>(
    repository: &DynamoDbAuthUserRepository<'client>,
    filter: SearchAuthUserAccountFilter,
) -> Result<AuthUserAccountSearch, RepositoryError> {
    // 業務用アプリケーションなので、ユーザー数は 100を超えない
    // dynamodb から全てのデータを取得してフィルタ、ソートする
    let mut users = repository.user.scan_user().await?;
    let mut destinations: HashMap<LoginId, ResetTokenDestination> = repository
        .login_id
        .scan_reset_token_destination()
        .await?
        .into_iter()
        .filter_map(|(login_id, destination)| {
            destination.map(|destination| (login_id, destination))
        })
        .collect();

    let all: i32 = users
        .len()
        .try_into()
        .map_err(|err| infra_error("convert users length error", err))?;

    match filter.sort().key() {
        SearchAuthUserAccountSortKey::LoginId => {
            users.sort_by_cached_key(|(login_id, _)| login_id.as_str().to_owned());
            match filter.sort().order() {
                SearchSortOrder::Normal => (),
                SearchSortOrder::Reverse => users.reverse(),
            }
        }
    };

    let mut users: Vec<AuthUserAccount> = users
        .into_iter()
        .filter(|(login_id, _)| match filter.login_id() {
            None => true,
            Some(filter_login_id) => login_id.as_str() == filter_login_id,
        })
        .map(|(login_id, granted_roles)| {
            let destination = destinations.remove(&login_id);
            AuthUserAccount {
                login_id,
                // TODO これは多分ドメイン知識
                granted_roles: granted_roles.unwrap_or(GrantedAuthRoles::empty()),
                reset_token_destination: destination.unwrap_or(ResetTokenDestination::None),
            }
        })
        .collect();

    let limit = 1000;
    let offset = SearchOffset { all, limit }.detect(filter.offset());
    let mut users = users.split_off(
        offset
            .try_into()
            .map_err(|err| infra_error("convert offset error", err))?,
    );
    users.truncate(
        limit
            .try_into()
            .map_err(|err| infra_error("convert limit error", err))?,
    );

    Ok(AuthUserAccountSearch {
        page: SearchPage { all, limit, offset },
        sort: filter.into_sort(),
        users,
    })
}
