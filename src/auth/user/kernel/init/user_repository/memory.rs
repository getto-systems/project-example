use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
};

use chrono::{DateTime, Utc};

use crate::z_lib::api::repository::helper::infra_error;

use crate::auth::user::{
    account::search::api::infra::{SearchAuthUserAccountFields, SearchAuthUserAccountRepository},
    kernel::infra::AuthUserRepository,
    password::{
        authenticate::api::infra::VerifyPasswordRepository,
        change::api::infra::ChangePasswordRepository,
        kernel::infra::{AuthUserPasswordHasher, AuthUserPasswordMatcher, HashedPassword},
        reset::{
            kernel::infra::{ResetTokenEntry, ResetTokenEntryExtract},
            request_token::api::infra::{
                RegisterResetTokenRepository, ResetTokenDestinationRepository,
            },
            reset::api::infra::ResetPasswordRepository,
        },
    },
};

use crate::{
    auth::{
        ticket::kernel::api::data::{AuthDateTime, ExpireDateTime},
        user::{
            account::search::api::data::{AuthUserAccountBasket, SearchAuthUserAccountBasket},
            kernel::data::{AuthUser, AuthUserExtract, AuthUserId, GrantedAuthRolesBasket},
            login_id::kernel::data::{LoginId, LoginIdBasket},
            password::{
                authenticate::api::data::VerifyPasswordRepositoryError,
                change::api::data::ChangePasswordRepositoryError,
                reset::{
                    kernel::data::{
                        ResetToken, ResetTokenDestination, ResetTokenDestinationExtract,
                    },
                    request_token::api::data::RegisterResetTokenRepositoryError,
                    reset::api::data::ResetPasswordRepositoryError,
                },
            },
        },
    },
    z_lib::api::{repository::data::RepositoryError, search::data::SearchPage},
};

pub type MemoryAuthUserStore = Mutex<MemoryAuthUserMap>;
pub struct MemoryAuthUserMap {
    login_id: HashMap<String, AuthUserId>, // login-id => user-id
    user_id: HashMap<String, LoginId>,     // login-id => user-id
    granted_roles: HashMap<String, HashSet<String>>, // user-id => granted-roles
    password: HashMap<String, HashedPassword>, // user-id => hashed-password
    reset_token_destination: HashMap<String, ResetTokenDestination>, // login-id => destination
    reset_token: HashMap<String, ResetEntry>, // reset-token => reset entry
}

struct UserEntry {
    pub user_id: String,
    pub granted_roles: HashSet<String>,
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
    ) -> Self {
        let mut store = Self::new();
        let user_id = user.clone().into_user_id();
        store
            .insert_granted_roles(user)
            .insert_login_id(login_id.clone(), user_id.clone())
            .insert_password(user_id, password);
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
            .insert_destination(login_id, destination);
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

    fn insert_login_id(&mut self, login_id: LoginId, user_id: AuthUserId) -> &mut Self {
        self.login_id
            .insert(login_id.clone().extract(), user_id.clone());
        self.user_id.insert(user_id.extract(), login_id);
        self
    }
    fn get_user_id(&self, login_id: &LoginId) -> Option<&AuthUserId> {
        self.login_id.get(login_id.as_str())
    }
    fn get_login_id(&self, user_id: &str) -> Option<&LoginId> {
        self.user_id.get(user_id)
    }

    fn insert_granted_roles(&mut self, user: AuthUser) -> &mut Self {
        let user = user.extract();
        self.granted_roles.insert(user.user_id, user.granted_roles);
        self
    }
    fn get_granted_roles(&self, user_id: &AuthUserId) -> Option<&HashSet<String>> {
        self.granted_roles.get(user_id.as_str())
    }
    fn all_users(&self) -> Vec<UserEntry> {
        self.granted_roles
            .iter()
            .map(|(user_id, granted_roles)| UserEntry {
                user_id: user_id.clone(),
                granted_roles: granted_roles.clone(),
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

    fn insert_destination(&mut self, login_id: LoginId, destination: ResetTokenDestination) {
        self.reset_token_destination
            .insert(login_id.extract(), destination);
    }
    fn get_destination(&self, login_id: &LoginId) -> Option<&ResetTokenDestination> {
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

#[async_trait::async_trait]
impl<'a> SearchAuthUserAccountRepository for MemoryAuthUserRepository<'a> {
    async fn search(
        &self,
        fields: &SearchAuthUserAccountFields,
    ) -> Result<SearchAuthUserAccountBasket, RepositoryError> {
        search(&self, fields)
    }
}
fn search<'a>(
    repository: &MemoryAuthUserRepository<'a>,
    _fields: &SearchAuthUserAccountFields,
) -> Result<SearchAuthUserAccountBasket, RepositoryError> {
    let store = repository.store.lock().unwrap();
    let users = store
        .all_users()
        .into_iter()
        // 実際のデータベースでは fields を使用して検索を行う
        .filter_map(|user| {
            store
                .get_login_id(&user.user_id)
                .map(|login_id| AuthUserAccountBasket {
                    login_id: LoginIdBasket::new(login_id.clone().extract()),
                    granted_roles: GrantedAuthRolesBasket::new(user.granted_roles),
                })
        })
        .collect();

    Ok(SearchAuthUserAccountBasket {
        page: SearchPage {
            offset: 0,
            limit: 0,
            all: 0,
        },
        users: users,
    })
}

#[async_trait::async_trait]
impl<'store> VerifyPasswordRepository for MemoryAuthUserRepository<'store> {
    async fn verify_password<'a>(
        &self,
        login_id: &'a LoginId,
        matcher: impl AuthUserPasswordMatcher + 'a,
    ) -> Result<AuthUserId, VerifyPasswordRepositoryError> {
        verify_password(self, login_id, matcher)
    }
}
fn verify_password<'store, 'a>(
    repository: &MemoryAuthUserRepository<'store>,
    login_id: &'a LoginId,
    matcher: impl AuthUserPasswordMatcher + 'a,
) -> Result<AuthUserId, VerifyPasswordRepositoryError> {
    let store = repository.store.lock().unwrap();

    let user_id = store
        .get_user_id(login_id)
        .ok_or(VerifyPasswordRepositoryError::PasswordNotFound)?;

    let password = store
        .get_password(user_id)
        .ok_or(VerifyPasswordRepositoryError::PasswordNotFound)?;

    let matched = matcher
        .match_password(password)
        .map_err(VerifyPasswordRepositoryError::PasswordHashError)?;

    if !matched {
        return Err(VerifyPasswordRepositoryError::PasswordNotMatched);
    }

    Ok(user_id.clone())
}

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

        // 実際のデータベースには registered_at も保存する必要がある
        store.insert_password(user_id.clone(), hashed_password);
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
        .map(|destination| destination.clone()))
}

#[async_trait::async_trait]
impl<'store> RegisterResetTokenRepository for MemoryAuthUserRepository<'store> {
    async fn register_reset_token(
        &self,
        login_id: LoginId,
        reset_token: ResetToken,
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
                infra_error("reset token conflict"),
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
            ResetPasswordRepositoryError::RepositoryError(infra_error("reset token not found")),
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
