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
        modify::infra::{ModifyAuthUserAccountChanges, ModifyAuthUserAccountRepository},
        register::infra::{RegisterAuthUserAccountFields, RegisterAuthUserAccountRepository},
        search::infra::SearchAuthUserAccountRepository,
        unregister::infra::UnregisterAuthUserAccountRepository,
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
        search::data::{detect_search_page, SearchOffsetDetecterExtract, SearchSortOrder},
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
impl<'a> AuthenticatePasswordRepository for DynamoDbAuthUserRepository<'a> {
    async fn lookup_user_id(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<AuthUserId>, RepositoryError> {
        self.login_id.get_user_id(login_id.clone()).await
    }

    async fn lookup_user(
        &self,
        user_id: &AuthUserId,
    ) -> Result<Option<(HashedPassword, Option<GrantedAuthRoles>)>, RepositoryError> {
        self.user
            .get_password_and_granted_roles(user_id.clone())
            .await
    }
}

#[async_trait::async_trait]
impl<'a> OverrideLoginIdRepository for DynamoDbAuthUserRepository<'a> {
    async fn lookup_user(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<OverrideLoginIdEntry>, RepositoryError> {
        self.login_id.get_override_entry(login_id.clone()).await
    }

    async fn check_login_id_registered(&self, login_id: &LoginId) -> Result<bool, RepositoryError> {
        self.login_id
            .check_login_id_registered(login_id.clone())
            .await
    }

    async fn override_login_id(
        &self,
        new_login_id: LoginId,
        user: OverrideLoginIdEntry,
    ) -> Result<(), RepositoryError> {
        self.user
            .update_login_id(user.user_id.clone(), new_login_id.clone())
            .await?;

        self.login_id.delete_entry(user.login_id.clone()).await?;
        self.login_id.put_override_entry(new_login_id, user).await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl<'a> ChangePasswordRepository for DynamoDbAuthUserRepository<'a> {
    async fn lookup_password(
        &self,
        user_id: &AuthUserId,
    ) -> Result<Option<HashedPassword>, RepositoryError> {
        self.user.get_password(user_id.clone()).await
    }

    async fn change_password(
        &self,
        user_id: AuthUserId,
        new_password: HashedPassword,
    ) -> Result<(), RepositoryError> {
        self.user.update_password(user_id, new_password).await
    }
}

#[async_trait::async_trait]
impl<'a> OverridePasswordRepository for DynamoDbAuthUserRepository<'a> {
    async fn lookup_user_id(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<AuthUserId>, RepositoryError> {
        self.login_id.get_user_id(login_id.clone()).await
    }

    async fn override_password(
        &self,
        user_id: AuthUserId,
        new_password: HashedPassword,
    ) -> Result<(), RepositoryError> {
        self.user.update_password(user_id, new_password).await
    }
}

#[async_trait::async_trait]
impl<'a> RegisterAuthUserAccountRepository for DynamoDbAuthUserRepository<'a> {
    async fn check_login_id_registered(&self, login_id: &LoginId) -> Result<bool, RepositoryError> {
        self.login_id
            .check_login_id_registered(login_id.clone())
            .await
    }

    async fn register_user(
        &self,
        user_id: AuthUserId,
        fields: RegisterAuthUserAccountFields,
    ) -> Result<(), RepositoryError> {
        self.user
            .put_new_entry(
                user_id.clone(),
                fields.login_id.clone(),
                fields.granted_roles,
                fields.attrs,
            )
            .await?;

        {
            // login-id が衝突した場合に rollback する
            let result = self
                .login_id
                .put_new_entry(
                    fields.login_id,
                    user_id.clone(),
                    fields.reset_token_destination,
                )
                .await;
            if result.is_err() {
                self.user.delete_entry(user_id.clone()).await?;
            }
            result?;
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl<'a> ModifyAuthUserAccountRepository for DynamoDbAuthUserRepository<'a> {
    async fn lookup_user_id(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<AuthUserId>, RepositoryError> {
        self.login_id.get_user_id(login_id.clone()).await
    }

    async fn lookup_changes(
        &self,
        user_id: &AuthUserId,
    ) -> Result<Option<ModifyAuthUserAccountChanges>, RepositoryError> {
        self.user.get_modify_changes(user_id.clone()).await
    }

    async fn modify_user(
        &self,
        user_id: AuthUserId,
        changes: ModifyAuthUserAccountChanges,
    ) -> Result<(), RepositoryError> {
        self.user.update_user(user_id, changes).await
    }
}

#[async_trait::async_trait]
impl<'a> UnregisterAuthUserAccountRepository for DynamoDbAuthUserRepository<'a> {
    async fn lookup_user_id(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<AuthUserId>, RepositoryError> {
        self.login_id.get_user_id(login_id.clone()).await
    }

    async fn unregister_user(
        &self,
        user_id: &AuthUserId,
        login_id: &LoginId,
    ) -> Result<(), RepositoryError> {
        let entry = self.user.get_entry(user_id.clone()).await?;
        self.user.delete_entry(user_id.clone()).await?;

        {
            let result = self.login_id.delete_entry(login_id.clone()).await;
            if result.is_err() {
                if let Some(entry) = entry {
                    self.user.put_entry(user_id.clone(), entry).await?;
                }
            }
            result?;
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl<'a> ChangeResetTokenDestinationRepository for DynamoDbAuthUserRepository<'a> {
    async fn lookup_destination(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<ResetTokenDestination>, RepositoryError> {
        self.login_id
            .get_reset_token_destination(login_id.clone())
            .await
    }

    async fn change_destination(
        &self,
        login_id: LoginId,
        new_destination: ResetTokenDestination,
    ) -> Result<(), RepositoryError> {
        self.login_id
            .update_reset_token_destination(login_id, new_destination)
            .await
    }
}

#[async_trait::async_trait]
impl<'a> RegisterResetTokenRepository for DynamoDbAuthUserRepository<'a> {
    async fn lookup_user(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<(AuthUserId, Option<ResetTokenDestination>)>, RepositoryError> {
        self.login_id.get_reset_token_entry(login_id.clone()).await
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
impl<'a> ResetPasswordRepository for DynamoDbAuthUserRepository<'a> {
    async fn lookup_reset_token_entry(
        &self,
        reset_token: &ResetToken,
    ) -> Result<
        Option<(AuthUserId, LoginId, ResetTokenDestination, ResetTokenMoment)>,
        RepositoryError,
    > {
        self.reset_token
            .get_reset_token_entry(reset_token.clone())
            .await
    }

    async fn lookup_granted_roles(
        &self,
        user_id: &AuthUserId,
    ) -> Result<Option<Option<GrantedAuthRoles>>, RepositoryError> {
        self.user.get_granted_roles(user_id.clone()).await
    }

    async fn reset_password(
        &self,
        user_id: AuthUserId,
        reset_token: ResetToken,
        new_password: HashedPassword,
        reset_at: AuthDateTime,
    ) -> Result<(), RepositoryError> {
        reset_password(self, user_id, reset_token, new_password, reset_at).await
    }
}
async fn reset_password<'a>(
    repository: &DynamoDbAuthUserRepository<'a>,
    user_id: AuthUserId,
    reset_token: ResetToken,
    new_password: HashedPassword,
    reset_at: AuthDateTime,
) -> Result<(), RepositoryError> {
    repository
        .reset_token
        .update_reset_at(reset_token, reset_at)
        .await?;

    repository
        .user
        .update_password(user_id, new_password)
        .await?;

    Ok(())
}

#[async_trait::async_trait]
impl<'a> SearchAuthUserAccountRepository for DynamoDbAuthUserRepository<'a> {
    async fn search(
        &self,
        filter: SearchAuthUserAccountFilter,
    ) -> Result<AuthUserAccountSearch, RepositoryError> {
        search_user_account(&self, filter).await
    }
}
async fn search_user_account<'a>(
    repository: &DynamoDbAuthUserRepository<'a>,
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

    match filter.sort().key() {
        SearchAuthUserAccountSortKey::LoginId => {
            users.sort_by_cached_key(|(login_id, _, _)| login_id.clone());
            match filter.sort().order() {
                SearchSortOrder::Normal => (),
                SearchSortOrder::Reverse => users.reverse(),
            }
        }
    };

    let mut users: Vec<AuthUserAccount> = users
        .into_iter()
        .filter(|(login_id, granted_roles, _)| {
            filter.match_login_id(login_id) && filter.match_granted_roles(granted_roles)
        })
        .map(|(login_id, granted_roles, attrs)| {
            let destination = destinations.remove(&login_id);
            AuthUserAccount {
                login_id,
                granted_roles: granted_roles.unwrap_or(GrantedAuthRoles::empty()),
                reset_token_destination: destination.unwrap_or(ResetTokenDestination::None),
                attrs,
            }
        })
        .collect();

    let detecter = SearchOffsetDetecterExtract {
        all: users.len(),
        limit: 1000,
    };
    let page = detect_search_page(
        detecter
            .try_into()
            .map_err(|err| infra_error("convert offset error", err))?,
        filter.offset(),
    );

    let mut users = users.split_off(
        page.offset
            .try_into()
            .map_err(|err| infra_error("convert offset error", err))?,
    );
    users.truncate(detecter.limit);

    Ok(AuthUserAccountSearch {
        page,
        sort: filter.into_sort(),
        users,
    })
}
