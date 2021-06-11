use std::{collections::HashMap, sync::Mutex};

use super::{AuthUserPasswordRepository, HashedPassword, MatchPasswordError};

use crate::auth::{
    auth_user::_api::kernel::data::{AuthUser, AuthUserId},
    login_id::_api::data::LoginId,
    password::_api::authenticate::data::PasswordHashError,
};

pub type MemoryAuthUserPasswordStore = Mutex<MemoryAuthUserPasswordMap>;
pub struct MemoryAuthUserPasswordMap(HashMap<String, Entry>);

struct Entry(AuthUserId, HashedPassword);

impl MemoryAuthUserPasswordMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn with_password(login_id: LoginId, user: AuthUser, password: HashedPassword) -> Self {
        let mut store = HashMap::new();
        store.insert(
            login_id.extract(),
            Entry(AuthUserId::new(user.id_as_str().into()), password),
        );
        Self(store)
    }

    pub fn to_store(self) -> MemoryAuthUserPasswordStore {
        Mutex::new(self)
    }

    fn get(&self, user_id: &str) -> Option<&Entry> {
        self.0.get(user_id)
    }
}

pub struct MemoryAuthUserPasswordRepository<'a> {
    store: &'a MemoryAuthUserPasswordStore,
}

impl<'a> MemoryAuthUserPasswordRepository<'a> {
    pub const fn new(store: &'a MemoryAuthUserPasswordStore) -> Self {
        Self { store }
    }
}

impl<'a> AuthUserPasswordRepository for MemoryAuthUserPasswordRepository<'a> {
    fn match_password(
        &self,
        login_id: &LoginId,
        matcher: impl Fn(&HashedPassword) -> Result<bool, PasswordHashError>,
    ) -> Result<Option<AuthUserId>, MatchPasswordError> {
        let store = self.store.lock().unwrap();
        Ok(match store.get(login_id.as_str()) {
            None => None,
            Some(Entry(user_id, password)) => {
                if matcher(password).map_err(MatchPasswordError::PasswordHashError)? {
                    Some(user_id.clone())
                } else {
                    None
                }
            }
        })
    }
}
