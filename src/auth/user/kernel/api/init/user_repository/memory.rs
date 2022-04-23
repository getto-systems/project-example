mod login_id;
mod reset_token;
mod user;

use std::collections::HashMap;

use crate::auth::user::{
    account::{
        register::infra::RegisterAuthUserAccountFields,
        unregister::infra::UnregisterAuthUserAccountRepository,
    },
    kernel::init::user_repository::memory::{
        login_id::{EntryLoginId, MapLoginId, StoreLoginId},
        reset_token::{EntryResetToken, MapResetToken, StoreResetToken},
        user::{EntryUser, MapUser, StoreUser},
    },
};

use crate::z_lib::repository::helper::infra_error;

use crate::auth::user::{
    account::{
        modify::infra::ModifyAuthUserAccountRepository,
        register::infra::RegisterAuthUserAccountRepository,
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
            kernel::data::{AuthUser, AuthUserId, GrantedAuthRoles},
            login_id::kernel::data::LoginId,
            password::reset::kernel::data::{ResetToken, ResetTokenDestination},
        },
    },
    z_lib::{
        repository::data::RepositoryError,
        search::data::{detect_search_page, SearchOffsetDetecterExtract, SearchSortOrder},
    },
};

pub struct MemoryAuthUserRepository<'a> {
    user: MapUser<'a>,
    login_id: MapLoginId<'a>,
    reset_token: MapResetToken<'a>,
}

pub struct MemoryAuthUserStore {
    user: StoreUser,
    login_id: StoreLoginId,
    reset_token: StoreResetToken,
}

impl MemoryAuthUserStore {
    pub fn new() -> Self {
        Self {
            user: MapUser::new_store(),
            login_id: MapLoginId::new_store(),
            reset_token: MapResetToken::new_store(),
        }
    }
}

impl<'a> MemoryAuthUserRepository<'a> {
    pub fn new(store: &'a MemoryAuthUserStore) -> Self {
        Self {
            user: MapUser::new(&store.user),
            login_id: MapLoginId::new(&store.login_id),
            reset_token: MapResetToken::new(&store.reset_token),
        }
    }

    pub fn with_user_id(
        store: &'a MemoryAuthUserStore,
        login_id: LoginId,
        user_id: AuthUserId,
    ) -> Self {
        let repository = Self::new(store);
        repository.insert_user(login_id, user_id);
        repository
    }
    pub fn with_user_and_password(
        store: &'a MemoryAuthUserStore,
        login_id: LoginId,
        user: AuthUser,
        password: HashedPassword,
        users: Vec<(LoginId, AuthUserId)>,
    ) -> Self {
        let repository = Self::new(store);

        let user = user.extract();
        let user_id = AuthUserId::restore(user.user_id);
        let granted_roles = GrantedAuthRoles::restore(user.granted_roles);

        repository.login_id.insert_entry(
            login_id.clone(),
            EntryLoginId {
                user_id: user_id.clone(),
                reset_token_destination: None,
            },
        );
        repository.user.insert_entry(
            user_id,
            EntryUser {
                login_id,
                granted_roles: Some(granted_roles),
                password: Some(password),
            },
        );

        for (login_id, user_id) in users {
            repository.insert_user(login_id, user_id);
        }

        repository
    }
    pub fn with_user_id_and_destination(
        store: &'a MemoryAuthUserStore,
        login_id: LoginId,
        user_id: AuthUserId,
        destination: ResetTokenDestination,
    ) -> Self {
        let repository = Self::new(store);

        repository.login_id.insert_entry(
            login_id.clone(),
            EntryLoginId {
                user_id: user_id.clone(),
                reset_token_destination: Some(destination),
            },
        );
        repository.user.insert_entry(
            user_id,
            EntryUser {
                login_id,
                granted_roles: None,
                password: None,
            },
        );

        repository
    }
    pub fn with_user_and_reset_token(
        store: &'a MemoryAuthUserStore,
        login_id: LoginId,
        user: AuthUser,
        reset_token: ResetToken,
        destination: ResetTokenDestination,
        expires: ExpireDateTime,
        requested_at: AuthDateTime,
        reset_at: Option<AuthDateTime>,
    ) -> Self {
        let repository = Self::new(store);

        let user = user.extract();
        let user_id = AuthUserId::restore(user.user_id);
        let granted_roles = GrantedAuthRoles::restore(user.granted_roles);

        repository.login_id.insert_entry(
            login_id.clone(),
            EntryLoginId {
                user_id: user_id.clone(),
                reset_token_destination: Some(destination.clone()),
            },
        );
        repository.user.insert_entry(
            user_id.clone(),
            EntryUser {
                login_id: login_id.clone(),
                granted_roles: Some(granted_roles),
                password: None,
            },
        );
        repository.reset_token.insert_entry(
            reset_token,
            EntryResetToken {
                user_id,
                login_id,
                destination,
                expires,
                requested_at,
                reset_at,
            },
        );

        repository
    }

    fn insert_user(&self, login_id: LoginId, user_id: AuthUserId) {
        self.login_id.insert_entry(
            login_id.clone(),
            EntryLoginId {
                user_id: user_id.clone(),
                reset_token_destination: None,
            },
        );
        self.user.insert_entry(
            user_id,
            EntryUser {
                login_id,
                granted_roles: None,
                password: None,
            },
        );
    }
}

#[async_trait::async_trait]
impl<'a> AuthenticatePasswordRepository for MemoryAuthUserRepository<'a> {
    async fn lookup_user_id(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<AuthUserId>, RepositoryError> {
        Ok(self.login_id.get_user_id(login_id))
    }

    async fn lookup_user(
        &self,
        user_id: &AuthUserId,
    ) -> Result<Option<(HashedPassword, Option<GrantedAuthRoles>)>, RepositoryError> {
        Ok(self.user.get_password_and_granted_roles(user_id))
    }
}

#[async_trait::async_trait]
impl<'a> OverrideLoginIdRepository for MemoryAuthUserRepository<'a> {
    async fn lookup_user(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<OverrideLoginIdEntry>, RepositoryError> {
        Ok(self.login_id.get_override_entry(login_id))
    }

    async fn check_login_id_registered(&self, login_id: &LoginId) -> Result<bool, RepositoryError> {
        Ok(self.login_id.get_user_id(login_id).is_some())
    }

    async fn override_login_id(
        &self,
        new_login_id: LoginId,
        user: OverrideLoginIdEntry,
    ) -> Result<(), RepositoryError> {
        override_login_id(self, new_login_id, user)
    }
}
fn override_login_id<'a>(
    repository: &MemoryAuthUserRepository<'a>,
    new_login_id: LoginId,
    user: OverrideLoginIdEntry,
) -> Result<(), RepositoryError> {
    repository
        .user
        .update_login_id(user.user_id.clone(), new_login_id.clone());

    repository.login_id.remove_entry(&user.login_id);
    repository
        .login_id
        .insert_override_entry(new_login_id, user);

    Ok(())
}

#[async_trait::async_trait]
impl<'a> ChangePasswordRepository for MemoryAuthUserRepository<'a> {
    async fn lookup_password(
        &self,
        user_id: &AuthUserId,
    ) -> Result<Option<HashedPassword>, RepositoryError> {
        Ok(self.user.get_password(user_id))
    }

    async fn change_password(
        &self,
        user_id: AuthUserId,
        new_password: HashedPassword,
    ) -> Result<(), RepositoryError> {
        Ok(self.user.update_password(user_id, new_password))
    }
}

#[async_trait::async_trait]
impl<'a> OverridePasswordRepository for MemoryAuthUserRepository<'a> {
    async fn lookup_user_id(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<AuthUserId>, RepositoryError> {
        Ok(self.login_id.get_user_id(login_id))
    }

    async fn override_password(
        &self,
        user_id: AuthUserId,
        new_password: HashedPassword,
    ) -> Result<(), RepositoryError> {
        Ok(self.user.update_password(user_id, new_password))
    }
}

#[async_trait::async_trait]
impl<'a> RegisterAuthUserAccountRepository for MemoryAuthUserRepository<'a> {
    async fn check_login_id_registered(&self, login_id: &LoginId) -> Result<bool, RepositoryError> {
        Ok(self.login_id.get_user_id(login_id).is_some())
    }

    async fn register_user(
        &self,
        user_id: AuthUserId,
        fields: RegisterAuthUserAccountFields,
    ) -> Result<(), RepositoryError> {
        self.user.insert_entry(
            user_id.clone(),
            EntryUser {
                login_id: fields.login_id.clone(),
                granted_roles: Some(fields.granted_roles),
                password: None,
            },
        );
        self.login_id.insert_entry(
            fields.login_id,
            EntryLoginId {
                user_id,
                reset_token_destination: Some(fields.reset_token_destination),
            },
        );

        Ok(())
    }
}

#[async_trait::async_trait]
impl<'a> UnregisterAuthUserAccountRepository for MemoryAuthUserRepository<'a> {
    async fn unregister_user(&self, login_id: &LoginId) -> Result<(), RepositoryError> {
        if let Some(user_id) = self.login_id.get_user_id(login_id) {
            self.user.remove_entry(&user_id);
            self.login_id.remove_entry(login_id);

            // TODO ticket repository と統合してここで user_id に紐づく ticket を全て削除する
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl<'a> ModifyAuthUserAccountRepository for MemoryAuthUserRepository<'a> {
    async fn lookup_user_id(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<AuthUserId>, RepositoryError> {
        Ok(self.login_id.get_user_id(login_id))
    }

    async fn lookup_changes(
        &self,
        user_id: &AuthUserId,
    ) -> Result<Option<ModifyAuthUserAccountChanges>, RepositoryError> {
        Ok(self.user.get_modify_changes(user_id))
    }

    async fn modify_user(
        &self,
        user_id: AuthUserId,
        changes: ModifyAuthUserAccountChanges,
    ) -> Result<(), RepositoryError> {
        Ok(self.user.update_user(user_id, changes))
    }
}

#[async_trait::async_trait]
impl<'a> ChangeResetTokenDestinationRepository for MemoryAuthUserRepository<'a> {
    async fn lookup_destination(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<ResetTokenDestination>, RepositoryError> {
        Ok(self.login_id.get_reset_token_destination(login_id))
    }

    async fn change_destination(
        &self,
        login_id: LoginId,
        new_destination: ResetTokenDestination,
    ) -> Result<(), RepositoryError> {
        Ok(self
            .login_id
            .update_reset_token_destination(login_id, new_destination))
    }
}

#[async_trait::async_trait]
impl<'a> RegisterResetTokenRepository for MemoryAuthUserRepository<'a> {
    async fn lookup_user(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<(AuthUserId, Option<ResetTokenDestination>)>, RepositoryError> {
        Ok(self.login_id.get_reset_token_entry(login_id))
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
        Ok(self.reset_token.insert_reset_token(
            reset_token,
            user_id,
            login_id,
            destination,
            expires,
            requested_at,
        ))
    }
}

#[async_trait::async_trait]
impl<'a> ResetPasswordRepository for MemoryAuthUserRepository<'a> {
    async fn lookup_reset_token_entry(
        &self,
        reset_token: &ResetToken,
    ) -> Result<
        Option<(AuthUserId, LoginId, ResetTokenDestination, ResetTokenMoment)>,
        RepositoryError,
    > {
        Ok(self.reset_token.get_reset_token_entry(reset_token))
    }

    async fn lookup_granted_roles(
        &self,
        user_id: &AuthUserId,
    ) -> Result<Option<Option<GrantedAuthRoles>>, RepositoryError> {
        Ok(self.user.get_granted_roles(user_id))
    }

    async fn reset_password(
        &self,
        user_id: AuthUserId,
        reset_token: ResetToken,
        new_password: HashedPassword,
        reset_at: AuthDateTime,
    ) -> Result<(), RepositoryError> {
        reset_password(self, user_id, reset_token, new_password, reset_at)
    }
}
fn reset_password<'a>(
    repository: &MemoryAuthUserRepository<'a>,
    user_id: AuthUserId,
    reset_token: ResetToken,
    new_password: HashedPassword,
    reset_at: AuthDateTime,
) -> Result<(), RepositoryError> {
    repository
        .reset_token
        .update_reset_at(reset_token, reset_at);

    repository.user.update_password(user_id, new_password);

    Ok(())
}

#[async_trait::async_trait]
impl<'a> SearchAuthUserAccountRepository for MemoryAuthUserRepository<'a> {
    async fn search(
        &self,
        filter: SearchAuthUserAccountFilter,
    ) -> Result<AuthUserAccountSearch, RepositoryError> {
        search(&self, filter)
    }
}
fn search<'a>(
    repository: &MemoryAuthUserRepository<'a>,
    filter: SearchAuthUserAccountFilter,
) -> Result<AuthUserAccountSearch, RepositoryError> {
    let mut users = repository.user.all();
    let mut destinations: HashMap<LoginId, EntryLoginId> =
        repository.login_id.all().into_iter().collect();

    match filter.sort().key() {
        SearchAuthUserAccountSortKey::LoginId => {
            users.sort_by_cached_key(|(_, user)| user.login_id.clone());
            match filter.sort().order() {
                SearchSortOrder::Normal => (),
                SearchSortOrder::Reverse => users.reverse(),
            }
        }
    };

    let mut users: Vec<AuthUserAccount> = users
        .into_iter()
        .filter(|(_, user)| {
            filter.match_login_id(&user.login_id) && filter.match_granted_roles(&user.granted_roles)
        })
        .map(|(_, user)| {
            let entry = destinations.remove(&user.login_id);
            AuthUserAccount {
                login_id: user.login_id,
                granted_roles: user.granted_roles.unwrap_or(GrantedAuthRoles::empty()),
                reset_token_destination: entry
                    .and_then(|entry| entry.reset_token_destination)
                    .unwrap_or(ResetTokenDestination::None),
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
