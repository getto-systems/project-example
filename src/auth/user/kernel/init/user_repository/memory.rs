use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
};

use chrono::{DateTime, Utc};

use crate::{
    auth::user::password::reset::token_destination::change::infra::ChangeResetTokenDestinationRepository,
    z_lib::repository::helper::infra_error,
};

use crate::auth::user::{
    account::{
        modify::infra::ModifyAuthUserAccountRepository,
        search::infra::SearchAuthUserAccountRepository,
    },
    kernel::infra::AuthUserRepository,
    login_id::change::infra::OverrideLoginIdRepository,
    password::{
        authenticate::infra::VerifyPasswordRepository,
        change::infra::{ChangePasswordRepository, OverridePasswordRepository},
        kernel::infra::{AuthUserPasswordHasher, AuthUserPasswordMatcher, HashedPassword},
        reset::{
            kernel::infra::{ResetTokenEntry, ResetTokenEntryExtract},
            request_token::infra::{RegisterResetTokenRepository, ResetTokenDestinationRepository},
            reset::infra::ResetPasswordRepository,
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
                search::data::{SearchAuthUserAccountBasket, SearchAuthUserAccountFilter},
            },
            kernel::data::{AuthUser, AuthUserExtract, AuthUserId, GrantedAuthRoles},
            login_id::kernel::data::LoginId,
            password::{
                change::data::{ChangePasswordRepositoryError, OverridePasswordRepositoryError},
                reset::{
                    kernel::data::{
                        ResetToken, ResetTokenDestination, ResetTokenDestinationExtract,
                    },
                    request_token::data::RegisterResetTokenRepositoryError,
                    reset::data::ResetPasswordRepositoryError,
                },
            },
        },
    },
    z_lib::{repository::data::RepositoryError, search::data::SearchPage},
};

pub type MemoryAuthUserStore = Mutex<MemoryAuthUserMap>;
// TODO user-id => {user} と login-id => {user} のマップにしたい
pub struct MemoryAuthUserMap {
    login_id: HashMap<String, AuthUserId>, // login-id => user-id
    user_id: HashMap<String, LoginId>,     // user-id => login-id
    granted_roles: HashMap<String, HashSet<String>>, // user-id => granted-roles
    password: HashMap<String, HashedPassword>, // user-id => hashed-password
    reset_token_destination: HashMap<String, ResetTokenDestinationExtract>, // login-id => destination
    reset_token: HashMap<String, ResetEntry>, // reset-token => reset entry
}

pub struct UserEntry {
    user_id: AuthUserId,
    login_id: LoginId,
    reset_token_destination: ResetTokenDestination,
}

impl Into<AuthUserId> for UserEntry {
    fn into(self) -> AuthUserId {
        self.user_id
    }
}

struct SearchUserEntry {
    pub user_id: String,
    pub granted_roles: HashSet<String>,
    pub reset_token_destination: ResetTokenDestinationExtract,
}

#[derive(Clone)]
struct ResetEntry {
    user_id: AuthUserId,
    login_id: String,
    destination: ResetTokenDestinationExtract,
    expires: DateTime<Utc>,
    reset_at: Option<DateTime<Utc>>,
}

impl ResetEntry {
    fn discard(self, reset_at: AuthDateTime) -> Self {
        Self {
            reset_at: Some(reset_at.extract()),
            ..self
        }
    }
}

impl MemoryAuthUserMap {
    pub fn new() -> Self {
        Self {
            login_id: HashMap::new(),
            user_id: HashMap::new(),
            granted_roles: HashMap::new(),
            password: HashMap::new(),
            reset_token_destination: HashMap::new(),
            reset_token: HashMap::new(),
        }
    }
    pub fn to_store(self) -> MemoryAuthUserStore {
        Mutex::new(self)
    }

    pub fn with_user_id(login_id: LoginId, user_id: AuthUserId) -> Self {
        let mut store = Self::new();
        store.insert_login_id(login_id, user_id);
        store
    }
    pub fn with_user(user: AuthUser) -> Self {
        let mut store = Self::new();
        store.insert_granted_roles(user);
        store
    }
    pub fn with_user_and_password(
        login_id: LoginId,
        user: AuthUser,
        password: HashedPassword,
        users: Vec<(LoginId, AuthUserId)>,
    ) -> Self {
        let mut store = Self::new();
        let user_id = user.clone().into_user_id();
        store
            .insert_granted_roles(user)
            .insert_login_id(login_id.clone(), user_id.clone())
            .insert_password(user_id, password);
        for (login_id, user_id) in users {
            store.insert_login_id(login_id, user_id);
        }
        store
    }
    pub fn with_dangling_password(
        login_id: LoginId,
        user: AuthUser,
        password: HashedPassword,
    ) -> Self {
        let mut store = Self::new();
        let user_id = user.into_user_id();
        store
            .insert_login_id(login_id, user_id.clone())
            .insert_password(user_id, password);
        store
    }
    pub fn with_user_id_and_destination(
        login_id: LoginId,
        user_id: AuthUserId,
        destination: ResetTokenDestination,
    ) -> Self {
        let mut store = Self::new();
        store
            .insert_login_id(login_id.clone(), user_id)
            .insert_destination(login_id, destination.extract());
        store
    }
    pub fn with_user_and_reset_token(
        login_id: LoginId,
        user: AuthUser,
        user_id: AuthUserId,
        reset_token: ResetToken,
        destination: ResetTokenDestination,
        expires: ExpireDateTime,
        discard_at: Option<AuthDateTime>,
    ) -> Self {
        let mut store = Self::new();
        store
            .insert_granted_roles(user)
            .insert_login_id(login_id.clone(), user_id.clone())
            .insert_reset_token(
                reset_token,
                ResetEntry {
                    user_id,
                    login_id: login_id.extract(),
                    destination: destination.extract(),
                    expires: expires.extract(),
                    reset_at: discard_at.map(|discard_at| discard_at.extract()),
                },
            );
        store
    }
    pub fn with_dangling_reset_token(
        login_id: LoginId,
        user_id: AuthUserId,
        reset_token: ResetToken,
        destination: ResetTokenDestination,
        expires: ExpireDateTime,
        discard_at: Option<AuthDateTime>,
    ) -> Self {
        let mut store = Self::new();
        store
            .insert_login_id(login_id.clone(), user_id.clone())
            .insert_reset_token(
                reset_token,
                ResetEntry {
                    user_id,
                    login_id: login_id.extract(),
                    destination: destination.extract(),
                    expires: expires.extract(),
                    reset_at: discard_at.map(|discard_at| discard_at.extract()),
                },
            );
        store
    }

    fn has_login_id(&self, login_id: &LoginId) -> bool {
        self.login_id.contains_key(login_id.as_str())
    }

    fn insert_login_id(&mut self, login_id: LoginId, user_id: AuthUserId) -> &mut Self {
        self.login_id
            .insert(login_id.clone().extract(), user_id.clone());
        self.user_id.insert(user_id.extract(), login_id);
        self
    }
    fn update_login_id(&mut self, user_id: AuthUserId, login_id: LoginId) {
        self.user_id.insert(user_id.extract(), login_id);
    }
    fn get_user_id(&self, login_id: &LoginId) -> Option<&AuthUserId> {
        self.login_id.get(login_id.as_str())
    }
    fn get_login_id(&self, user_id: &str) -> Option<&LoginId> {
        self.user_id.get(user_id)
    }

    fn remove_user(&mut self, login_id: LoginId) {
        let login_id = login_id.extract();
        self.login_id.remove(&login_id);
        self.reset_token_destination.remove(&login_id);
    }
    fn insert_user(&mut self, login_id: LoginId, user: UserEntry) {
        let login_id = login_id.extract();
        self.login_id.insert(login_id.clone(), user.user_id);
        self.reset_token_destination
            .insert(login_id, user.reset_token_destination.extract());
    }

    fn insert_granted_roles(&mut self, user: AuthUser) -> &mut Self {
        let user = user.extract();
        self.granted_roles.insert(user.user_id, user.granted_roles);
        self
    }
    fn update_granted_roles(
        &mut self,
        user_id: AuthUserId,
        granted_roles: GrantedAuthRoles,
    ) -> &mut Self {
        self.granted_roles
            .insert(user_id.extract(), granted_roles.extract());
        self
    }
    fn get_granted_roles(&self, user_id: &AuthUserId) -> Option<&HashSet<String>> {
        self.granted_roles.get(user_id.as_str())
    }
    fn all_users(&self) -> Vec<SearchUserEntry> {
        self.granted_roles
            .iter()
            .map(|(user_id, granted_roles)| SearchUserEntry {
                user_id: user_id.clone(),
                granted_roles: granted_roles.clone(),
                reset_token_destination: {
                    if let Some(login_id) = self.get_login_id(user_id) {
                        if let Some(destination) = self.get_destination(login_id) {
                            destination.clone()
                        } else {
                            ResetTokenDestinationExtract::None
                        }
                    } else {
                        ResetTokenDestinationExtract::None
                    }
                },
            })
            .collect()
    }

    fn insert_password(
        &mut self,
        user_id: AuthUserId,
        hashed_password: HashedPassword,
    ) -> &mut Self {
        self.password.insert(user_id.extract(), hashed_password);
        self
    }
    fn get_password(&self, user_id: &AuthUserId) -> Option<&HashedPassword> {
        self.password.get(user_id.as_str())
    }

    fn insert_destination(&mut self, login_id: LoginId, destination: ResetTokenDestinationExtract) {
        self.reset_token_destination
            .insert(login_id.extract(), destination);
    }
    fn update_destination(&mut self, login_id: LoginId, destination: ResetTokenDestination) {
        self.reset_token_destination
            .insert(login_id.extract(), destination.extract());
    }
    fn get_destination(&self, login_id: &LoginId) -> Option<&ResetTokenDestinationExtract> {
        self.reset_token_destination.get(login_id.as_str())
    }

    fn insert_reset_token(&mut self, token: ResetToken, entry: ResetEntry) -> &mut Self {
        self.reset_token.insert(token.extract(), entry);
        self
    }
    fn get_reset_entry(&self, token: &ResetToken) -> Option<&ResetEntry> {
        self.reset_token.get(token.as_str())
    }
}

pub struct MemoryAuthUserRepository<'a> {
    store: &'a MemoryAuthUserStore,
}

impl<'a> MemoryAuthUserRepository<'a> {
    pub const fn new(store: &'a MemoryAuthUserStore) -> Self {
        Self { store }
    }
}

// TODO (AuthUserId, GrantedAuthRoles) を返せばいい気がする
#[async_trait::async_trait]
impl<'a> AuthUserRepository for MemoryAuthUserRepository<'a> {
    async fn get(&self, user_id: &AuthUserId) -> Result<Option<AuthUser>, RepositoryError> {
        get_granted_roles(self, user_id)
    }
}
fn get_granted_roles<'a>(
    repository: &MemoryAuthUserRepository<'a>,
    user_id: &AuthUserId,
) -> Result<Option<AuthUser>, RepositoryError> {
    let store = repository.store.lock().unwrap();
    Ok(store.get_granted_roles(user_id).map(|granted_roles| {
        AuthUserExtract {
            user_id: user_id.as_str().into(),
            granted_roles: granted_roles.clone(),
        }
        .restore()
    }))
}

// TODO Basket をやめる
#[async_trait::async_trait]
impl<'a> SearchAuthUserAccountRepository for MemoryAuthUserRepository<'a> {
    async fn search(
        &self,
        filter: SearchAuthUserAccountFilter,
    ) -> Result<SearchAuthUserAccountBasket, RepositoryError> {
        search(&self, filter)
    }
}
fn search<'a>(
    repository: &MemoryAuthUserRepository<'a>,
    filter: SearchAuthUserAccountFilter,
) -> Result<SearchAuthUserAccountBasket, RepositoryError> {
    let store = repository.store.lock().unwrap();
    let users = store
        .all_users()
        .into_iter()
        // 実際のデータベースでは fields を使用して検索を行う
        .filter_map(|user| {
            store
                .get_login_id(&user.user_id)
                .map(|login_id| AuthUserAccount {
                    login_id: LoginId::restore(login_id.clone().extract()),
                    granted_roles: GrantedAuthRoles::restore(user.granted_roles),
                    reset_token_destination: ResetTokenDestination::restore(
                        user.reset_token_destination,
                    ),
                })
        })
        .collect();

    Ok(SearchAuthUserAccountBasket {
        page: SearchPage {
            offset: 0,
            limit: 0,
            all: 0,
        },
        sort: filter.into_sort(),
        users,
    })
}

#[async_trait::async_trait]
impl<'a> ModifyAuthUserAccountRepository for MemoryAuthUserRepository<'a> {
    async fn lookup_user(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<(AuthUserId, ModifyAuthUserAccountChanges)>, RepositoryError> {
        lookup_modify_user_data(self, login_id)
    }

    async fn modify_user(
        &self,
        user_id: &AuthUserId,
        data: ModifyAuthUserAccountChanges,
    ) -> Result<(), RepositoryError> {
        modify_user(self, user_id, data)
    }

    async fn get_updated_user(
        &self,
        user_id: &AuthUserId,
    ) -> Result<ModifyAuthUserAccountChanges, RepositoryError> {
        get_modify_user_data(self, user_id)
    }
}
fn lookup_modify_user_data<'a>(
    repository: &MemoryAuthUserRepository<'a>,
    login_id: &LoginId,
) -> Result<Option<(AuthUserId, ModifyAuthUserAccountChanges)>, RepositoryError> {
    let target_user_id: AuthUserId;

    {
        let store = repository.store.lock().unwrap();

        match store.get_user_id(login_id) {
            None => {
                return Ok(None);
            }
            Some(user_id) => {
                target_user_id = user_id.clone();
            }
        }
    }

    Ok(Some((
        target_user_id.clone(),
        get_modify_user_data(repository, &target_user_id)?,
    )))
}
fn modify_user<'a>(
    repository: &MemoryAuthUserRepository<'a>,
    user_id: &AuthUserId,
    data: ModifyAuthUserAccountChanges,
) -> Result<(), RepositoryError> {
    let mut store = repository.store.lock().unwrap();

    store.update_granted_roles(user_id.clone(), data.granted_roles);

    Ok(())
}
fn get_modify_user_data<'a>(
    repository: &MemoryAuthUserRepository<'a>,
    user_id: &AuthUserId,
) -> Result<ModifyAuthUserAccountChanges, RepositoryError> {
    let store = repository.store.lock().unwrap();

    Ok(ModifyAuthUserAccountChanges {
        granted_roles: store
            .get_granted_roles(user_id)
            .map(|granted_roles| GrantedAuthRoles::restore(granted_roles.clone()))
            .unwrap_or(GrantedAuthRoles::empty()),
    })
}

#[async_trait::async_trait]
impl<'a> ChangeResetTokenDestinationRepository for MemoryAuthUserRepository<'a> {
    async fn lookup_destination(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<(AuthUserId, ResetTokenDestination)>, RepositoryError> {
        lookup_reset_token_destination(self, login_id)
    }

    async fn change_destination(
        &self,
        login_id: &LoginId,
        data: ResetTokenDestination,
    ) -> Result<(), RepositoryError> {
        change_reset_token_destination(self, login_id, data)
    }

    async fn get_updated_destination(
        &self,
        login_id: &LoginId,
    ) -> Result<ResetTokenDestination, RepositoryError> {
        get_destination(self, login_id).and_then(|destination| {
            destination.ok_or(RepositoryError::InfraError(
                "updated destination not found".into(),
            ))
        })
    }
}
fn lookup_reset_token_destination<'a>(
    repository: &MemoryAuthUserRepository<'a>,
    login_id: &LoginId,
) -> Result<Option<(AuthUserId, ResetTokenDestination)>, RepositoryError> {
    let target_user_id: AuthUserId;

    {
        let store = repository.store.lock().unwrap();

        match store.get_user_id(login_id) {
            None => {
                return Ok(None);
            }
            Some(user_id) => {
                target_user_id = user_id.clone();
            }
        }
    }

    match get_destination(repository, login_id)? {
        Some(destination) => Ok(Some((target_user_id, destination))),
        _ => Ok(None),
    }
}
fn change_reset_token_destination<'a>(
    repository: &MemoryAuthUserRepository<'a>,
    login_id: &LoginId,
    data: ResetTokenDestination,
) -> Result<(), RepositoryError> {
    let mut store = repository.store.lock().unwrap();

    store.update_destination(login_id.clone(), data);

    Ok(())
}

#[async_trait::async_trait]
impl<'store> OverrideLoginIdRepository for MemoryAuthUserRepository<'store> {
    type User = UserEntry;

    async fn lookup_user<'a>(
        &self,
        login_id: &'a LoginId,
    ) -> Result<Option<Self::User>, RepositoryError> {
        lookup_user(self, login_id)
    }

    async fn check_login_id_registered<'a>(
        &self,
        login_id: &'a LoginId,
    ) -> Result<bool, RepositoryError> {
        check_login_id_registered(self, login_id)
    }

    async fn override_login_id<'a>(
        &self,
        user: Self::User,
        new_login_id: LoginId,
    ) -> Result<(), RepositoryError> {
        override_login_id(self, user, new_login_id)
    }
}
fn lookup_user<'store, 'a>(
    repository: &MemoryAuthUserRepository<'store>,
    login_id: &'a LoginId,
) -> Result<Option<UserEntry>, RepositoryError> {
    let user_id: Option<AuthUserId>;
    let reset_token_destination: ResetTokenDestination;
    {
        let store = repository.store.lock().unwrap();

        user_id = store.get_user_id(login_id).map(|user_id| user_id.clone());
        reset_token_destination = store
            .get_destination(login_id)
            .map(|destination| ResetTokenDestination::restore(destination.clone()))
            .unwrap_or(ResetTokenDestination::None);
    }

    Ok(user_id.map(|user_id| UserEntry {
        user_id,
        login_id: login_id.clone(),
        reset_token_destination,
    }))
}
fn check_login_id_registered<'store, 'a>(
    repository: &MemoryAuthUserRepository<'store>,
    new_login_id: &'a LoginId,
) -> Result<bool, RepositoryError> {
    let store = repository.store.lock().unwrap();

    Ok(store.has_login_id(new_login_id))
}
fn override_login_id<'store, 'a>(
    repository: &MemoryAuthUserRepository<'store>,
    user: UserEntry,
    new_login_id: LoginId,
) -> Result<(), RepositoryError> {
    let mut store = repository.store.lock().unwrap();

    store.update_login_id(user.user_id.clone(), new_login_id.clone());
    store.remove_user(user.login_id.clone());
    store.insert_user(new_login_id, user);

    Ok(())
}

#[async_trait::async_trait]
impl<'store> VerifyPasswordRepository for MemoryAuthUserRepository<'store> {
    async fn lookup_user_id<'a>(
        &self,
        login_id: &'a LoginId,
    ) -> Result<Option<AuthUserId>, RepositoryError> {
        lookup_user_id(self, login_id)
    }

    async fn lookup_granted_roles<'a>(
        &self,
        user_id: &'a AuthUserId,
    ) -> Result<Option<GrantedAuthRoles>, RepositoryError> {
        lookup_granted_roles(self, user_id)
    }

    async fn lookup_password<'a>(
        &self,
        user_id: &'a AuthUserId,
    ) -> Result<Option<HashedPassword>, RepositoryError> {
        lookup_password(self, user_id)
    }
}
fn lookup_user_id<'store, 'a>(
    repository: &MemoryAuthUserRepository<'store>,
    login_id: &'a LoginId,
) -> Result<Option<AuthUserId>, RepositoryError> {
    let store = repository.store.lock().unwrap();
    Ok(store.get_user_id(login_id).map(|user_id| user_id.clone()))
}
fn lookup_granted_roles<'store, 'a>(
    repository: &MemoryAuthUserRepository<'store>,
    user_id: &'a AuthUserId,
) -> Result<Option<GrantedAuthRoles>, RepositoryError> {
    let store = repository.store.lock().unwrap();
    Ok(store
        .get_granted_roles(user_id)
        .map(|granted_roles| GrantedAuthRoles::restore(granted_roles.clone())))
}
fn lookup_password<'store, 'a>(
    repository: &MemoryAuthUserRepository<'store>,
    user_id: &'a AuthUserId,
) -> Result<Option<HashedPassword>, RepositoryError> {
    let store = repository.store.lock().unwrap();
    Ok(store
        .get_password(user_id)
        .map(|password| password.clone()))
}

// TODO password を取得してマッチするのを外に出す
// TODO password hash を外に出す
#[async_trait::async_trait]
impl<'store> ChangePasswordRepository for MemoryAuthUserRepository<'store> {
    async fn change_password<'a>(
        &self,
        user_id: &'a AuthUserId,
        matcher: impl 'a + AuthUserPasswordMatcher,
        hasher: impl 'a + AuthUserPasswordHasher,
    ) -> Result<(), ChangePasswordRepositoryError> {
        change_password(self, user_id, matcher, hasher)
    }
}
fn change_password<'store, 'a>(
    repository: &MemoryAuthUserRepository<'store>,
    user_id: &'a AuthUserId,
    matcher: impl 'a + AuthUserPasswordMatcher,
    hasher: impl 'a + AuthUserPasswordHasher,
) -> Result<(), ChangePasswordRepositoryError> {
    {
        let store = repository.store.lock().unwrap();

        let password = store
            .get_password(user_id)
            .ok_or(ChangePasswordRepositoryError::PasswordNotFound)?;

        let matched = matcher
            .match_password(password)
            .map_err(ChangePasswordRepositoryError::PasswordHashError)?;

        if !matched {
            return Err(ChangePasswordRepositoryError::PasswordNotMatched);
        }
    }

    {
        let hashed_password = hasher
            .hash_password()
            .map_err(ChangePasswordRepositoryError::PasswordHashError)?;

        let mut store = repository.store.lock().unwrap();

        store.insert_password(user_id.clone(), hashed_password);
    }

    Ok(())
}

// TODO password hash を外に出す
// TODO user を取得するのを外に出す
#[async_trait::async_trait]
impl<'store> OverridePasswordRepository for MemoryAuthUserRepository<'store> {
    async fn override_password<'a>(
        &self,
        login_id: &'a LoginId,
        hasher: impl 'a + AuthUserPasswordHasher,
    ) -> Result<(), OverridePasswordRepositoryError> {
        override_password(self, login_id, hasher)
    }
}
fn override_password<'store, 'a>(
    repository: &MemoryAuthUserRepository<'store>,
    login_id: &'a LoginId,
    hasher: impl 'a + AuthUserPasswordHasher,
) -> Result<(), OverridePasswordRepositoryError> {
    {
        let hashed_password = hasher
            .hash_password()
            .map_err(OverridePasswordRepositoryError::PasswordHashError)?;

        let mut store = repository.store.lock().unwrap();

        let user_id = store
            .get_user_id(login_id)
            .ok_or(OverridePasswordRepositoryError::UserNotFound)
            .map(|id| id.clone())?;

        store.insert_password(user_id, hashed_password);
    }

    Ok(())
}

#[async_trait::async_trait]
impl<'store> ResetTokenDestinationRepository for MemoryAuthUserRepository<'store> {
    async fn get(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<ResetTokenDestination>, RepositoryError> {
        get_destination(self, login_id)
    }
}
fn get_destination<'store>(
    repository: &MemoryAuthUserRepository<'store>,
    login_id: &LoginId,
) -> Result<Option<ResetTokenDestination>, RepositoryError> {
    let store = repository.store.lock().unwrap();
    Ok(store
        .get_destination(login_id)
        .map(|destination| ResetTokenDestination::restore(destination.clone())))
}

// TODO reset token を取得して有効期限を確認するのを外に出す
#[async_trait::async_trait]
impl<'store> RegisterResetTokenRepository for MemoryAuthUserRepository<'store> {
    async fn register_reset_token(
        &self,
        reset_token: ResetToken,
        login_id: LoginId,
        destination: ResetTokenDestination,
        expires: ExpireDateTime,
        requested_at: AuthDateTime,
    ) -> Result<(), RegisterResetTokenRepositoryError> {
        register_reset_token(
            self,
            login_id,
            reset_token,
            destination,
            expires,
            requested_at,
        )
    }
}
fn register_reset_token<'store>(
    repository: &MemoryAuthUserRepository<'store>,
    login_id: LoginId,
    reset_token: ResetToken,
    destination: ResetTokenDestination,
    expires: ExpireDateTime,
    _requested_at: AuthDateTime,
) -> Result<(), RegisterResetTokenRepositoryError> {
    let target_user_id: AuthUserId;

    {
        let store = repository.store.lock().unwrap();

        let user_id = store
            .get_user_id(&login_id)
            .ok_or(RegisterResetTokenRepositoryError::UserNotFound)?;

        // 有効期限が切れていても削除しない
        // もし衝突したら token generator の桁数を増やす
        if store.get_reset_entry(&reset_token).is_some() {
            return Err(RegisterResetTokenRepositoryError::RepositoryError(
                infra_error("get reset entry error", "reset token conflict"),
            ));
        }

        target_user_id = user_id.clone();
    }

    {
        let mut store = repository.store.lock().unwrap();

        // 実際のデータベースには registered_at も保存する必要がある
        store.insert_reset_token(
            reset_token.clone(),
            ResetEntry {
                user_id: target_user_id,
                login_id: login_id.extract(),
                destination: destination.extract(),
                expires: expires.extract(),
                reset_at: None,
            },
        );
    }

    Ok(())
}

// TODO reset token を取得して有効期限を確認するのを外に出す
// TODO password hash を外に出す
#[async_trait::async_trait]
impl<'store> ResetPasswordRepository for MemoryAuthUserRepository<'store> {
    async fn reset_token_entry(
        &self,
        reset_token: &ResetToken,
    ) -> Result<Option<ResetTokenEntry>, RepositoryError> {
        reset_token_entry(self, reset_token)
    }
    async fn reset_password<'a>(
        &self,
        reset_token: &'a ResetToken,
        hasher: impl AuthUserPasswordHasher + 'a,
        reset_at: AuthDateTime,
    ) -> Result<AuthUserId, ResetPasswordRepositoryError> {
        reset_password(self, reset_token, hasher, reset_at)
    }
}
fn reset_token_entry<'store>(
    repository: &MemoryAuthUserRepository<'store>,
    reset_token: &ResetToken,
) -> Result<Option<ResetTokenEntry>, RepositoryError> {
    let store = repository.store.lock().unwrap();

    Ok(store.get_reset_entry(&reset_token).map(|entry| {
        let entry = entry.clone();
        ResetTokenEntryExtract {
            login_id: entry.login_id,
            destination: entry.destination,
            expires: entry.expires,
            reset_at: entry.reset_at,
        }
        .restore()
    }))
}
fn reset_password<'store, 'a>(
    repository: &MemoryAuthUserRepository<'store>,
    reset_token: &'a ResetToken,
    hasher: impl AuthUserPasswordHasher + 'a,
    reset_at: AuthDateTime,
) -> Result<AuthUserId, ResetPasswordRepositoryError> {
    let target_entry: ResetEntry;

    {
        let store = repository.store.lock().unwrap();

        let entry = store.get_reset_entry(&reset_token).ok_or(
            ResetPasswordRepositoryError::RepositoryError(infra_error(
                "get reset entry error",
                "reset token not found",
            )),
        )?;

        target_entry = entry.clone().discard(reset_at);
    }

    {
        let hashed_password = hasher
            .hash_password()
            .map_err(ResetPasswordRepositoryError::PasswordHashError)?;

        let mut store = repository.store.lock().unwrap();

        // 実際のデータベースには registered_at も保存する必要がある
        store
            .insert_password(target_entry.user_id.clone(), hashed_password)
            .insert_reset_token(reset_token.clone(), target_entry.clone());
    }

    Ok(target_entry.user_id)
}
