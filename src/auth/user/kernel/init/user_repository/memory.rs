use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
};

use crate::{
    auth::user::password::reset::reset::infra::ResetTokenMoment,
    z_lib::repository::helper::infra_error,
};

use crate::auth::user::{
    account::{
        modify::infra::ModifyAuthUserAccountRepository,
        search::infra::SearchAuthUserAccountRepository, // TODO 整理
    },
    kernel::infra::AuthUserRepository,
    login_id::change::infra::{OverrideLoginIdRepository, OverrideUserEntry},
    password::{
        authenticate::infra::AuthenticatePasswordRepository,
        change::infra::{ChangePasswordRepository, OverridePasswordRepository},
        kernel::infra::HashedPassword,
        reset::{
            request_token::infra::RegisterResetTokenRepository,
            reset::infra::ResetPasswordRepository,
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
                search::data::{SearchAuthUserAccountBasket, SearchAuthUserAccountFilter},
            },
            kernel::data::{AuthUser, AuthUserExtract, AuthUserId, GrantedAuthRoles},
            login_id::kernel::data::LoginId,
            password::reset::kernel::data::{
                ResetToken, ResetTokenDestination, ResetTokenDestinationExtract,
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

struct SearchUserEntry {
    pub user_id: String,
    pub granted_roles: HashSet<String>,
    pub reset_token_destination: ResetTokenDestinationExtract,
}

#[derive(Clone)]
struct ResetEntry {
    user_id: AuthUserId,
    login_id: LoginId,
    destination: ResetTokenDestination,
    expires: ExpireDateTime,
    requested_at: AuthDateTime,
    reset_at: Option<AuthDateTime>,
}

impl ResetEntry {
    fn discard(self, reset_at: AuthDateTime) -> Self {
        Self {
            reset_at: Some(reset_at),
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
        requested_at: AuthDateTime,
        reset_at: Option<AuthDateTime>,
    ) -> Self {
        let mut store = Self::new();
        store
            .insert_granted_roles(user)
            .insert_login_id(login_id.clone(), user_id.clone())
            .insert_reset_token(
                reset_token,
                ResetEntry {
                    user_id,
                    login_id,
                    destination,
                    expires,
                    requested_at,
                    reset_at,
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
        requested_at: AuthDateTime,
        reset_at: Option<AuthDateTime>,
    ) -> Self {
        let mut store = Self::new();
        store
            .insert_login_id(login_id.clone(), user_id.clone())
            .insert_reset_token(
                reset_token,
                ResetEntry {
                    user_id,
                    login_id,
                    destination,
                    expires,
                    requested_at,
                    reset_at,
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
    fn insert_user(&mut self, login_id: LoginId, user: OverrideUserEntry) {
        let login_id = login_id.extract();
        self.login_id.insert(login_id.clone(), user.user_id);
        self.reset_token_destination
            .insert(login_id, user.reset_token_destination);
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
    fn update_password(
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
        // requested_at は参照されない(データとして放り込んでおくだけ)
        // warning が出るのを抑制するためにここで無駄に参照する
        let _ = &entry.requested_at;
        self.reset_token.insert(token.extract(), entry);
        self
    }
    fn update_reset_at(&mut self, token: ResetToken, reset_at: AuthDateTime) -> &mut Self {
        if let Some(entry) = self.reset_token.remove(token.as_str()) {
            self.reset_token
                .insert(token.extract(), entry.discard(reset_at));
        }
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
    ) -> Result<Option<ResetTokenDestination>, RepositoryError> {
        lookup_reset_token_user(self, login_id)
            .map(|user| user.map(|(_user_id, destination)| destination))
    }

    async fn change_destination(
        &self,
        login_id: &LoginId,
        data: ResetTokenDestination,
    ) -> Result<(), RepositoryError> {
        change_reset_token_destination(self, login_id, data)
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
    async fn lookup_user<'a>(
        &self,
        login_id: &'a LoginId,
    ) -> Result<Option<OverrideUserEntry>, RepositoryError> {
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
        user: OverrideUserEntry,
        new_login_id: LoginId,
    ) -> Result<(), RepositoryError> {
        override_login_id(self, user, new_login_id)
    }
}
fn lookup_user<'store, 'a>(
    repository: &MemoryAuthUserRepository<'store>,
    login_id: &'a LoginId,
) -> Result<Option<OverrideUserEntry>, RepositoryError> {
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

    Ok(user_id.map(|user_id| OverrideUserEntry {
        user_id,
        login_id: login_id.clone(),
        reset_token_destination: reset_token_destination.extract(),
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
    user: OverrideUserEntry,
    new_login_id: LoginId,
) -> Result<(), RepositoryError> {
    let mut store = repository.store.lock().unwrap();

    store.update_login_id(user.user_id.clone(), new_login_id.clone());
    store.remove_user(user.login_id.clone());
    store.insert_user(new_login_id, user);

    Ok(())
}

#[async_trait::async_trait]
impl<'store> AuthenticatePasswordRepository for MemoryAuthUserRepository<'store> {
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
    Ok(store.get_password(user_id).map(|password| password.clone()))
}

#[async_trait::async_trait]
impl<'store> ChangePasswordRepository for MemoryAuthUserRepository<'store> {
    async fn lookup_password<'a>(
        &self,
        user_id: &'a AuthUserId,
    ) -> Result<Option<HashedPassword>, RepositoryError> {
        lookup_password(self, user_id)
    }

    async fn change_password<'a>(
        &self,
        user_id: &'a AuthUserId,
        new_password: HashedPassword,
    ) -> Result<(), RepositoryError> {
        change_password(self, user_id, new_password)
    }
}
fn change_password<'store, 'a>(
    repository: &MemoryAuthUserRepository<'store>,
    user_id: &'a AuthUserId,
    new_password: HashedPassword,
) -> Result<(), RepositoryError> {
    let mut store = repository.store.lock().unwrap();
    store.insert_password(user_id.clone(), new_password);

    Ok(())
}

#[async_trait::async_trait]
impl<'store> OverridePasswordRepository for MemoryAuthUserRepository<'store> {
    async fn lookup_user_id<'a>(
        &self,
        login_id: &'a LoginId,
    ) -> Result<Option<AuthUserId>, RepositoryError> {
        lookup_user_id(self, login_id)
    }

    async fn override_password<'a>(
        &self,
        user_id: &'a AuthUserId,
        new_password: HashedPassword,
    ) -> Result<(), RepositoryError> {
        override_password(self, user_id, new_password)
    }
}
fn override_password<'store, 'a>(
    repository: &MemoryAuthUserRepository<'store>,
    user_id: &'a AuthUserId,
    new_password: HashedPassword,
) -> Result<(), RepositoryError> {
    let mut store = repository.store.lock().unwrap();
    store.insert_password(user_id.clone(), new_password);

    Ok(())
}

#[async_trait::async_trait]
impl<'store> RegisterResetTokenRepository for MemoryAuthUserRepository<'store> {
    async fn lookup_user(
        &self,
        login_id: &LoginId,
    ) -> Result<Option<(AuthUserId, ResetTokenDestination)>, RepositoryError> {
        lookup_reset_token_user(self, login_id)
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
        register_reset_token(
            self,
            reset_token,
            user_id,
            login_id,
            destination,
            expires,
            requested_at,
        )
    }
}
fn lookup_reset_token_user<'store>(
    repository: &MemoryAuthUserRepository<'store>,
    login_id: &LoginId,
) -> Result<Option<(AuthUserId, ResetTokenDestination)>, RepositoryError> {
    let store = repository.store.lock().unwrap();

    match (store.get_user_id(login_id), store.get_destination(login_id)) {
        (Some(user_id), Some(destination)) => Ok(Some((
            user_id.clone(),
            ResetTokenDestination::restore(destination.clone()),
        ))),
        _ => Ok(None),
    }
}
fn register_reset_token<'store>(
    repository: &MemoryAuthUserRepository<'store>,
    reset_token: ResetToken,
    user_id: AuthUserId,
    login_id: LoginId,
    destination: ResetTokenDestination,
    expires: ExpireDateTime,
    requested_at: AuthDateTime,
) -> Result<(), RepositoryError> {
    {
        let store = repository.store.lock().unwrap();

        // 有効期限が切れていても削除しない
        // もし衝突したら token generator の桁数を増やす
        if store.get_reset_entry(&reset_token).is_some() {
            return Err(infra_error("get reset entry error", "reset token conflict"));
        }
    }

    {
        let mut store = repository.store.lock().unwrap();

        store.insert_reset_token(
            reset_token.clone(),
            ResetEntry {
                user_id,
                login_id,
                destination,
                expires,
                requested_at,
                reset_at: None,
            },
        );
    }

    Ok(())
}

#[async_trait::async_trait]
impl<'store> ResetPasswordRepository for MemoryAuthUserRepository<'store> {
    async fn lookup_reset_token_entry(
        &self,
        reset_token: &ResetToken,
    ) -> Result<
        Option<(AuthUserId, LoginId, ResetTokenDestination, ResetTokenMoment)>,
        RepositoryError,
    > {
        lookup_reset_token_entry(self, reset_token)
    }

    async fn lookup_granted_roles<'a>(
        &self,
        user_id: &'a AuthUserId,
    ) -> Result<Option<GrantedAuthRoles>, RepositoryError> {
        lookup_granted_roles(self, user_id)
    }

    async fn reset_password(
        &self,
        reset_token: &ResetToken,
        user_id: &AuthUserId,
        new_password: HashedPassword,
        reset_at: AuthDateTime,
    ) -> Result<(), RepositoryError> {
        reset_password(self, reset_token, user_id, new_password, reset_at)
    }
}
fn lookup_reset_token_entry<'store>(
    repository: &MemoryAuthUserRepository<'store>,
    reset_token: &ResetToken,
) -> Result<Option<(AuthUserId, LoginId, ResetTokenDestination, ResetTokenMoment)>, RepositoryError>
{
    let store = repository.store.lock().unwrap();

    Ok(store.get_reset_entry(&reset_token).map(|entry| {
        (
            entry.user_id.clone(),
            entry.login_id.clone(),
            entry.destination.clone(),
            ResetTokenMoment::restore(entry.expires.clone(), entry.reset_at.clone()),
        )
    }))
}
fn reset_password<'store, 'a>(
    repository: &MemoryAuthUserRepository<'store>,
    reset_token: &ResetToken,
    user_id: &AuthUserId,
    new_password: HashedPassword,
    reset_at: AuthDateTime,
) -> Result<(), RepositoryError> {
    let mut store = repository.store.lock().unwrap();

    store
        .update_password(user_id.clone(), new_password)
        .update_reset_at(reset_token.clone(), reset_at);

    Ok(())
}
