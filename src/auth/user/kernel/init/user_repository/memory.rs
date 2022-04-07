mod login_id;
mod reset_token;
mod user;

use std::collections::HashMap;

use crate::auth::user::kernel::data::AuthUser;
use crate::auth::user::kernel::init::user_repository::memory::login_id::{
    EntryLoginId, MapLoginId,
};
use crate::auth::user::kernel::init::user_repository::memory::reset_token::{
    EntryResetToken, MapResetToken,
};
use crate::auth::user::kernel::init::user_repository::memory::user::{EntryUser, MapUser};
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

pub struct MemoryAuthUserRepository {
    user: MapUser,
    login_id: MapLoginId,
    reset_token: MapResetToken,
}

impl MemoryAuthUserRepository {
    pub fn new() -> Self {
        Self {
            user: MapUser::new(),
            login_id: MapLoginId::new(),
            reset_token: MapResetToken::new(),
        }
    }

    pub fn insert_user(&self, login_id: LoginId, user_id: AuthUserId) {
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

    pub fn with_user_id(login_id: LoginId, user_id: AuthUserId) -> Self {
        let repository = Self::new();
        repository.insert_user(login_id, user_id);
        repository
    }
    pub fn with_user_and_password(
        login_id: LoginId,
        user: AuthUser,
        password: HashedPassword,
        users: Vec<(LoginId, AuthUserId)>,
    ) -> Self {
        let repository = Self::new();

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
        login_id: LoginId,
        user_id: AuthUserId,
        destination: ResetTokenDestination,
    ) -> Self {
        let repository = Self::new();

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
        login_id: LoginId,
        user: AuthUser,
        reset_token: ResetToken,
        destination: ResetTokenDestination,
        expires: ExpireDateTime,
        requested_at: AuthDateTime,
        reset_at: Option<AuthDateTime>,
    ) -> Self {
        let repository = Self::new();

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
}

#[async_trait::async_trait]
impl AuthenticatePasswordRepository for MemoryAuthUserRepository {
    async fn lookup_user_id(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<AuthUserId>, RepositoryError> {
        Ok(self.login_id.lookup_user_id(login_id))
    }

    async fn lookup_user(
        &self,
        user_id: &AuthUserId,
    ) -> Result<Option<(HashedPassword, Option<GrantedAuthRoles>)>, RepositoryError> {
        Ok(self.user.lookup_password_and_granted_roles(user_id))
    }
}

#[async_trait::async_trait]
impl OverrideLoginIdRepository for MemoryAuthUserRepository {
    async fn lookup_user(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<OverrideLoginIdEntry>, RepositoryError> {
        Ok(self.login_id.lookup_override_entry(login_id))
    }

    async fn check_login_id_registered(&self, login_id: &LoginId) -> Result<bool, RepositoryError> {
        Ok(self.login_id.lookup_user_id(login_id).is_some())
    }

    async fn override_login_id(
        &self,
        new_login_id: LoginId,
        user: OverrideLoginIdEntry,
    ) -> Result<(), RepositoryError> {
        override_login_id(self, new_login_id, user)
    }
}
fn override_login_id(
    repository: &MemoryAuthUserRepository,
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
impl ChangePasswordRepository for MemoryAuthUserRepository {
    async fn lookup_password(
        &self,
        user_id: &AuthUserId,
    ) -> Result<Option<HashedPassword>, RepositoryError> {
        Ok(self.user.lookup_password(user_id))
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
impl OverridePasswordRepository for MemoryAuthUserRepository {
    async fn lookup_user_id(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<AuthUserId>, RepositoryError> {
        Ok(self.login_id.lookup_user_id(login_id))
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
impl ModifyAuthUserAccountRepository for MemoryAuthUserRepository {
    async fn lookup_user_id(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<AuthUserId>, RepositoryError> {
        Ok(self.login_id.lookup_user_id(login_id))
    }

    async fn lookup_changes(
        &self,
        user_id: &AuthUserId,
    ) -> Result<Option<ModifyAuthUserAccountChanges>, RepositoryError> {
        Ok(self.user.lookup_modify_changes(user_id))
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
impl ChangeResetTokenDestinationRepository for MemoryAuthUserRepository {
    async fn lookup_destination(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<ResetTokenDestination>, RepositoryError> {
        Ok(self.login_id.lookup_reset_token_destination(login_id))
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
impl RegisterResetTokenRepository for MemoryAuthUserRepository {
    async fn lookup_user(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<(AuthUserId, Option<ResetTokenDestination>)>, RepositoryError> {
        Ok(self.login_id.lookup_reset_token_entry(login_id))
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
impl ResetPasswordRepository for MemoryAuthUserRepository {
    async fn lookup_reset_token_entry(
        &self,
        reset_token: &ResetToken,
    ) -> Result<
        Option<(AuthUserId, LoginId, ResetTokenDestination, ResetTokenMoment)>,
        RepositoryError,
    > {
        Ok(self.reset_token.lookup_reset_token_entry(reset_token))
    }

    async fn lookup_granted_roles(
        &self,
        user_id: &AuthUserId,
    ) -> Result<Option<Option<GrantedAuthRoles>>, RepositoryError> {
        Ok(self.user.lookup_granted_roles(user_id))
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
fn reset_password(
    repository: &MemoryAuthUserRepository,
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
impl SearchAuthUserAccountRepository for MemoryAuthUserRepository {
    async fn search(
        &self,
        filter: SearchAuthUserAccountFilter,
    ) -> Result<AuthUserAccountSearch, RepositoryError> {
        search(&self, filter)
    }
}
fn search(
    repository: &MemoryAuthUserRepository,
    filter: SearchAuthUserAccountFilter,
) -> Result<AuthUserAccountSearch, RepositoryError> {
    let mut users = repository.user.all();
    let mut destinations: HashMap<LoginId, EntryLoginId> =
        repository.login_id.all().into_iter().collect();

    let all: i32 = users
        .len()
        .try_into()
        .map_err(|err| infra_error("convert users length error", err))?;

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
        .filter(|(_, user)| match filter.login_id() {
            None => true,
            Some(filter_login_id) => user.login_id.as_str() == filter_login_id,
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
