mod login_id;
mod reset_token;
mod user;

use std::collections::HashMap;

use crate::auth::user::kernel::init::user_repository::memory::{
    login_id::{EntryLoginId, MapLoginId, StoreLoginId},
    reset_token::{EntryResetToken, MapResetToken, StoreResetToken},
    user::{EntryUser, MapUser, StoreUser},
};

use crate::common::api::search::helper::{clip_search, sort_normal, sort_search};

use crate::auth::{
    ticket::authorize::infra::ClarifyAuthorizeTokenAuthUserRepository,
    user::{
        account::{
            modify::infra::ModifyAuthUserAccountRepository,
            register::infra::RegisterAuthUserAccountRepository,
            search::infra::SearchAuthUserAccountRepository,
            unregister::infra::UnregisterAuthUserAccountRepository,
        },
        login_id::change::infra::{OverwriteLoginIdEntry, OverwriteLoginIdRepository},
        password::{
            authenticate::infra::AuthenticatePasswordRepository,
            change::infra::{ChangePasswordRepository, OverwritePasswordRepository},
            kernel::infra::HashedPassword,
            reset::{
                request_token::infra::RegisterResetPasswordTokenRepository,
                reset::infra::{ResetPasswordRepository, ResetPasswordTokenMoment},
                token_destination::change::infra::ChangeResetTokenDestinationRepository,
            },
        },
    },
};

use crate::{
    auth::{
        kernel::data::{AuthDateTime, ExpireDateTime},
        ticket::kernel::data::AuthPermissionGranted,
        user::{
            account::{
                kernel::data::{AuthUserAccount, AuthUserAccountAttrs, AuthUserMemo},
                search::data::{
                    AuthUserAccountSearch, SearchAuthUserAccountFilter,
                    SearchAuthUserAccountSortKey,
                },
            },
            kernel::data::{AuthUser, AuthUserId},
            login_id::kernel::data::LoginId,
            password::reset::kernel::data::{ResetPasswordId, ResetPasswordTokenDestination},
        },
    },
    common::api::{repository::data::RepositoryError, search::data::SearchLimit},
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
        repository.insert_user(login_id, user_id, None, None);
        repository
    }
    pub fn with_user_and_permission(
        store: &'a MemoryAuthUserStore,
        user_id: AuthUserId,
        login_id: LoginId,
        granted: AuthPermissionGranted,
    ) -> Self {
        let repository = Self::new(store);
        repository.insert_user(login_id, user_id, Some(granted), None);
        repository
    }
    pub fn with_user(
        store: &'a MemoryAuthUserStore,
        user_id: AuthUserId,
        login_id: LoginId,
        attrs: AuthUserAccountAttrs,
    ) -> Self {
        let repository = Self::new(store);
        repository.insert_user(login_id, user_id, Some(attrs.granted), Some(attrs.memo));
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

        repository.login_id.insert_entry(
            login_id.clone(),
            EntryLoginId {
                user_id: user.user_id.clone(),
                reset_token_destination: None,
            },
        );
        repository.user.insert_entry(
            user.user_id,
            EntryUser {
                login_id,
                granted: Some(user.granted),
                hashed_password: Some(password),
                memo: None,
            },
        );

        for (login_id, user_id) in users {
            repository.insert_user(login_id, user_id, None, None);
        }

        repository
    }
    pub fn with_user_id_and_destination(
        store: &'a MemoryAuthUserStore,
        login_id: LoginId,
        user_id: AuthUserId,
        destination: ResetPasswordTokenDestination,
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
                granted: None,
                hashed_password: None,
                memo: None,
            },
        );

        repository
    }
    pub fn with_user_and_reset_token(
        store: &'a MemoryAuthUserStore,
        login_id: LoginId,
        user: AuthUser,
        reset_token: ResetPasswordId,
        destination: ResetPasswordTokenDestination,
        expires: ExpireDateTime,
        requested_at: AuthDateTime,
        reset_at: Option<AuthDateTime>,
    ) -> Self {
        let repository = Self::new(store);

        repository.login_id.insert_entry(
            login_id.clone(),
            EntryLoginId {
                user_id: user.user_id.clone(),
                reset_token_destination: Some(destination.clone()),
            },
        );
        repository.user.insert_entry(
            user.user_id.clone(),
            EntryUser {
                login_id: login_id.clone(),
                granted: Some(user.granted),
                hashed_password: None,
                memo: None,
            },
        );
        repository.reset_token.insert_entry(
            reset_token,
            EntryResetToken {
                user_id: user.user_id,
                login_id,
                destination,
                expires,
                requested_at,
                reset_at,
            },
        );

        repository
    }

    fn insert_user(
        &self,
        login_id: LoginId,
        user_id: AuthUserId,
        granted: Option<AuthPermissionGranted>,
        memo: Option<AuthUserMemo>,
    ) {
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
                granted,
                hashed_password: None,
                memo,
            },
        );
    }
}

#[async_trait::async_trait]
impl<'a> ClarifyAuthorizeTokenAuthUserRepository for MemoryAuthUserRepository<'a> {
    async fn lookup_permission_granted(
        &self,
        user_id: &AuthUserId,
    ) -> Result<Option<AuthPermissionGranted>, RepositoryError> {
        Ok(self.user.get_granted(user_id))
    }
}

#[async_trait::async_trait]
impl<'a> AuthenticatePasswordRepository for MemoryAuthUserRepository<'a> {
    async fn lookup_user(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<(AuthUserId, HashedPassword, Option<AuthPermissionGranted>)>, RepositoryError>
    {
        match self.login_id.get_user_id(login_id) {
            None => Ok(None),
            Some(user_id) => match self.user.get_password_and_granted(&user_id) {
                None => Ok(None),
                Some((password, granted)) => Ok(Some((user_id, password, granted))),
            },
        }
    }
}

#[async_trait::async_trait]
impl<'a> OverwriteLoginIdRepository for MemoryAuthUserRepository<'a> {
    async fn lookup_user(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<OverwriteLoginIdEntry>, RepositoryError> {
        Ok(self.login_id.get_overwrite_entry(login_id))
    }

    async fn check_login_id_registered(&self, login_id: &LoginId) -> Result<bool, RepositoryError> {
        Ok(self.login_id.get_user_id(login_id).is_some())
    }

    async fn overwrite_login_id(
        &self,
        new_login_id: LoginId,
        user: OverwriteLoginIdEntry,
    ) -> Result<(), RepositoryError> {
        overwrite_login_id(self, new_login_id, user)
    }
}
fn overwrite_login_id<'a>(
    repository: &MemoryAuthUserRepository<'a>,
    new_login_id: LoginId,
    user: OverwriteLoginIdEntry,
) -> Result<(), RepositoryError> {
    repository
        .user
        .update_login_id(user.user_id.clone(), new_login_id.clone());

    repository.login_id.remove_entry(&user.login_id);
    repository
        .login_id
        .insert_overwrite_entry(new_login_id, user);

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
impl<'a> OverwritePasswordRepository for MemoryAuthUserRepository<'a> {
    async fn lookup_user_id(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<AuthUserId>, RepositoryError> {
        Ok(self.login_id.get_user_id(login_id))
    }

    async fn overwrite_password(
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
        fields: AuthUserAccount,
    ) -> Result<(), RepositoryError> {
        self.user.insert_entry(
            user_id.clone(),
            EntryUser {
                login_id: fields.login_id.clone(),
                granted: Some(fields.attrs.granted),
                hashed_password: None,
                memo: Some(fields.attrs.memo),
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
    async fn lookup_user_id(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<AuthUserId>, RepositoryError> {
        Ok(self.login_id.get_user_id(login_id))
    }

    async fn unregister_user(
        &self,
        user_id: &AuthUserId,
        login_id: &LoginId,
    ) -> Result<(), RepositoryError> {
        self.user.remove_entry(user_id);
        self.login_id.remove_entry(login_id);
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

    async fn lookup_attrs(
        &self,
        user_id: &AuthUserId,
    ) -> Result<Option<AuthUserAccountAttrs>, RepositoryError> {
        Ok(self.user.get_attrs(user_id))
    }

    async fn modify_user(
        &self,
        user_id: AuthUserId,
        attrs: AuthUserAccountAttrs,
    ) -> Result<(), RepositoryError> {
        Ok(self.user.update_user(user_id, attrs))
    }
}

#[async_trait::async_trait]
impl<'a> ChangeResetTokenDestinationRepository for MemoryAuthUserRepository<'a> {
    async fn lookup_destination(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<ResetPasswordTokenDestination>, RepositoryError> {
        Ok(self.login_id.get_reset_token_destination(login_id))
    }

    async fn change_destination(
        &self,
        login_id: LoginId,
        new_destination: ResetPasswordTokenDestination,
    ) -> Result<(), RepositoryError> {
        Ok(self
            .login_id
            .update_reset_token_destination(login_id, new_destination))
    }
}

#[async_trait::async_trait]
impl<'a> RegisterResetPasswordTokenRepository for MemoryAuthUserRepository<'a> {
    async fn lookup_user(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<(AuthUserId, Option<ResetPasswordTokenDestination>)>, RepositoryError> {
        Ok(self.login_id.get_reset_token_entry(login_id))
    }

    async fn register_reset_token(
        &self,
        reset_token: ResetPasswordId,
        user_id: AuthUserId,
        login_id: LoginId,
        destination: ResetPasswordTokenDestination,
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
        reset_token: &ResetPasswordId,
    ) -> Result<
        Option<(
            AuthUserId,
            LoginId,
            ResetPasswordTokenDestination,
            ResetPasswordTokenMoment,
        )>,
        RepositoryError,
    > {
        Ok(self.reset_token.get_reset_token_entry(reset_token))
    }

    async fn lookup_permission_granted(
        &self,
        user_id: &AuthUserId,
    ) -> Result<Option<AuthPermissionGranted>, RepositoryError> {
        Ok(self.user.get_granted(user_id))
    }

    async fn reset_password(
        &self,
        user_id: AuthUserId,
        reset_token: ResetPasswordId,
        new_password: HashedPassword,
        reset_at: AuthDateTime,
    ) -> Result<(), RepositoryError> {
        reset_password(self, user_id, reset_token, new_password, reset_at)
    }
}
fn reset_password<'a>(
    repository: &MemoryAuthUserRepository<'a>,
    user_id: AuthUserId,
    reset_token: ResetPasswordId,
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
    let mut destinations: HashMap<LoginId, EntryLoginId> =
        repository.login_id.all().into_iter().collect();

    let users = repository
        .user
        .all()
        .into_iter()
        .filter(|(_, user)| {
            filter.props.match_login_id(&user.login_id) && filter.props.match_granted(&user.granted)
        })
        .map(|(_, user)| {
            let entry = destinations.remove(&user.login_id);
            AuthUserAccount {
                login_id: user.login_id,
                attrs: AuthUserAccountAttrs {
                    granted: user.granted.unwrap_or_default(),
                    memo: user.memo.unwrap_or(AuthUserMemo::empty()),
                },
                reset_token_destination: entry
                    .and_then(|entry| entry.reset_token_destination)
                    .unwrap_or(ResetPasswordTokenDestination::None),
            }
        })
        .collect();

    let (users, page) = clip_search(
        sort_search(
            users,
            |user| match filter.sort.key {
                SearchAuthUserAccountSortKey::LoginId => user.login_id.clone(),
            },
            match filter.sort.key {
                SearchAuthUserAccountSortKey::LoginId => sort_normal,
            }(filter.sort.order),
        ),
        filter.offset,
        SearchLimit::default(),
    )?;

    Ok(AuthUserAccountSearch {
        page,
        sort: filter.sort,
        users,
    })
}
